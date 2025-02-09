use super::*;

pub(crate) fn run(options: Options) -> SubcommandResult {
  let mut connection = Connection::open(options.wallet_dir()?)?;
  let mut wallet = wallet::open(&options, &mut connection)?;

  let address = wallet.reveal_next_address(KeychainKind::External).address;

  wallet.persist(&mut connection)?;

  Ok(Some(Box::new(address)))
}
