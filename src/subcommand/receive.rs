use super::*;

pub(crate) fn run(wallet: &mut PersistedWallet<Connection>) -> SubcommandResult {
  Ok(Some(Box::new(
    wallet.reveal_next_address(KeychainKind::External).address,
  )))
}
