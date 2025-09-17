use anyhow::Result;
use num_enum::TryFromPrimitive;
use rand::random;
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

#[derive(Debug, Clone)]
struct DNSHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

impl DNSHeader {
    fn new(flags: u16, num_questions: u16) -> Self {
        DNSHeader {
            id: random(),
            flags,
            num_questions,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        [
            self.id.to_be_bytes(),
            self.flags.to_be_bytes(),
            self.num_questions.to_be_bytes(),
            self.num_answers.to_be_bytes(),
            self.num_authorities.to_be_bytes(),
            self.num_additionals.to_be_bytes(),
        ]
        .concat()
    }

    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            id: u16::from_be_bytes(bytes[0..2].try_into()?),
            flags: u16::from_be_bytes(bytes[2..4].try_into()?),
            num_questions: u16::from_be_bytes(bytes[4..6].try_into()?),
            num_answers: u16::from_be_bytes(bytes[6..8].try_into()?),
            num_authorities: u16::from_be_bytes(bytes[8..10].try_into()?),
            num_additionals: u16::from_be_bytes(bytes[10..12].try_into()?),
        })
    }
}

#[derive(Debug, Clone, Default, TryFromPrimitive, PartialEq)]
#[repr(u16)]
enum RecordType {
    #[default]
    A = 1,
    Ns = 2,
    Md = 3,
    Mf = 4,
    Cname = 5,
    Aaaa = 28,
}

#[derive(Debug, Clone, Default, TryFromPrimitive, PartialEq)]
#[repr(u16)]
enum Class {
    #[default]
    In = 1,
}

#[derive(Debug, Clone)]
struct DNSQuestion {
    name: String,
    type_: RecordType,
    class: Class,
}

impl DNSQuestion {
    fn new(name: String, type_: RecordType, class: Class) -> Self {
        Self { name, type_, class }
    }
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.name.as_bytes(),
            &(self.type_.clone() as u16).to_be_bytes(),
            &(self.class.clone() as u16).to_be_bytes(),
        ]
        .concat()
    }

    fn parse(buf: &[u8], cursor_start: usize) -> Result<(Self, usize)> {
        let mut cursor = cursor_start;
        let (name, length) = decode_name(buf, cursor);
        cursor += length;
        Ok((
            Self {
                name,
                type_: RecordType::try_from(u16::from_be_bytes(
                    buf[cursor..cursor + 2].try_into()?,
                ))
                .unwrap(),
                class: Class::try_from(u16::from_be_bytes(buf[cursor + 2..cursor + 4].try_into()?))
                    .unwrap(),
            },
            cursor + 4 - cursor_start,
        ))
    }
}

fn decode_name(buf: &[u8], cursor_start: usize) -> (String, usize) {
    let mut cursor = cursor_start;
    let mut length = buf[cursor] as usize;
    let mut components = Vec::new();
    while length != 0 {
        if length & 0b11000000 != 0 {
            // DNS component max length is 63 bytes so in case first 2 bits are set,
            // then takes the bottom 6 bits of the length byte, plus the next byte,
            // and converts that to a pointer.
            components.push(decode_compressed_name(buf, cursor));
            cursor += 2;
            return (components.join("."), cursor - cursor_start);
        } else {
            let start = cursor + 1;
            cursor += length + 1;
            components.push(String::from_utf8_lossy(&buf[start..cursor]).into_owned());
            length = buf[cursor] as usize;
        }
    }
    // Added one for the zero at the end
    cursor += 1;
    (components.join("."), cursor - cursor_start)
}

fn decode_compressed_name(buf: &[u8], cursor_start: usize) -> String {
    let cursor =
        u16::from_be_bytes([(buf[cursor_start] & 0b00111111), buf[cursor_start + 1]]) as usize;
    decode_name(buf, cursor).0
}

