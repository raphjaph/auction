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

f: front end client that runs in the browser, and also has wallet
o: ord node with api
a: agent

architecture 1
--------------

1. client connects to agent
2. client sends psbt to agent
3. agent requests data from ord
   - are inputs valid?
   - maybe: what assets are in inputs? 
4. agent takes action depending on psbt and response from ord
   agent is trusting ord here, definitely about which assets are
   in which inputs. optionally trusting ord unless agent operates
   in spv mode
   - psbt is invalid (inputs aren't valid)
     return error response
   - psbt is no good for other reason (inputs too small for fixed price,
     or under reserve), nonstandard
   - psbt is good, in which case either broadcast it immediately, or
     depending on policy, hold it and broadcast it later
5. if it's taking action later, wake up at some time later on, and
   sign and broadcast transaction

architecture 2
--------------

1. client connects to ord
2. client sends psbt to ord
3. ord rejects PSBT if outright invalid:
   - bad inputs, whatever
4. annotates psbt with assets and values
5. sends to agent
6. agent acts on policy as above, accepts or rejects
7. agent rejects
8. ord responds to client with rejection
