use super::*;

mod balance;
mod create;

const COOKIE_FILE_PATH: &str = "/Users/raphael/lib/Bitcoin/signet/.cookie";
const EXTERNAL_DESCRIPTOR: &str = "wpkh([20648cc1/84h/1h/0h]tpubDCLwqmrwAw1ie9rU3R92RrYBYxBDdFHW7Cta8ZbGfkfNAsFiQZKEhMKDfC7bFpMiqtR21n1hhZ3YjPPaj8x5P8Lac6gXK1eoa1uncM8AmEc/0/*)#3vw5s4wr";
const INTERNAL_DESCRIPTOR: &str = "wpkh([20648cc1/84h/1h/0h]tpubDCLwqmrwAw1ie9rU3R92RrYBYxBDdFHW7Cta8ZbGfkfNAsFiQZKEhMKDfC7bFpMiqtR21n1hhZ3YjPPaj8x5P8Lac6gXK1eoa1uncM8AmEc/1/*)";

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[command(about = "Create a wallet")]
  Create(create::Create),
  #[command(about = "Show wallet balance")]
  Balance,
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let file_path = "test_wallet.sqlite3";
    let mut conn = Connection::open(file_path).unwrap();

    let wallet_opt = Wallet::load()
      .check_network(Network::Signet)
      .load_wallet(&mut conn)
      .unwrap();

    let mut wallet = match wallet_opt {
      Some(wallet) => {
        log::info!("Loaded existing wallet database.");
        wallet
      }
      None => {
        log::info!("Creating new wallet database.");
        Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
          .network(Network::Signet)
          .create_wallet(&mut conn)
          .unwrap()
      }
    };

    let rpc_client: Client = Client::new(
      "http://127.0.0.1:38332",
      Auth::CookieFile(COOKIE_FILE_PATH.into()),
    )
    .unwrap();

    let blockchain_info = rpc_client.get_blockchain_info().unwrap();
    log::info!("Connected to Bitcoin Core RPC.");
    log::info!("Chain: {}", blockchain_info.chain);
    log::info!(
      "Latest block: {} at height {}",
      blockchain_info.best_block_hash,
      blockchain_info.blocks,
    );

    let wallet_tip: CheckPoint = wallet.latest_checkpoint();
    log::info!(
      "Current wallet tip is: {} at height {}",
      &wallet_tip.hash(),
      &wallet_tip.height()
    );

    let mut emitter = Emitter::new(&rpc_client, wallet_tip.clone(), wallet_tip.height());

    log::info!("Syncing blocks...");
    while let Some(block) = emitter.next_block().unwrap() {
      wallet
        .apply_block_connected_to(&block.block, block.block_height(), block.connected_to())
        .unwrap();
    }

    log::info!("Syncing mempool...");
    let mempool_emissions: Vec<(Transaction, u64)> = emitter.mempool().unwrap();
    wallet.apply_unconfirmed_txs(mempool_emissions);

    wallet.persist(&mut conn).unwrap();

    //    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    //    println!(
    //      "Generated address {} at index {}",
    //      address.address, address.index
    //    );

    match self {
      Self::Create(create) => create.run(options),
      Self::Balance => balance::run(wallet),
    }
  }
}

pub trait Output: Send {
  fn print(&self);
}

impl<T> Output for T
where
  T: Serialize + Send,
{
  fn print(&self) {
    serde_json::to_writer_pretty(std::io::stdout(), self).ok();
    println!();
  }
}

pub(crate) type SubcommandResult = Result<Option<Box<dyn Output>>>;
