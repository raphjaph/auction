use super::*;

mod balance;
mod create;
mod receive;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[command(about = "Create a wallet")]
  Create(create::Create),
  #[command(about = "Show wallet balance")]
  Balance,
  #[command(about = "Get new receive address")]
  Receive,
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    match self {
      Self::Create(create) => create.run(options),
      Self::Balance => balance::run(options),
      Self::Receive => receive::run(options),
    }
  }
}

pub trait Output: Send {
  fn print(&self);
}

impl<T> Output for T
where
  T: Serialize + Send,
{
  fn print(&self) {
    serde_json::to_writer_pretty(std::io::stdout(), self).ok();
    println!();
  }
}

pub(crate) type SubcommandResult = Result<Option<Box<dyn Output>>>;
