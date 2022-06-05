use rfd::FileDialog;
use std::{env, path::PathBuf, fs, io::{self, Read}, process::Command};
use serde::{Serialize, Deserialize};


// declare and define constants related to this file
const CONFIG_FILE_NAME: &str = ".config.toml";
const _ENV_FILE_PATH: &str = ".env";
// const SB_FILE_PATH:  =  FileDialog::new();

// simple function to prompt the user for input or instructions.
fn prompt_user(text: &str) {
    println!("{}", text);

}

// create a new struct that takes in discord token String, and sb.exe file path PathBuf
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    token: String,
    path: PathBuf,
}

// implements the config file.  Handles the get fields fns, also the create, write, file contents, 
// serial/deserial, config exist check, and run path from config 
impl Config {
    fn get_token(&mut self) -> String {
        let mut token = String::new();
        match io::stdin().read_line(&mut token) {
            Ok(_) => {
                token.to_string()
            },
            Err(e) => {
                println!("Failed to read in line from user, Creating empty key. ERROR: {}", e);
                return String::new();
            }
        }
    }

    // gets the path of the file from the user;  Opens the file explorer on windows to help user pick file
    
    fn prompt_get_path(&mut self) -> PathBuf {
        let path = FileDialog::new().pick_file();
        match path { 
            Some(v) => {
                v
            }
            None => {
                println!("You failed to select a file, generating empty path.\n\
                (Program will not start sb.exe and must be started manually)");
                return PathBuf::new();
            }
        }
    }

    // takes in get_token() and get_path() to help organize the main function more neatly
    fn prompt_get_fields(&mut self) {
        prompt_user("Please enter your Discord Token:");
        //println!("from get_fields: {}",self.get_token());  //but only get number outputs when I set self.token = self.get_token()?
        self.token = self.get_token();

        prompt_user("Navigate to your Shadowbane folder and select the sb.exe file:");
        self.path = self.prompt_get_path(); // why do I need to set self.path = self.prompt_get_path() here?


    }

    // create a new config file
    fn create_new_config(&self) {
        match fs::File::create(CONFIG_FILE_NAME) {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to create new config file, {e}.  Generating new file");
                fs::File::create(CONFIG_FILE_NAME).unwrap()
            }
        };
    }

    // serialize the config struct
    fn serialize(&self) -> String {
       toml::to_string(self).unwrap()
    }

    // takes in the contents from the config file, returns the string created, 
    fn from_config_file(&self) -> String {
        let contents: String = fs::read_to_string(CONFIG_FILE_NAME).unwrap();
        contents
    }

    // deserializes the config file into a struct, calls the from_config_file()
    fn deser(&self) -> Config {
        toml::from_str(&self.from_config_file()).unwrap()
    }

    //checks if the config file exists
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

    //writes all of the serialized config contents into the .config.toml file
    fn write_config(&self) {
        match fs::write(CONFIG_FILE_NAME, self.serialize()) {
            Ok(_) => fs::write(CONFIG_FILE_NAME, self.serialize()).unwrap(),
            Err(e) => println!("Error writing to file {}, {}", CONFIG_FILE_NAME, e),
        }
    }

    // Takes in the path to the sb.exe executable and runs the program
    fn run_sb(&self, path: &PathBuf) {
        Command::new(path).spawn().unwrap();
    }
}


fn main() {
    // create an instance of Config struct
    let mut config: Config = Config { token: String::new(), path: PathBuf::new() };

        // true if the config file exists
        if config.does_exist() { 


            let conf = config.deser();  // create a new config struct with the deserialized data
            config.run_sb(&conf.path);          // call the run function to 

            
        // false if the config file does not exist 
        } else { 
            
            //creates a new config file, serializes the content of the Config struct
            // and writes the contents of the struct to the config
            config.prompt_get_fields();        // obtain information to populate fields
            config.create_new_config(); // create the config file
            config.serialize();         // serialize the config struct
            config.write_config();      // tie it all together and write to config file!
            
        }
}
