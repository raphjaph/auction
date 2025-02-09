use super::*;

#[derive(Debug, Parser, Clone)]
pub struct Create {
  #[clap(long, help = "Wallet <DESCRIPTOR>.")]
  pub descriptor: Option<String>,
}

impl Create {
  pub fn run(&self, options: Options) -> SubcommandResult {
    wallet::create(&options)?;

    Ok(None)
  }
}
