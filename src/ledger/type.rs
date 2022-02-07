use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}