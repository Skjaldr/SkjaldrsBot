use crate::discord;
use std::{io::ErrorKind, path::Path};
use std::process::Child;
use rfd::FileDialog;
use std::{env, path::PathBuf, fs, io, process::Command, borrow::Borrow};
use serde::{Serialize, Deserialize};

// declare and define constants related to this file
const CONFIG_FILE_NAME: &str = ".config.toml";
const _ENV_FILE_PATH: &str = ".env";

// simple function to prompt the user for input or instructions.
fn prompt_user(text: &str) {
    println!("{}", text);
}

struct Env {
    token: String,
}
impl Env {
    fn set_env_token(&self) {
        env::set_var("DISCORD_TOKEN", &self.token);
    }
}

fn introduction() {
    prompt_user("Welcome to Skjaldrs Bot!\n\n
    
    The first time you run this program, you will be asked to put in your discord token you recieved\n\
    from the Discord Developer Portal upon making your bot.  When prompted, please enter your key so\n\
    it can be stored by the .config file that is generated.\n\
    \n\
    Then, the windows explorer program will open and you will be asked to navigate to your Shadowbane\n\
    folder and pick the sb.exe file (Your desktop shortcut is also a valid and recommended option).\n\
    The path to the selected file with also be saved to your .config file and called on upon each\n\
    startup, eliminating the need to start a separate Shadowbane instance.\n\
    \n\
    Once you've completed the initial setup, Skjaldrs Bot will then rename the title of the Shadowbane\n\
    window that was opened with the program.  This is why it's convenient let this program start the\n\
    instance on which your in-game bot will run.\n\
    \n\
    If you have any questions or concerns about using this program, please feel free to ask.  Also\n\
    I ask that you report any bugs or errors directly to me so that I may get them fixed as soon as\n\
    possible!  Thank you for using Skjaldrs Bot, I hope you will find the experience beneficial. =D\n\
    Press enter to continue......");

    let mut wait = String::new();
    io::stdin().read_line(&mut wait).expect("Failed to press enter???? WHY DID YOU FAIL!?");


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
    fn serial(&self) -> String {
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
        match fs::write(CONFIG_FILE_NAME, self.serial()) {
            Ok(_) => fs::write(CONFIG_FILE_NAME, self.serial()).unwrap(),
            Err(e) => println!("Error writing to file {}, {}", CONFIG_FILE_NAME, e),
        }
    }

    // Takes in the path to the sb.exe executable and runs the program
    fn run_sb(&mut self, path: &PathBuf) {
        //Command::new(&path).spawn().unwrap();
        if let Err(e) = Command::new(&path).arg("?").spawn() {
            println!("{:?}",e);
            println!("File path has been lost or deleted, please navigate to and select sb.exe (or shortcut)");
            self.repair_path_err();
        } 
    }

    // if the file path becomes corrupted, prompt the user and get a new path, serialize, and write it to the config
    fn repair_path_err(&mut self) {
        self.token = self.deser().token;        //obtain the token from the config to save it's state
        self.path = self.prompt_get_path();     
        self.serial();             
        self.write_config();

    }
}

pub fn run_program() {
    // create an instance of Config struct
    let mut config: Config = Config { token: String::new(), path: PathBuf::new() };

        
        if config.does_exist() { // true if the config file exists

            let conf = config.deser();  // create a new config struct with the deserialized data
            config.run_sb(&conf.path);          // call the run function to 
            
            let nenv = Env {
                token: conf.token,
            };
            nenv.set_env_token();
            
            
         
        } else { // false if the config file does not exist
            
            introduction();
            //creates a new config file, serializes the content of the Config struct
            // and writes the contents of the struct to the config
            config.prompt_get_fields();         // obtain information to populate fields
            config.create_new_config();         // create the config file
            config.serial();                    // serialize the config struct
            config.write_config();              // tie it all together and write to config file!
            println!("Skjaldr's Bot is starting sb.exe");
            config.run_sb(&config.path.to_path_buf());
            let nenv = Env {
                token: config.token.to_string(),
            };
            nenv.set_env_token();
            config.run_sb(&config.path.to_path_buf());
            //println!("{}", nenv.token)
    
        }
}