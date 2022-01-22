use structopt::StructOpt;
use structopt::clap::AppSettings;

pub fn main() {
    match Arguments::from_args().command {
        Command::Set{name} => {},
        Command::Reload => {},
        Command::List => {},
        Command::Current => {},
    };
}

#[derive(StructOpt)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(alias = "s", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Set specific mode
    Set { name: String },

    #[structopt(alias = "r", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Reload current mode
    Reload,    

    #[structopt(alias = "l", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///List all available modes
    List,

    #[structopt(alias = "c", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Current mode
    Current,      
}