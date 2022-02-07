use std::collections::{HashMap, HashSet};

use self::{account::Account, r#type::Type, error::Error};
pub use transaction::Transaction;

mod account;
mod error;
mod transaction;
mod r#type;

/// A naive ledger implementation for processing a list of incoming transactions.
#[derive(Default)]
pub struct Ledger {
    accounts: HashMap<u16, Account>,
    history: HashMap<u32, Transaction>,
    ongoing_rollbacks: HashMap<u32, (u16, Type)>,
    finished_rollbacks: HashSet<u32>,
}

impl Ledger {

    /// Apllies the given transaction onto the ledger.
    /// 
    /// # Arguments
    /// 
    /// * `transaction` - the transaction to be applied to the ledger
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the update was successfull otherwise returns the error.
    pub fn update(&mut self, transaction: Transaction) -> Result<(), Error> {
        if self.is_new_transaction(&transaction) {
            self.register_account(transaction.client);
            match transaction.r#type {
                Type::Deposit => self.deposit(transaction),
                Type::Withdrawal => self.withdrawal(transaction),
                Type::Dispute => self.dispute(transaction),
                Type::Resolve => self.resolve(transaction),
                Type::Chargeback => self.chargeback(transaction),
            }
        } else {
            Err(Error::DuplicatedTransactionError(transaction.tx))
        }
    }

    /// Check if a given transaction was already processed.
    /// 
    /// # Arguments
    /// 
    /// * `transaction` - transaction to be evaluated
    /// 
    /// # Returns
    /// 
    /// Returns *true* in case the transaction was already processed, otherwise
    /// returns *false*. Keep in mind the diputes, resolves, and chargebacks to not
    /// apear in the history.
    fn is_new_transaction(&self, transaction: &Transaction) -> bool {
        !self.history.contains_key(&transaction.tx)
    }

    /// Registeres a new account if not already present.
    /// 
    /// # Arguments
    /// 
    /// * `client` - the client ID to be used during the registration
    fn register_account(&mut self, client: u16) {
        self.accounts
            .entry(client)
            .or_insert_with(|| Account::new(client));
    }

    fn deposit(&mut self, transaction: Transaction) -> Result<(), Error> {
        let account = self.accounts.get_mut(&transaction.client).expect("registered account");
        account.available += transaction.amount;
        account.total += transaction.amount;
        self.history.insert(transaction.tx, transaction);
        Ok(())
    }

    fn withdrawal(&mut self, transaction: Transaction) -> Result<(), Error> {
        let account = self.accounts.get_mut(&transaction.client).expect("registered account");
        if account.available >= transaction.amount { 
            account.available -= transaction.amount;
            account.total -= transaction.amount;
            self.history.insert(transaction.tx, transaction);
            Ok(())
        } else {
            Err(Error::InsufficientFundsError(transaction.tx, account.client))
        }
    }

    fn dispute(&mut self, transaction: Transaction) -> Result<(), Error> {
        if self.ongoing_rollbacks.contains_key(&transaction.tx) {
            return Err(Error::DisputeAlreadyInProgressError(transaction.tx));
        }
        if self.finished_rollbacks.contains(&transaction.tx) {
            return Err(Error::DisputeAlreadyFinishedError(transaction.tx));
        }
        if self.is_new_transaction(&transaction) {
            return Err(Error::MissingTransactionError(transaction.tx));
        }
        let target = self.history.get(&transaction.tx).unwrap();
        let account = self.accounts.get_mut(&transaction.client).expect("registered account");
        match target.r#type {
            Type::Deposit => {
                account.available -= target.amount;
                account.held += target.amount;
            }
            Type::Withdrawal => {
                account.held += target.amount;
                account.total += target.amount;
            }
            _ => panic!("only deposits and withdrawals can be rollbacked")
        }
        self.ongoing_rollbacks.insert(transaction.tx, (account.client, target.r#type));
        Ok(())
    }

    fn resolve(&mut self, transaction: Transaction) -> Result<(), Error> {
        match self.ongoing_rollbacks.get(&transaction.tx) {
            None => return Err(Error::MissingTransactionError(transaction.tx)),
            Some((client, r#type)) => {
                let account = self.accounts.get_mut(client).expect("registered account");
                let target = self.history.get(&transaction.tx).expect("source transaction");
                match r#type {
                    Type::Deposit => {
                        account.available += target.amount;
                        account.held -= target.amount;
                    }
                    Type::Withdrawal => {
                        account.total -= target.amount;
                        account.held -= target.amount;
                    }
                    _ => panic!("only deposits and withdrawals can be rollbacked"),
                }
            }
        }
        self.finished_rollbacks.insert(transaction.tx);
        self.ongoing_rollbacks.remove(&transaction.tx);
        Ok(())
    }

    fn chargeback(&mut self, transaction: Transaction) -> Result<(), Error> {
        match self.ongoing_rollbacks.get(&transaction.tx) {
            None => return Err(Error::MissingTransactionError(transaction.tx)),
            Some((client, r#type)) => {
                let account = self.accounts.get_mut(client).expect("registered account");
                let target = self.history.get(&transaction.tx).expect("source transaction");
                match r#type {
                    Type::Deposit => {
                        account.held -= target.amount;
                        account.total -= target.amount;                   
                        account.locked = true;
                    }
                    Type::Withdrawal => {
                        account.available += target.amount;
                        account.total += target.amount;
                        account.held -= target.amount;                
                        account.locked = true;
                    }
                    _ => panic!("only deposits and withdrawals can be rollbacked"),
                }
            }
        }
        self.finished_rollbacks.insert(transaction.tx);
        self.ongoing_rollbacks.remove(&transaction.tx);
        Ok(())
    }

    /// Printes the current state of the ledger into the systems `stdout`
    /// formatted as CSV.
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