#[allow(unused)]
#[derive(Debug, Clone)]
enum DNSRecordData {
    Data(Vec<u8>),
    Name(String),
    Ipv4Addr(Ipv4Addr),
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct DNSRecord {
    name: String,
    type_: RecordType,
    class: Class,
    ttl: u32,
    data: DNSRecordData,
}

impl DNSRecord {
    fn parse(buf: &[u8], start_cursor: usize) -> Result<(Self, usize)> {
        let mut cursor = start_cursor;
        let (name, length) = decode_name(buf, cursor);
        cursor += length;
        let type_ =
            RecordType::try_from(u16::from_be_bytes(buf[cursor..cursor + 2].try_into()?)).unwrap();
        let class =
            Class::try_from(u16::from_be_bytes(buf[cursor + 2..cursor + 4].try_into()?)).unwrap();
        let ttl = u32::from_be_bytes(buf[cursor + 4..cursor + 8].try_into()?);
        let data_len = u16::from_be_bytes(buf[cursor + 8..cursor + 10].try_into()?) as usize;
        cursor += 10;
        let data = match type_ {
            RecordType::A => {
                let ip = Ipv4Addr::new(
                    buf[cursor],
                    buf[cursor + 1],
                    buf[cursor + 2],
                    buf[cursor + 3],
                );
                cursor += 4;
                DNSRecordData::Ipv4Addr(ip)
            }
            RecordType::Ns | RecordType::Cname => {
                let (name, len) = decode_name(buf, cursor);
                cursor += len;
                DNSRecordData::Name(name)
            }
            _ => {
                let data = buf[cursor..cursor + data_len].to_vec();
                cursor += data_len;
                DNSRecordData::Data(data)
            }
        };
        Ok((
            Self {
                name,
                type_,
                class,
                ttl,
                data,
            },
            cursor - start_cursor,
        ))
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct DNSPacket {
    header: DNSHeader,
    questions: Vec<DNSQuestion>,
    answers: Vec<DNSRecord>,
    authorities: Vec<DNSRecord>,
    additionals: Vec<DNSRecord>,
}

impl DNSPacket {
    pub fn parse(buf: &[u8]) -> Result<Self> {
        let header = DNSHeader::parse(buf)?;
        const DNS_HEADER_LEN: usize = 12;
        let mut cursor = DNS_HEADER_LEN;
        let mut questions = Vec::new();
        for _ in 0..header.num_questions {
            let (question, length) = DNSQuestion::parse(buf, cursor)?;
            questions.push(question);
            cursor += length;
        }

        let mut answers = Vec::new();
        for _ in 0..header.num_answers {
            let (answer, length) = DNSRecord::parse(buf, cursor)?;
            answers.push(answer);
            cursor += length;
        }

        let mut authorities = Vec::new();
        for _ in 0..header.num_authorities {
            let (authority, length) = DNSRecord::parse(buf, cursor)?;
            authorities.push(authority);
            cursor += length;
        }

        let mut additionals = Vec::new();
        for _ in 0..header.num_additionals {
            let (additional, length) = DNSRecord::parse(buf, cursor)?;
            additionals.push(additional);
            cursor += length;
        }
        Ok(Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        })
    }

    fn get_answer(&self) -> Option<Ipv4Addr> {
        for answer in &self.answers {
            if let DNSRecordData::Ipv4Addr(name) = answer.data {
                return Some(name);
            }
        }
        None
    }

    fn get_nameserver_ip(&self) -> Option<Ipv4Addr> {
        for record in &self.additionals {
            if let DNSRecordData::Ipv4Addr(ip) = record.data {
                return Some(ip);
            }
        }
        None
    }

    pub fn get_nameserver(&self) -> Option<&str> {
        for record in &self.authorities {
            if let DNSRecordData::Name(name) = &record.data {
                return Some(name.as_str());
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct DNSResolver {
    id_addr: Ipv4Addr,
}

impl DNSResolver {
    pub fn new(id_addr: &str) -> Self {
        DNSResolver {
            id_addr: id_addr.parse::<Ipv4Addr>().unwrap(),
        }
    }

    fn encode_dns_name(name: &str) -> Vec<u8> {
        let mut encoded = Vec::new();
        for component in name.split('.') {
            encoded.push(component.len() as u8);
            encoded.extend(component.as_bytes());
        }
        encoded.push(0);
        encoded
    }

    fn build_query(domain_name: &str, record_type: RecordType, class: Class) -> Vec<u8> {
        let encoded_name = Self::encode_dns_name(domain_name);
        let header = DNSHeader::new(1 << 8, 1).to_bytes();
        let questions =
            DNSQuestion::new(String::from_utf8(encoded_name).unwrap(), record_type, class)
                .to_bytes();
        [header, questions].concat()
    }

    fn lookup(domain_name: &str, ip_addr: &Ipv4Addr) -> Result<DNSPacket> {
        println!("Querying {ip_addr} for {domain_name}");
        let query = Self::build_query(domain_name, RecordType::A, Class::In);
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        socket.set_read_timeout(Some(Duration::from_secs(3)))?;
        socket.set_write_timeout(Some(Duration::from_secs(3)))?;
        socket.send_to(&query, (*ip_addr, 53))?;

        let mut buf = [0; 1024];
        let (size, _src) = socket.recv_from(&mut buf)?;
        DNSPacket::parse(&buf[..size])
    }

    pub fn resolve(&self, domain_name: &str) -> Result<Ipv4Addr> {
        let mut ip_addr = self.id_addr;
        loop {
            let dns_packet = Self::lookup(domain_name, &ip_addr)?;
            if let Some(ip) = dns_packet.get_answer() {
                return Ok(ip);
            } else if let Some(ns_ip) = dns_packet.get_nameserver_ip() {
                ip_addr = ns_ip;
            } else if let Some(name) = dns_packet.get_nameserver() {
                ip_addr = self.resolve(name)?;
            } else {
                anyhow::bail!("Could not resolve DNS packet");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Class, DNSResolver, RecordType, decode_name};

    #[test]
    fn test_encode_dns_name() {
        assert_eq!(
            DNSResolver::encode_dns_name("google.com"),
            b"\x06google\x03com\x00"
        );
    }

    #[test]
    fn test_class() {
        assert_eq!(Class::In as u16, 1);
        assert_eq!(Class::try_from(1).unwrap(), Class::In);
    }

    #[test]
    fn test_record_type() {
        assert_eq!(RecordType::A as u16, 1);
        assert_eq!(RecordType::Ns as u16, 2);
        assert_eq!(RecordType::Md as u16, 3);
        assert_eq!(RecordType::Mf as u16, 4);

        assert_eq!(RecordType::try_from(1).unwrap(), RecordType::A);
        assert_eq!(RecordType::try_from(2).unwrap(), RecordType::Ns);
        assert_eq!(RecordType::try_from(3).unwrap(), RecordType::Md);
        assert_eq!(RecordType::try_from(4).unwrap(), RecordType::Mf);
    }

    #[test]
    fn test_build_query() {
        // validate after the random id
        assert_eq!(
            &DNSResolver::build_query("example.com", RecordType::A, Class::In)[2..],
            b"\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07example\x03com\x00\x00\x01\x00\x01"
        );
    }

    #[test]
    fn test_decode_name() {
        let mut buf = [0; 17];
        buf[0] = 3;
        buf[1] = 'w' as u8;
        buf[2] = 'w' as u8;
        buf[3] = 'w' as u8;
        buf[4] = 7;
        buf[5] = 'e' as u8;
        buf[6] = 'x' as u8;
        buf[7] = 'a' as u8;
        buf[8] = 'm' as u8;
        buf[9] = 'p' as u8;
        buf[10] = 'l' as u8;
        buf[11] = 'e' as u8;
        buf[12] = 3;
        buf[13] = 'c' as u8;
        buf[14] = 'o' as u8;
        buf[15] = 'm' as u8;
        buf[16] = 0;

        let (name, usize) = decode_name(&buf, 0);
        assert_eq!(name, "www.example.com");
        assert_eq!(usize as usize, 17);
    }
}
