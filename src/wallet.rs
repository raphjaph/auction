use super::*;

const EXTERNAL_DESCRIPTOR: &str = "wpkh([20648cc1/84h/1h/0h]tpubDCLwqmrwAw1ie9rU3R92RrYBYxBDdFHW7Cta8ZbGfkfNAsFiQZKEhMKDfC7bFpMiqtR21n1hhZ3YjPPaj8x5P8Lac6gXK1eoa1uncM8AmEc/0/*)#3vw5s4wr";
const INTERNAL_DESCRIPTOR: &str = "wpkh([20648cc1/84h/1h/0h]tpubDCLwqmrwAw1ie9rU3R92RrYBYxBDdFHW7Cta8ZbGfkfNAsFiQZKEhMKDfC7bFpMiqtR21n1hhZ3YjPPaj8x5P8Lac6gXK1eoa1uncM8AmEc/1/*)";

pub(crate) fn create(options: &Options) -> Result {
  log::info!("Creating new wallet database.");

  let wallet_dir = options.wallet_dir()?;

  if let Err(err) = std::fs::create_dir_all(wallet_dir.parent().unwrap()) {
    bail!(
      "failed to create data dir `{}`: {err}",
      wallet_dir.parent().unwrap().display()
    );
  }

  let mut connection = Connection::open(wallet_dir)?;

  let mut wallet = Wallet::create(EXTERNAL_DESCRIPTOR, INTERNAL_DESCRIPTOR)
    .network(options.network())
    .create_wallet(&mut connection)?;

  wallet.persist(&mut connection)?;

  Ok(())
}

pub(crate) fn open(
  options: &Options,
  conn: &mut Connection,
) -> Result<PersistedWallet<Connection>> {
  log::info!("Loaded existing wallet database.");

  Ok(
    match Wallet::load()
      .check_network(options.network())
      .load_wallet(conn)?
    {
      Some(wallet) => wallet,
      None => bail!("no wallet found, create one first"),
    },
  )
}
