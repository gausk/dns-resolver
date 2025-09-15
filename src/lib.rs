use num_enum::TryFromPrimitive;
use rand::random;

#[derive(Debug, Clone)]
pub struct DNSHeader {
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
}

#[derive(Debug, Clone, Default, TryFromPrimitive, PartialEq)]
#[repr(u16)]
pub enum RecordType {
    #[default]
    A = 1,
    Ns = 2,
    Md = 3,
    Mf = 4,
}

#[derive(Debug, Clone, Default, TryFromPrimitive, PartialEq)]
#[repr(u16)]
pub enum Class {
    #[default]
    In = 1,
}

#[derive(Debug, Clone)]
pub struct DNSQuestion {
    name: Vec<u8>,
    type_: RecordType,
    class: Class,
}

impl DNSQuestion {
    pub fn new(name: Vec<u8>, type_: RecordType, class: Class) -> Self {
        DNSQuestion { name, type_, class }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [
            self.name.as_slice(),
            &(self.type_.clone() as u16).to_be_bytes(),
            &(self.class.clone() as u16).to_be_bytes(),
        ]
        .concat()
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

/*
def build_query(domain_name, record_type):
    name = encode_dns_name(domain_name)
    id = random.randint(0, 65535)
    RECURSION_DESIRED = 1 << 8
    header = DNSHeader(id=id, num_questions=1, flags=RECURSION_DESIRED)
    question = DNSQuestion(name=name, type_=record_type, class_=CLASS_IN)
    return header_to_bytes(header) + question_to_bytes(question)
 */

pub fn build_query(domain_name: &str, record_type: RecordType, class: Class) -> Vec<u8> {
    let encoded_name = encode_dns_name(domain_name);
    let header = DNSHeader::new(1 << 8, 1).to_bytes();
    let questions = DNSQuestion::new(encoded_name, record_type, class).to_bytes();
    [header, questions].concat()
}

#[cfg(test)]
mod tests {
    use crate::{Class, RecordType, build_query, encode_dns_name};

    #[test]
    fn test_encode_dns_name() {
        assert_eq!(encode_dns_name("google.com"), b"\x06google\x03com\x00");
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
            &build_query("example.com", RecordType::A, Class::In)[2..],
            b"\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x07example\x03com\x00\x00\x01\x00\x01"
        );
    }
}
