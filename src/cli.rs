use clap::{Parser, Subcommand};
use crate::io::BadLockIO;

#[derive(Parser)]
#[command(author, version, about)]
pub struct BadLockCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "encrypt file with password(s)")]
    Lock {
        #[arg(help = "file to be encrypted")]
        file: String,
        #[arg(short = 's', long, help = "secret used to encrypt the file")]
        secret: Option<String>,
        #[arg(short = 'p', long, help = "passwords to encrypt the secret")]
        passwords: Vec<String>,
        #[arg(short = 'o', long, help = "destination of the encrypted file")]
        output: Option<String>,
    },
    #[command(about = "decrypt file with password")]
    Unlock {
        #[arg(help = "file to be decrypted")]
        file: String,
        #[arg(short = 'p', long, help = "password to decrypt the file")]
        password: String,
        #[arg(short = 'o', long, help = "destination of the decrypted file")]
        output: Option<String>,
    },
}

impl BadLockCli {
    pub fn delegate_main() {
        match BadLockCli::try_parse() {
            Ok(BadLockCli { command }) => match command {
                Commands::Lock { file, secret, passwords, output } => {
                    match BadLockIO::lock(&file, secret, passwords, output) {
                        Ok(o) => println!("The file at [{}] was successfully encrypted, output is located at {:?}", file, o),
                        Err(err) => println!("{}", err),
                    }
                }
                Commands::Unlock { file, password, output } => {
                    match BadLockIO::unlock(&file, &password, output) {
                        Ok(o) => println!("The file at [{}] was successfully decrypted using the password [{}]\n(output is located at {:?})", file, password, o),
                        Err(err) => println!("{}", err),
                    }
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
