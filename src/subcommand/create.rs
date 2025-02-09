use super::*;

#[derive(Debug, Parser, Clone)]
pub struct Create {
  #[clap(long, help = "Wallet <DESCRIPTOR>.")]
  pub descriptor: String,
}

impl Create {
  pub fn run(&self, _options: Options) -> SubcommandResult {
    todo!()
  }
}
