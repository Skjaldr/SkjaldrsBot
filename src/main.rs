use rfd::FileDialog;
use std::{env, path::{Path, PathBuf}, fs::{self, OpenOptions}, io::{self, Read}};
use serde::{Serialize, Deserialize};

const CONFIG_FILE_NAME: &str = ".config.toml";
const ENV_FILE_PATH: &str = ".env";
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
       let test = toml::to_string(self).unwrap();
       println!();
       println!("1: {}", &test);
       test
    }
    // deserialize the config struct
    fn deser(&self) {
        let test: Config = match toml::from_str(CONFIG_FILE_NAME) {
            Ok(()) => toml::from_str(CONFIG_FILE_NAME).unwrap(),
            Err(e) => {

                println!("Failed to find config file.  New config generation needed");
                self.create_new_config();
                let mut conf = Config { 
                token: String::from("Need to init"), 
                path: Path::new(CONFIG_FILE_NAME).to_path_buf() 
                };
                conf.get_fields();
                conf
            }
        };
        println!();
        dbg!(test);
    }

    fn does_exist(&self) -> bool {
        match fs::read_to_string(CONFIG_FILE_NAME) {
            Ok(_) => {
                fs::read_to_string(CONFIG_FILE_NAME).unwrap();
                return true
            }
            Err(e) => {
                println!("{e} \".config.toml\".  The config does not exist or has been deleted.");
                return false 
            }
        };
    }

}


fn main() {

    let mut config: Config = Config { token: String::new(), path: PathBuf::new() };
    config.get_fields();
    // config.serialize();
    // config.deser();
    //config.create_new_config();
    // let path = Path::new(CONFIG_FILE_NAME);
    // let file = match fs::read_to_string(CONFIG_FILE_NAME) {
    //     Ok(_) => fs::read_to_string(CONFIG_FILE_NAME).unwrap(),
    //     Err(e) => {
    //         println!("{e}  Creating new config file.");
    //         return config.create_new_config() 
    //     }
    // };

        // if config.does_exist() {
        //     //println!("File is empty");
        //     println!("File exists");
        // } else {
        //     //println!("File is not empty!");
        //     println!("File does not exist!");
        // }

        while config.does_exist() == false {
            if false {
                    //println!("File is empty");
                    println!("File exists");
                } else {
                    //println!("File is not empty!");
                    config.create_new_config();
                    println!("Generating new .config.toml");
                }
        }
    


    //println!("{}", &config.token);
    //println!("{}", &config.path.to_string_lossy());



}
