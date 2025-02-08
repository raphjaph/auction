use super::*;

mod balance;
mod create;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[command(about = "Create a wallet")]
  Create(create::Create),
  #[command(about = "Show wallet balance")]
  Balance,
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
      Self::Create(create) => create.run(options),
      Self::Balance => balance::run(options),
    }
  }
}
