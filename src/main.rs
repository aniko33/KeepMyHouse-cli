use std::{
    fs,
    io::{Read, Write},
};

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod crypto;
mod interactive;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Actions,
}

#[derive(Subcommand)]
enum Actions {
    /// Create new database
    Init(Init),
    /// Open a database
    Open(Opendb),
    /// List of elements
    List(ListCmd),
    /// Transfer your database file
    Transfer(Tnf),
}

#[derive(Args)]
struct Init {
    filename: String,
}

#[derive(Args)]
struct Opendb {
    filename: String,
    #[arg(short)]
    encryption: String,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct ListCmd {
    #[arg(short)]
    encryption: bool,
}

#[derive(Args)]
struct Tnf {
    #[arg(short)]
    protocol: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonDatabseKMH {
    title: String,
    username: String,
    password: String,
    notes: String,
}

#[derive(Debug)]
enum Encryption {
    AES256GCM,
    SALSA20,
    CHACHA20,
}

#[derive(Clone)]
struct DBManage {
    db: Vec<JsonDatabseKMH>,
}

impl DBManage {
    fn show(&self) {
        let mut creds: Vec<Vec<String>> = Vec::new();

        for (i, e) in self.db.iter().enumerate() {
            let i_string = i.to_string();
            let tc = Vec::from([
                i_string,
                e.title.to_string(),
                e.username.to_string(),
                e.password.to_string(),
                e.notes.to_string(),
            ]);

            creds.push(tc);
        }

        interactive::table(
            vec!["ID", "Title", "Username", "Password", "Notes"],
            creds,
            '-',
            5,
        );
    }
}

fn main() {
    let cli = Cli::parse();
    let all_encryptions = vec!["aes256", "salsa20", "chacha20"];

    match &cli.command {
        Actions::Init(init) => {
            init_db(&init.filename);
        }

        Actions::Open(open) => {
            open_db(&open.filename, &open.encryption);
        }

        Actions::List(list) => {
            if list.encryption {
                interactive::tree_classic("Encryption list", all_encryptions);
            }
        }
        Actions::Transfer(transfer) => {}
    }

    return;
}

fn init_db(filename: &String) {
    let ans = match interactive::select(
        vec!["AES256 GCM", "Salsa20", "Chacha20-Poly1305"],
        "Which cryptography do you want to use?",
    ) {
        Some(r) => r,
        None => {
            return;
        }
    };

    let password = match interactive::ask_password("Add a password:", true) {
        Some(r) => r,
        None => {
            return;
        }
    };

    match ans.as_str() {
        "aes256 gcm" => {
            let out = crypto::encrypt_database_aes(&vec![], &password).unwrap();
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
        "salsa20" => {
            let out = crypto::encrypt_database_salsa20(&vec![], &password);
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
        "chacha20-poly1305" => (),
        _ => (),
    }
}

fn open_db(filename: &String, encryption: &String) {
    let mut fbuffer = Vec::new();

    let encryption_type = match encryption.to_lowercase().as_str() {
        "aes256" => Encryption::AES256GCM,
        "salsa20" => Encryption::SALSA20,
        "chacha20" => Encryption::CHACHA20,
        _ => {
            println!("This encryption don't exist :(");
            return;
        }
    };

    // Check if exist
    if fs::metadata(filename).is_err() {
        println!("File not found");
        return;
    }

    // Open file
    let mut hfile = match fs::File::open(filename) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Read file
    match hfile.read_to_end(&mut fbuffer) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Get password
    let password = match interactive::ask_password("password:", false) {
        Some(r) => r,
        None => {
            return;
        }
    };

    let decrypted_db = match encryption_type {
        Encryption::AES256GCM => match crypto::decrypt_database_aes(fbuffer, &password) {
            Ok(r) => Some(r),
            Err(_) => None,
        },
        Encryption::SALSA20 => Some(crypto::decrypt_database_salsa20(fbuffer, &password)),
        // Encryption::CHA
        _ => {
            return;
        }
    };

    if decrypted_db.is_none() {
        println!("Password invalid");
        return;
    }

    // Deserialize DB
    let json_db: Vec<JsonDatabseKMH> = serde_json::from_str(
        String::from_utf8(decrypted_db.unwrap())
            .expect("Bytes to String failed")
            .as_str(),
    )
    .expect("Invalid JSON format");

    // Init DB
    let mut dbmanage = DBManage { db: json_db };

    loop {
        // Database interaction
        dbmanage.show();

        let ans =
            match interactive::select(vec!["Add", "Remove", "Modify"], "What do you want to do?") {
                Some(r) => r,
                None => return,
            };

        match ans.to_lowercase().as_str() {
            "add" => {
                let title = interactive::ask("Title:").unwrap();
                let username = interactive::ask("Username:").unwrap();
                let password_asked = interactive::ask("Password:").unwrap();
                let notes = interactive::ask("Notes:").unwrap();

                dbmanage.db.push(JsonDatabseKMH {
                    title,
                    username,
                    password: password_asked,
                    notes,
                });

                let edb = match encryption_type {
                    Encryption::AES256GCM => {
                        crypto::encrypt_database_aes(&dbmanage.db, &password).unwrap()
                    }
                    Encryption::SALSA20 => {
                        crypto::encrypt_database_salsa20(&dbmanage.db, &password)
                    }
                    Encryption::CHACHA20 => {
                        crypto::encrypt_database_chacha20(&dbmanage.db, &password)
                    }
                };

                fs::OpenOptions::new()
                    .write(true)
                    .open(filename)
                    .unwrap()
                    .write_all(edb.as_slice())
                    .unwrap();
            }
            "remove" => {
                let id = interactive::ask("ID:").unwrap().parse::<usize>().unwrap();
                dbmanage.db.remove(id);
            }
            "modify" => {}
            _ => return,
        }
    }
}


//Guess i'll die 