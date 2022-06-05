use rfd::FileDialog;
use std::{env, path::PathBuf, fs, io, process::Command};
use serde::{Serialize, Deserialize};

const CONFIG_FILE_NAME: &str = ".config.toml";
const _ENV_FILE_PATH: &str = ".env";
// const SB_FILE_PATH:  =  FileDialog::new();


fn prompt_user(text: &str) {
    println!("{}", text);

}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    token: String,
    path: PathBuf,
}

impl Config {
    // fn new() -> Self {
    //     let contents = fs::read_to_string(CONFIG_FILE_NAME).unwrap();
    //     toml::from_str(&contents).unwrap()
    // }

    // function to get the fields for Discord Token and Path
    fn get_fields(&mut self) {
        prompt_user("Please enter your Discord Token:");
        io::stdin().read_line(&mut self.token).expect("Failed to get input from user").to_string();
        prompt_user("Navigate to your Shadowbane folder and select the sb.exe file:");
        self.path = FileDialog::new().pick_file().expect("Failed to select file");
    }
    // create a new config file
    fn create_new_config(&self) {
        match fs::File::create(CONFIG_FILE_NAME) {
            Ok(_) => fs::File::create(CONFIG_FILE_NAME).unwrap(),
            Err(e) => {
                println!("Faile to create new config file, {e}.  Generating new file");
                fs::File::create(CONFIG_FILE_NAME).unwrap()
            }
        };
    }
    // serialize the config struct
    fn serialize(&self) -> String {
       toml::to_string(self).unwrap()
    }

    fn from_config_file(&self) -> String {
        let contents: String = fs::read_to_string(CONFIG_FILE_NAME).unwrap();
        contents
    }

    fn deser(&self) -> Config {
        toml::from_str(&self.from_config_file()).unwrap()
    }

    fn does_exist(&self) -> bool {
        match fs::read_to_string(CONFIG_FILE_NAME) {
            Ok(_) => {
                return true
            } 
            Err(e) => {
                println!("Error: {e}");
                return false
            }
        };
    }

    fn write_config(&self) {
        match fs::write(CONFIG_FILE_NAME, self.serialize()) {
            Ok(_) => fs::write(CONFIG_FILE_NAME, self.serialize()).unwrap(),
            Err(e) => println!("Error writing to file {}, {}", CONFIG_FILE_NAME, e),
        }
    }

    fn run_sb(&self, path: &PathBuf) {
        Command::new(path).spawn().unwrap();
    }
}


fn main() {

    let mut config: Config = Config { token: String::new(), path: PathBuf::new() };

        //let mut check = false;
        if config.does_exist() {
            //returns true
            println!("is true");
            println!("I will now start the Shadowbane file you chose.");
            
            let conf = config.deser();
            config.run_sb(&conf.path);
            
        } else {
            // returns false, creates a new config file, serializes the content of the Config struct
            // and writes the contents of the struct to the config
            config.get_fields();
            config.create_new_config();
            config.serialize();
            config.write_config();
            println!("Config should now exist and have content!");
        }
}
