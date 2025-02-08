use super::*;

#[derive(Clone, Debug, Parser)]
pub struct Options {
  #[arg(long, help = "Load Bitcoin Core data dir from <BITCOIN_DATA_DIR>.")]
  pub(crate) bitcoin_data_dir: Option<PathBuf>,
  #[arg(long, help = "Connect to Bitcoin Core RPC at <RPC_URL>.")]
  pub(crate) bitcoin_rpc_port: Option<u16>,
  #[clap(long, help = "Store acme cache in <DATA_DIR>.")]
  pub(crate) data_dir: Option<PathBuf>,
  #[clap(long, help = "Run on <CHAIN>.")]
  pub(crate) chain: Option<Chain>,
  #[clap(
    long,
    default_value = "2016",
    help = "Auction <CYCLE_LENGTH> in blocks."
  )]
  #[clap(long, help = "Wallet <DESCRIPTOR> to use for bidding address.")]
  pub(crate) descriptor: Option<String>,
}

impl Options {
  pub(crate) fn data_dir(&self) -> PathBuf {
    self.data_dir.clone().unwrap_or_default()
  }

  pub(crate) fn network(&self) -> Network {
    self.chain.unwrap_or(Chain::Mainnet).network()
  }

  pub(crate) fn chain(&self) -> Chain {
    self.chain.unwrap_or(Chain::Mainnet)
  }

  pub(crate) fn bitcoin_rpc_auth(&self) -> Result<Auth> {
    let path = self.bitcoin_data_dir.clone().unwrap_or(self.data_dir());
    let cookie_file = match self.network() {
      Network::Bitcoin => path.join(".cookie"),
      _ => path.join(self.network().to_string()).join(".cookie"),
    };

    Ok(Auth::CookieFile(cookie_file))
  }

  pub(crate) fn bitcoin_rpc_port(&self) -> u16 {
    self
      .bitcoin_rpc_port
      .unwrap_or_else(|| self.chain().default_rpc_port())
  }

  pub(crate) fn bitcoin_rpc_url(&self, wallet_name: Option<String>) -> String {
    let base_url = format!("127.0.0.1:{}", self.bitcoin_rpc_port());

    match wallet_name {
      Some(wallet_name) => format!("{base_url}/wallet/{wallet_name}"),
      None => format!("{base_url}/"),
    }
  }

  pub(crate) fn bitcoin_rpc_client(&self, wallet: Option<String>) -> Result<Client> {
    let rpc_url = self.bitcoin_rpc_url(wallet.clone());

    let auth = self.bitcoin_rpc_auth()?;

    if let Auth::CookieFile(cookie_file) = &auth {
      ensure!(
        cookie_file.is_file(),
        "cookie file `{}` does not exist",
        cookie_file.display()
      );
    }

    Client::new(&rpc_url, auth)
      .with_context(|| format!("failed to connect to Bitcoin Core RPC at `{rpc_url}`"))
  }
}
