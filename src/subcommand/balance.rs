use super::*;

pub fn run(wallet: PersistedWallet<Connection>) -> SubcommandResult {
  Ok(Some(Box::new(wallet.balance())))
}
