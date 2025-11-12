use clap::Parser;
use colored::*;
use rand::Rng;
use rand::prelude::IndexedRandom;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Parser)]
#[command(version, about = "Random file generator", long_about = None)]
struct Args {
    #[arg(short, long)]
    amount: u32,

    #[arg(short, long, default_value = ".")]
    path: String,

    #[arg(short, long, required = true)]
    extension: String,

    #[arg(long, default_value_t = true)]
    noprint: bool,
}

pub enum CharSetKind {
    Uppercase,
    Lowercase,
    Digits,
    Symbols,
}

impl CharSetKind {
    pub fn chars(&self) -> &'static [char] {
        match self {
            CharSetKind::Uppercase => &UPPERCASE_LETTERS,
            CharSetKind::Lowercase => &LOWERCASE,
            CharSetKind::Digits => &NUMBERS,
            CharSetKind::Symbols => &SYMBOLS,
        }
    }
}

const UPPERCASE_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const SYMBOLS: [char; 26] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '=', '?', '>', '<', ',', '.', ':',
    ';', '"', '[', ']', '{', '}', '|',
];

fn generate_data(length: u32) -> String {
    let mut text_string = String::new();
    let mut rng = rand::rng();
    for _ in 0..length {
        let selection = rng.random_range(1..=4);
        let charset = match selection {
            1 => CharSetKind::Uppercase,
            2 => CharSetKind::Lowercase,
            3 => CharSetKind::Digits,
            4 => CharSetKind::Symbols,
            _ => unreachable!(),
        };

        if let Some(&c) = charset.chars().choose(&mut rng) {
            text_string.push(c);
        };
    }

    text_string
}

fn generate_file_name() -> String {
    let name_len: u32 = 6;
    let mut file_name = String::new();
    let mut rng = rand::rng();

    for _ in 0..name_len {
        let selection = rng.random_range(1..=3);
        let charset = match selection {
            1 => CharSetKind::Uppercase,
            2 => CharSetKind::Lowercase,
            3 => CharSetKind::Digits,
            _ => unreachable!(),
        };

        if let Some(&c) = charset.chars().choose(&mut rng) {
            file_name.push(c);
        };
    }

    file_name
}

fn create_file(path: &Path) -> Result<File, std::io::Error> {
    let file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error while trying to create a file! {}", e);
            return Err(e);
        }
    };
    Ok(file)
}

fn create_files(
    amount: u32,
    path: &String,
    extension: &String,
    printflag: bool,
) -> Result<Vec<File>, std::io::Error> {
    let mut files = Vec::<File>::new();
    let default_data_length: u32 = 4096;

    for _ in 1..=amount {
        let random_file_name = generate_file_name();
        let formatted_name = format!("{}.{}", random_file_name, extension);
        let formatted_path = Path::new(path).join(&formatted_name);
        let file = match create_file(&formatted_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error trying to create a file! {}", e);
                return Err(e);
            }
        };

        if printflag {
            println!(
                "{}",
                format!("Created: {}", formatted_path.display()).bright_cyan()
            );
        }

        let data = generate_data(default_data_length);
        match write_to_file(&file, &data) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error trying to write into file! {}", e);
                return Err(e);
            }
        }
        files.push(file);
    }

    Ok(files)
}

fn write_to_file(mut file: &File, content: &String) -> Result<(), std::io::Error> {
    let bytes = content.as_bytes();
    match file.write_all(bytes) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error trying to write into file! {}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let amount = args.amount;
    let path = args.path;
    let extension = args.extension;
    let printbool = args.noprint;

    match create_files(amount, &path, &extension, printbool) {
        Ok(files) => {
            println!(
                "{}",
                format!("Successfully created {} files!", files.len()).cyan()
            );
            println!("{}", format!("Path: {}", path).yellow());
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to create files: {}", e).bright_red());
        }
    }
}
