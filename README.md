Auction Design
==============

- lightweight, no dependency on `ord` or `bitcoind` when running
- should use SPV and compact block filters to load/verify wallet state
  - [BDK Kyoto](https://github.com/bitcoindevkit/bdk-kyoto)
- database considerations:
  - BDK uses sqlite3 or postgres
  - should the auction database be the same or just use redb
  - We could also implement the `Persistence` trait for redb as a backend for
    the wallet
- only hold descriptors and then sign according to some policy
- smart clients, they have to do all the heavy lifting about checking what is
  contained in the UTXO, construct PSBTs and send final transaction into the
  network
- Policies:
  - start with fixed price for every UTXO
