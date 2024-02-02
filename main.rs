use std::{fs, io::Write, process};

use arboard::Clipboard;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod crypto;
mod export;
mod interactive;
mod utils;

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
    Export(Export),
    /// Import db
    Import(Import),
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
struct Import {
    input: String,
    output: String,
    #[arg(short, long)]
    format: String,
    #[arg(short)]
    encryption: String,
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
pub enum Encryption {
    AES256GCM,
    SALSA20,
    CHACHA20,
}

#[derive(Debug)]
pub enum LoginType {
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
            export_db(
                &export.input,
                &export.output,
                &export.format,
                &export.encryption,
                export.keyfile,
            );
        }
        Actions::Import(import) => {
            import_db(
                &import.input,
                &import.output,
                &import.format,
                &import.encryption,
            );
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

fn decrypt_database(
    encryption: &Encryption,
    password: &String,
    db: Vec<u8>,
) -> Vec<JsonDatabseKMH> {
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

// TODO: finish import
fn import_db(input: &String, output: &String, format: &String, encryption: &String) {
    init_db(output);
}

fn export_db(input: &String, output: &String, format: &String, encryption: &String, keyfile: bool) {
    let fileformat = match format.to_lowercase().as_str() {
        "csv" => FormatExport::CSV,
        _ => {
            println!("This format doesn't exist ¯\\_( ͡° ͜ʖ ͡°)_/¯");
            return;
        }
    };

    let encryption = utils::encryption_str_to_enum(encryption);

    let (dbmanage, _) = interactive::ask_cred_db(&encryption, keyfile, input).unwrap();

    match fileformat {
        FormatExport::CSV => export::csv_export(&dbmanage, output),
    }
}

fn init_db(filename: &String) {
    let (logintype, encryption, password) = interactive::ask_init_db();

    match logintype {
        LoginType::PASSWORD => match encryption {
            Encryption::AES256GCM => {
                encrypt_database_password(encryption, &password, filename);
            }
            Encryption::SALSA20 => {
                encrypt_database_password(encryption, &password, filename);
            }
            Encryption::CHACHA20 => {
                encrypt_database_password(encryption, &password, filename);
            }
        },
        LoginType::FILE => {
            let keyfile_filename = match interactive::ask("Insert keyfile name:") {
                Some(r) => r,
                None => {
                    return;
                }
            };

            fs::File::create(keyfile_filename)
                .unwrap()
                .write_all(password.as_bytes())
                .unwrap();

            match encryption {
                Encryption::AES256GCM => {
                    encrypt_database_file(encryption, password.as_bytes().to_vec(), filename);
                }
                Encryption::SALSA20 => {
                    encrypt_database_file(encryption, password.as_bytes().to_vec(), filename);
                }
                Encryption::CHACHA20 => {
                    encrypt_database_file(encryption, password.as_bytes().to_vec(), filename);
                }
            }
        }
    }
}

fn open_db(filename: &String, encryption: &String, keyfile: bool) {
    let encryption = utils::encryption_str_to_enum(encryption);
    let (mut dbmanage, password) =
        interactive::ask_cred_db(&encryption, keyfile, &filename).unwrap();

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
                "Exit",
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
                let edb = match &encryption {
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
            }
            _ => return,
        }
        interactive::clear_screen();
    }
}
