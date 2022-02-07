use csv::{Writer, WriterBuilder};
use serde::{Serialize};

#[derive(Debug, Serialize, PartialEq)]
pub struct Account {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl Account {

    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    /// Creates a CSV writer with proper configuration.
    /// 
    /// # Arguments
    /// 
    /// * `inner` - the destination of the serialization
    /// 
    /// # Returns
    /// 
    /// Returns the configured CSV writer.
    pub fn writer<W>(inner: W) -> Writer<W> 
    where
        W: std::io::Write
    {
        WriterBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .from_writer(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::Account;

    #[test]
    fn serialize() {
        let mut buffer = vec![];
        {
            let mut writer = Account::writer(&mut buffer);
            let result = writer.serialize(Account::new(1));
            assert!(result.is_ok());
        }
        let result = String::from_utf8_lossy(&buffer);
        let expected = "client,available,held,total,locked\n1,0.0,0.0,0.0,false\n";
        assert_eq!(expected, result);
    }
}