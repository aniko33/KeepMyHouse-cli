use std::{
    fs,
    io::{Read, Write}, process,
};

use arboard::Clipboard;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod crypto;
mod export;
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
    /// Export db
    Export(Export)

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
    #[arg(long)]
    file: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct ListCmd {
    #[arg(short)]
    encryption: bool,
    #[arg(short)]
    formatexport: bool,
}

#[derive(Args)]
struct Export {
    input: String,
    output: String,
    #[arg(short, long)]
    format: String,
    #[arg(short)]
    encryption: String,
    #[arg(short, long)]
    keyfile: bool,
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

#[derive(Debug)]
enum LoginType {
    PASSWORD,
    FILE,
}

enum FormatExport {
    CSV,
}

#[derive(Clone)]
pub struct DBManage {
    db: Vec<JsonDatabseKMH>,
}

impl DBManage {
    fn show(&self) {
        let mut creds: Vec<Vec<String>> = Vec::new();

        for (i, e) in self.db.iter().enumerate() {
            let i_string = i.to_string();
            let password = "*".repeat(e.password.len());
            let tc = Vec::from([
                i_string,
                e.title.to_string(),
                e.username.to_string(),
                password.to_string(),
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
    let all_formats = vec!["csv"];

    match &cli.command {
        Actions::Init(init) => {
            init_db(&init.filename);
        }

        Actions::Open(open) => {
            open_db(&open.filename, &open.encryption, open.file);
        }

        Actions::List(list) => {
            if list.encryption {
                interactive::tree_classic("Encryption list", all_encryptions);
            }
            if list.formatexport {
                interactive::tree_classic("Export format list", all_formats);
            }
        }
        Actions::Export(export) => {
            export_db(&export.input, &export.output, &export.format, &export.encryption, export.keyfile);
            
        }
    }

    return;
}

fn encrypt_database_password(encryption: Encryption, password: &String, filename: &String) {
    match encryption {
        Encryption::AES256GCM => {
            let out = crypto::encrypt_database_aes(&vec![], &password).unwrap();
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
        Encryption::SALSA20 => {
            let out = crypto::encrypt_database_salsa20(&vec![], &password);
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
        Encryption::CHACHA20 => {
            let out = crypto::encrypt_database_chacha20(&vec![], &password);
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
    }
}

fn encrypt_database_file(encryption: Encryption, password_file: Vec<u8>, filename: &String) {
    match encryption {
        Encryption::AES256GCM => {
            let out =
                crypto::encrypt_database_aes(&vec![], &String::from_utf8(password_file).unwrap())
                    .unwrap();
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
        Encryption::SALSA20 => {
            let out = crypto::encrypt_database_salsa20(
                &vec![],
                &String::from_utf8(password_file).unwrap(),
            );
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
        Encryption::CHACHA20 => {
            let out = crypto::encrypt_database_chacha20(
                &vec![],
                &String::from_utf8(password_file).unwrap(),
            );
            fs::File::create(filename)
                .unwrap()
                .write_all(out.as_slice())
                .unwrap();
        }
    }
}

fn decrypt_database(encryption: &Encryption, password: &String, db: Vec<u8>) -> Vec<JsonDatabseKMH>{
    let decrypted_db = match encryption {
        Encryption::AES256GCM => match crypto::decrypt_database_aes(db, &password) {
            Ok(r) => Some(r),
            Err(_) => None,
        },
        Encryption::SALSA20 => Some(crypto::decrypt_database_salsa20(db, &password)),
        Encryption::CHACHA20 => Some(crypto::decrypt_database_chacha20(db, &password)),
    };

    // Deserialize DB
    let json_db: Vec<JsonDatabseKMH> = serde_json::from_str(
        String::from_utf8(decrypted_db.unwrap())
            .expect("Bytes to String failed")
            .as_str(),
    )
    .expect("Invalid JSON format");

    json_db
}

fn check_for_modify(str: &str) -> Option<String> {
    if str.trim() != "" {
        return Some(str.to_string());
    }
    return None;
}

fn export_db(input: &String, output: &String, format: &String, encryption: &String, keyfile: bool) {
    let mut fbuffer = Vec::new();
    
    let fileformat = match format.to_lowercase().as_str() {
        "csv" => {
            FormatExport::CSV
        },
        _ => {
            println!("This format don't exist ¯\\_( ͡° ͜ʖ ͡°)_/¯");
            return;
        }
    };

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
    if fs::metadata(input).is_err() {
        println!("File not found");
        return;
    }

    // Open file
    let mut hfile = match fs::File::open(input) {
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

    let password: String = if keyfile {
        let mut filebuf = Vec::new();

        let keyfile_path = match interactive::ask("Insert keyfile path:") {
            Some(r) => r,
            None => {
                return;
            }
        };
        match fs::metadata(&keyfile_path) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }
        fs::File::open(&keyfile_path)
            .unwrap()
            .read_to_end(&mut filebuf)
            .unwrap();

        String::from_utf8(filebuf).unwrap()
    } else {
        // Get password
        match interactive::ask_password("password:", false) {
            Some(r) => r,
            None => {
                return;
            }
        }
    };

    let json_db = decrypt_database(&encryption_type, &password, fbuffer);

    // Init DB
    let dbmanage = DBManage { db: json_db };

    match fileformat {
        FormatExport::CSV => export::csv_export(&dbmanage, output),
    }

}

fn init_db(filename: &String) {
    let type_form = match interactive::select(
        vec!["Password", "File"],
        "What type of login do you want to use?",
    ) {
        Some(r) => r,
        None => {
            return;
        }
    };

    let ans = match interactive::select(
        vec!["AES256 GCM", "Salsa20", "Chacha20-Poly1305"],
        "Which cryptography do you want to use?",
    ) {
        Some(r) => r,
        None => {
            return;
        }
    };

    // data = size of keufile or password
    let (logintype, data) = match type_form.as_str() {
        "password" => {
            let password = match interactive::ask_password("Add a password:", true) {
                Some(r) => r,
                None => {
                    return;
                }
            };
            (LoginType::PASSWORD, password)
        }
        "file" => {
            let size_file = match interactive::select(vec!["1024", "2048", "4096"], "Keyfile size")
            {
                Some(r) => r,
                None => {
                    return;
                }
            };
            (LoginType::FILE, size_file)
        }
        _ => {
            return;
        }
    };

    match logintype {
        LoginType::PASSWORD => match ans.as_str() {
            "aes256 gcm" => {
                encrypt_database_password(Encryption::AES256GCM, &data, filename);
            }
            "salsa20" => {
                encrypt_database_password(Encryption::SALSA20, &data, filename);
            }
            "chacha20-poly1305" => {
                encrypt_database_password(Encryption::CHACHA20, &data, filename);
            }
            _ => (),
        },
        LoginType::FILE => {
            let size: usize = data.parse().unwrap();
            let keyfile_filename = match interactive::ask("Insert keyfile name:") {
                Some(r) => r,
                None => {
                    return;
                }
            };

            let keycontent = crypto::generate_random_utf8(size);

            fs::File::create(keyfile_filename)
                .unwrap()
                .write_all(keycontent.as_slice())
                .unwrap();

            match ans.as_str() {
                "aes256 gcm" => {
                    encrypt_database_file(Encryption::AES256GCM, keycontent, filename);
                }
                "salsa20" => {
                    encrypt_database_file(Encryption::SALSA20, keycontent, filename);
                }
                "chacha20-poly1305" => {
                    encrypt_database_file(Encryption::CHACHA20, keycontent, filename);
                }
                _ => (),
            }
        }
    }
}

fn open_db(filename: &String, encryption: &String, keyfile: bool) {
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

    let password: String = if keyfile {
        let mut filebuf = Vec::new();

        let keyfile_path = match interactive::ask("Insert keyfile path:") {
            Some(r) => r,
            None => {
                return;
            }
        };
        match fs::metadata(&keyfile_path) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }
        fs::File::open(&keyfile_path)
            .unwrap()
            .read_to_end(&mut filebuf)
            .unwrap();

        String::from_utf8(filebuf).unwrap()
    } else {
        // Get password
        match interactive::ask_password("password:", false) {
            Some(r) => r,
            None => {
                return;
            }
        }
    };

    let json_db = decrypt_database(&encryption_type, &password, fbuffer);

    // Init DB
    let mut dbmanage = DBManage { db: json_db };

    loop {
        // Database interaction
        dbmanage.show();

        let ans = match interactive::select(
            vec![
                "Add",
                "Remove",
                "Modify",
                "Show password",
                "Copy password",
                "Save",
                "Export in csv",
                "Exit"
            ],
            "What do you want to do?",
        ) {
            Some(r) => r,
            None => return,
        };

        match ans.to_lowercase().as_str() {
            "add" => {
                let title = interactive::ask("Title:").unwrap();
                let username = interactive::ask("Username:").unwrap();
                let password_asked = interactive::ask_password("Password:", false).unwrap();
                let notes = interactive::ask("Notes:").unwrap();

                dbmanage.db.push(JsonDatabseKMH {
                    title,
                    username,
                    password: password_asked,
                    notes,
                });
            }
            "remove" => {
                let id = interactive::ask("ID:").unwrap().parse::<usize>().unwrap();
                dbmanage.db.remove(id);
            }
            "modify" => {
                let id = interactive::ask("ID:").unwrap().parse::<usize>().unwrap();
                let id_selected = &mut dbmanage.db[id];
                let title = interactive::ask("Titie").unwrap();
                let username = interactive::ask("Username").unwrap();
                let password = interactive::ask_password("Password", false).unwrap();
                let notes = interactive::ask("Notes").unwrap();

                match check_for_modify(title.as_str()) {
                    Some(r) => id_selected.title = r,
                    None => (),
                };
                match check_for_modify(username.as_str()) {
                    Some(r) => id_selected.username = r,
                    None => (),
                }
                match check_for_modify(password.as_str()) {
                    Some(r) => id_selected.password = r,
                    None => (),
                }
                match check_for_modify(notes.as_str()) {
                    Some(r) => id_selected.notes = r,
                    None => (),
                }
            }
            "show password" => {
                let id = interactive::ask("ID:").unwrap().parse::<usize>().unwrap();
                let id_selected = dbmanage.db.get(id).expect("don't exist");

                println!("{}", id_selected.password);
                print!("Press enter for continue");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
            "copy password" => {
                let mut clipboard = Clipboard::new().unwrap();

                let id = interactive::ask("ID:").unwrap().parse::<usize>().unwrap();
                let id_selected = dbmanage.db.get(id).expect("don't exist").to_owned();

                clipboard.set_text(id_selected.password).unwrap()
            }
            "save" => {
                let edb = match &encryption_type {
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
            "export in csv" => {
                let exportfilename = interactive::ask("Output file:").unwrap();
                export::csv_export(&dbmanage, &exportfilename);
            }
            "exit" => {
                println!("Exiting...");
                process::exit(0);
            },
            _ => return
        }
        interactive::clear_screen();
    }
}
