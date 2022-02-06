use std::fmt;

pub enum Error {
    DuplicatedTransactionError(u32),
    InsufficientFundsError(u32, u32),
}

impl fmt::Display for Error {

    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::DuplicatedTransactionError(tx) => format!("skipped transaction {} because it was already processed", tx),
                Self::InsufficientFundsError(tx, client) => format!("skipped transaction {} because client {} has insufficient funds", tx, client),
            }
        )
    }
}