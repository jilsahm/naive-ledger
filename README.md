# naive-ledger
A simple inmemory ledger for processing a given set of transactions.

## Usage
```
$> cargo run -- source.csv > target.csv
```

## Design Flaws
* the `.flexible(true)` configuration is not working as expected, resulting in disputes, resolves, and chargebacks have to have an amount required even if unused
* the dispute chain can only triggered once per transaction
* OOM in case an infinite transaction source is used
* simple dirty rollback mechanism
* not heavily integration tested
* not all corner cases tested