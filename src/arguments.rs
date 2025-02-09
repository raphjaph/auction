use {
  super::*,
  clap::builder::styling::{AnsiColor, Effects, Styles},
  subcommand::{Subcommand, SubcommandResult},
};

#[derive(Debug, Parser)]
#[command(
  version,
  styles = Styles::styled()
    .error(AnsiColor::Red.on_default() | Effects::BOLD)
    .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
    .invalid(AnsiColor::Red.on_default())
    .literal(AnsiColor::Blue.on_default())
    .placeholder(AnsiColor::Cyan.on_default())
    .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
    .valid(AnsiColor::Green.on_default()),
)]
pub(crate) struct Arguments {
  #[command(flatten)]
  pub(crate) options: Options,
  #[command(subcommand)]
  pub(crate) subcommand: Subcommand,
}

impl Arguments {
  pub(crate) fn run(self) -> SubcommandResult {
    self.subcommand.run(self.options)
  }
}
