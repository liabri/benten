use structopt::StructOpt;
use structopt::clap::AppSettings;

pub fn main() {
    match Arguments::from_args().command {
        Command::Start => {},
        Command::Ping => {},
        Command::Kill => {},
        Command::Set{name} => {
            let file_path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_mode");
            std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            std::fs::write(file_path, name);

        },
        Command::Reload =>{
            let file_path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_mode");
            let current_mode = std::fs::read_to_string(&file_path).unwrap();
            std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            std::fs::write(file_path, current_mode);
        },

        Command::Current => {
            let file_path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_mode");
            println!("{}", std::fs::read_to_string(&file_path).unwrap());
        },

        Command::List => {},
    };
}

fn write_current_mode(mode: &str) {
    let file_path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_mode");
}

#[derive(StructOpt)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
    #[structopt(short="n", long="no-daemonise")]
    ///Do not daemonize benten
    pub daemonise: bool,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(alias = "s", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Start benten
    Start,

    #[structopt(alias = "k", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Ping benten to check if its reachable
    Ping,

    #[structopt(alias = "k", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Kill benten
    Kill,

    #[structopt(alias = "set", no_version, global_settings = &[AppSettings::DisableVersion])]
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