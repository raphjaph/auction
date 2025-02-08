use super::*;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[command(about = "Run the auction")]
  Run,
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
      Self::Run => {
        const COOKIE_FILE_PATH: &str = "/Users/raphael/lib/Bitcoin/signet/.cookie";
        const EXTERNAL_DESCRIPTOR: &str = "wpkh([20648cc1/84h/1h/0h]tpubDCLwqmrwAw1ie9rU3R92RrYBYxBDdFHW7Cta8ZbGfkfNAsFiQZKEhMKDfC7bFpMiqtR21n1hhZ3YjPPaj8x5P8Lac6gXK1eoa1uncM8AmEc/0/*)#3vw5s4wr";
        const INTERNAL_DESCRIPTOR: &str = "wpkh([20648cc1/84h/1h/0h]tpubDCLwqmrwAw1ie9rU3R92RrYBYxBDdFHW7Cta8ZbGfkfNAsFiQZKEhMKDfC7bFpMiqtR21n1hhZ3YjPPaj8x5P8Lac6gXK1eoa1uncM8AmEc/1/*)";
        // const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
        // const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

        let file_path = "test_wallet.sqlite3";
        let mut conn = rusqlite::Connection::open(file_path).unwrap();

        let wallet_opt = Wallet::load()
          .check_network(Network::Signet)
          .load_wallet(&mut conn)
          .unwrap();

        let mut wallet = match wallet_opt {
          Some(wallet) => {
            println!("Loaded existing wallet database.");
            wallet
          }
          None => {
            println!("Creating new wallet database.");
            Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
              .network(Network::Signet)
              .create_wallet(&mut conn)
              .unwrap()
          }
        };

        let balance: Balance = wallet.balance();
        println!("Wallet balance before syncing: {}", balance.total());

        let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
        println!(
          "Generated address {} at index {}",
          address.address, address.index
        );

        let rpc_client: Client = Client::new(
          "http://127.0.0.1:38332",
          Auth::CookieFile(COOKIE_FILE_PATH.into()),
        )
        .unwrap();

        let blockchain_info = rpc_client.get_blockchain_info().unwrap();
        println!(
          "\nConnected to Bitcoin Core RPC.\nChain: {}\nLatest block: {} at height {}\n",
          blockchain_info.chain, blockchain_info.best_block_hash, blockchain_info.blocks,
        );

        let wallet_tip: CheckPoint = wallet.latest_checkpoint();
        println!(
          "Current wallet tip is: {} at height {}",
          &wallet_tip.hash(),
          &wallet_tip.height()
        );

        let mut emitter = Emitter::new(&rpc_client, wallet_tip.clone(), wallet_tip.height());

        println!("Syncing blocks...");
        while let Some(block) = emitter.next_block().unwrap() {
          print!("{} ", block.block_height());
          wallet
            .apply_block_connected_to(&block.block, block.block_height(), block.connected_to())
            .unwrap();
        }
        println!();

        println!("Syncing mempool...");
        let mempool_emissions: Vec<(Transaction, u64)> = emitter.mempool().unwrap();
        wallet.apply_unconfirmed_txs(mempool_emissions);

        wallet.persist(&mut conn).unwrap();

        let balance: Balance = wallet.balance();
        println!("Wallet balance after syncing: {}", balance.total());

        Ok(())
      }
    }
  }
}
