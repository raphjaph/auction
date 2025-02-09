use super::*;

pub fn run(options: Options) -> SubcommandResult {
  let mut connection = Connection::open(options.wallet_dir()?)?;
  let mut wallet = wallet::open(&options, &mut connection)?;

  log::info!(
    "Connecting to Bitcoin Core RPC at {}",
    options.bitcoin_rpc_url()
  );

  let rpc_client: Client = options.bitcoin_rpc_client()?;

  let blockchain_info = rpc_client.get_blockchain_info()?;

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

  while let Some(block) = emitter.next_block()? {
    wallet.apply_block_connected_to(&block.block, block.block_height(), block.connected_to())?
  }

  log::info!("Syncing mempool...");

  let mempool_emissions: Vec<(Transaction, u64)> = emitter.mempool()?;
  wallet.apply_unconfirmed_txs(mempool_emissions);

  wallet.persist(&mut connection)?;

  Ok(Some(Box::new(wallet.balance())))
}
