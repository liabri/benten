use structopt::StructOpt;
use structopt::clap::AppSettings;

pub fn main() {
    match Arguments::from_args().command {
        Command::Start => {},
        Command::Kill => {},
        Command::Set{name} => {
            let file_path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_layout");
            std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            std::fs::write(file_path, name).unwrap();
        },
        
        Command::Reload =>{
            let file_path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_layout");
            let current_layout = std::fs::read_to_string(&file_path).unwrap();
            std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            std::fs::write(file_path, current_layout).unwrap();
        },

        Command::Current => {
            let file_path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_layout");
            println!("{}", std::fs::read_to_string(&file_path).unwrap());
        },

        Command::List => {
            let base_dir = xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home(); 

            println!("\nlayouts");
            let layouts_dir = base_dir.join("layouts"); 
            let paths = std::fs::read_dir(layouts_dir).unwrap();
            for path in paths {
                println!("  {}", path.unwrap().file_name().to_str().unwrap().split('.').next().unwrap())           
            }
        },
    };
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
    ///Kill benten
    Kill,

    #[structopt(no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Set specific layout
    Set { name: String },

    #[structopt(alias = "r", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Reload current layout
    Reload,    

    #[structopt(alias = "l", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///List all available layouts
    List,

    #[structopt(alias = "c", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Current layout
    Current,      
}