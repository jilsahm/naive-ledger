use std::fmt;

#[derive(Debug)]
pub enum Error {
    DuplicatedTransactionError(u32),
    InsufficientFundsError(u32, u16),
    MissingTransactionError(u32),
    DisputeAlreadyInProgressError(u32),
    DisputeAlreadyFinishedError(u32),
}

impl fmt::Display for Error {

    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::DuplicatedTransactionError(tx) => format!("skipped transaction {} because it was already processed", tx),
                Self::InsufficientFundsError(tx, client) => format!("skipped transaction {} because client {} has insufficient funds", tx, client),
                Self::MissingTransactionError(tx) => format!("rollback not possible because transaction {} is missing", tx),
                Self::DisputeAlreadyInProgressError(tx) => format!("skipping transaction because there is already a dispute for {} in progress", tx),
                Self::DisputeAlreadyFinishedError(tx) => format!("skipping transaction because the dispute for {} was already finished", tx),
            }
        )
    }
}