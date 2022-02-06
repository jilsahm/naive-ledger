use std::collections::HashMap;

use self::{account::Account, r#type::Type, error::Error};
pub use transaction::Transaction;

mod account;
mod error;
mod r#type;
mod transaction;

#[derive(Default)]
pub struct Ledger {
    accounts: HashMap<u32, Account>,
    history: HashMap<u32, Transaction>,
    rollbacks: HashMap<u32, ()>,
}

impl Ledger {

    pub fn update(&mut self, transaction: Transaction) -> Result<(), Error> {
        if self.is_new_transaction(&transaction) {
            match transaction.r#type {
                Type::Deposit => self.deposit(transaction),
                Type::Withdrawal => self.withdrawal(transaction),
                Type::Dispute => todo!(),
                Type::Resolve => todo!(),
                Type::Chargeback => todo!(),
            }
        } else {
            Err(Error::DuplicatedTransactionError(transaction.tx))
        }
    }

    fn is_new_transaction(&self, transaction: &Transaction) -> bool {
        !self.history.contains_key(&transaction.tx)
    }

    fn deposit(&mut self, transaction: Transaction) -> Result<(), Error> {
        let account = self.accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));
        account.available += transaction.amount;
        account.total += transaction.amount;
        self.history.insert(transaction.tx, transaction);
        Ok(())
    }

    fn withdrawal(&mut self, transaction: Transaction) -> Result<(), Error> {
        let account = self.accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));
        if account.available >= transaction.amount { 
            account.available -= transaction.amount;
            account.total -= transaction.amount;
            self.history.insert(transaction.tx, transaction);
            Ok(())
        } else {
            Err(Error::InsufficientFundsError(transaction.tx, account.client))
        }
    }

    pub fn print(&self) {
        let mut writer = Account::writer(std::io::stdout());
        self.accounts
            .iter()
            .map(|(_, v,)| v)
            .for_each(|account| { let _ = writer.serialize(account); }); 
    }
}

#[cfg(test)]
mod tests {
    use super::{Ledger, transaction::Transaction, r#type::Type, account::Account};

    #[test]
    fn deposit() {
        let mut ledger = Ledger::default();
        let transaction = Transaction {
            r#type: Type::Deposit,
            client: 100,
            tx: 1,
            amount: 50.0,
        };
        let expected = Account {
            client: 100,
            available: 50.0,
            held: 0.0,
            total: 50.0,
            locked: false,
        };
        let result = ledger.update(transaction);
        assert!(result.is_ok());
        assert_eq!(1, ledger.history.drain().next().expect("one history entry").0);
        assert_eq!(expected, ledger.accounts.drain().next().expect("one account entry").1);
    }

    #[test]
    fn withdrawal_sufficient() {
        let mut ledger = Ledger::default();
        let transaction1 = Transaction {
            r#type: Type::Deposit,
            client: 100,
            tx: 1,
            amount: 50.0,
        };
        let transaction2 = Transaction {
            r#type: Type::Withdrawal,
            client: 100,
            tx: 2,
            amount: 20.0,
        };
        let _ = ledger.update(transaction1);
        let result = ledger.update(transaction2);
        assert!(result.is_ok());
        assert_eq!(30.0, ledger.accounts.drain().next().expect("one account entry").1.available);
    }

    #[test]
    fn withdrawal_insufficient() {
        let mut ledger = Ledger::default();
        let transaction = Transaction {
            r#type: Type::Withdrawal,
            client: 100,
            tx: 1,
            amount: 50.0,
        };
        let result = ledger.update(transaction);
        assert!(result.is_err());
    }
}