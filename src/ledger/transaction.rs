use csv::{ReaderBuilder, Trim, DeserializeRecordsIntoIter};
use serde::Deserialize;

use super::r#type::Type;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Transaction {
    pub r#type: Type,
    pub client: u32,
    pub tx: u32,
    pub amount: f64,
}

impl Transaction {

    pub fn reader<R>(inner: R) -> DeserializeRecordsIntoIter<R, Self> 
    where  
        R: std::io::Read
    {
        ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .trim(Trim::All)
            .flexible(true)
            .from_reader(inner)
            .into_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::ledger::{transaction::Transaction, r#type::Type};


    #[test]
    fn deserialze() {
        let expected = Transaction { r#type: Type::Deposit, client: 1, tx: 2, amount: 5.50 };
        let sample = "type, client, tx, amount\ndeposit,    1,   2, 5.50\nwithdrawal,1,1";
        let buffer = BufReader::new(sample.as_bytes());
        let mut reader = Transaction::reader(buffer);
        let entry = reader.next();
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert!(entry.is_ok(), "{:?}", entry.err());
        assert_eq!(expected, entry.unwrap());
        let entry = reader.next();
        assert!(entry.is_some());
    }
}