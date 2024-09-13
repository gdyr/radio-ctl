use clap::{Parser, Subcommand, ValueEnum};
use windows::Devices::Radios::RadioKind;

#[derive(Parser, Debug)]
#[command(
  version,
  about = "A command-line tool to manage radios such as WiFi, Bluetooth, etc.",
  long_about = None,
  after_help = "EXAMPLES:\n\
                radio-ctl list\n\
                radio-ctl on --kind=bluetooth\n\
                radio-ctl off --name=\"Builtin Wi-Fi\""
)]
pub struct Args {

    #[command(subcommand)]
    pub subcommand: Option<Command>,

    #[arg(long, help = "Specify the radio type", value_enum)]
    pub kind: Option<Kind>,

    #[arg(long, help = "Specify the device name")]
    pub name: Option<String>,

}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {

    /// Turn on the matching radio device(s)
    On,

    /// Turn off the matching radio device(s)
    Off,

    /// List all radio devices
    List,

}

impl Args {
    pub fn command(&self) -> Command {
        self.subcommand.clone().unwrap_or(Command::List)
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Kind {
    Bluetooth,
    WiFi,
    FM,
    MobileBroadband,
    Other,
}

impl Into<RadioKind> for Kind {
    fn into(self) -> RadioKind {
        match self {
            Kind::Bluetooth => RadioKind::Bluetooth,
            Kind::WiFi => RadioKind::WiFi,
            Kind::FM => RadioKind::FM,
            Kind::MobileBroadband => RadioKind::MobileBroadband,
            Kind::Other => RadioKind::Other,
        }
    }
}