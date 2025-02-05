use {
  anyhow::Error,
  arguments::Arguments,
  bdk_bitcoind_rpc::{
    bitcoincore_rpc::{Auth, Client, RpcApi},
    Emitter,
  },
  bdk_wallet::{
    bitcoin::{Network, Transaction},
    chain::local_chain::CheckPoint,
    AddressInfo, Balance, KeychainKind, Wallet,
  },
  clap::Parser,
  serde::{Deserialize, Serialize},
  std::{env, process},
};

mod arguments;
mod subcommand;

type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn main() {
  env_logger::init();

  let args = Arguments::parse();

  match args.run() {
    Err(err) => {
      eprintln!("error: {err}");
      if env::var_os("RUST_BACKTRACE")
        .map(|val| val == "1")
        .unwrap_or_default()
      {
        eprintln!("{}", err.backtrace());
      }

      process::exit(1);
    }

    Ok(_) => {
      process::exit(0);
    }
  }
}
