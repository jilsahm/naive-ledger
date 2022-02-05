use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}