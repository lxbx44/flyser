use clap::Parser;
use file_format::FileFormat;
use std::{
    env,
    fs::{metadata, File},
    io::{BufRead, BufReader},
};

const DEFAULT_FALIURE: &str = "Unknown";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_name: String,
}

fn main() {
    let args = Args::parse();
    let mut file_path = match env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Cound get the current path");
            return;
        }
    };
    file_path.push(&args.file_name);

    let file_extension = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(DEFAULT_FALIURE),
        None => DEFAULT_FALIURE,
    };


    let metadata = match metadata(&file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                metadata
            } else {
                eprintln!("\"{}\" is not a file!", args.file_name);
                return;
            }
        }
        Err(_) => {
            eprintln!("Could not read the metadata on {}", args.file_name);
            return;
        }
    };
    let file_size = metadata.len() as f64 / 1_048_576.0;
    
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Could not open \"{}\"", args.file_name);
            return;
        }
    };

    let file_buffer = BufReader::new(&file);

    let (line_count, word_count, char_count) =
        file_buffer.lines().fold((0, 0, 0), |counts, line_result| {
            let line = line_result.unwrap_or_else(|_| String::new());
            (
                counts.0 + 1,
                counts.1 + line.split_whitespace().count(),
                counts.2 + line.chars().count(),
            )
        });

    let file_type = match FileFormat::from_file(file_path.clone()) {
        Ok(format) => {
            let format = format.name().to_owned();
            if format == *"Arbitrary Binary Data" {
                get_type_from_ext(file_extension).to_string()
            } else {
                format
            }
        }
        Err(_) => String::from(DEFAULT_FALIURE),
    };

    println!("[Flyser :3]\n");
    println!("File name: {}", args.file_name.split('/').last().unwrap_or(DEFAULT_FALIURE).split('.').next().unwrap_or(DEFAULT_FALIURE));
    println!("File type: {file_type}");
    println!(
        "File path: {}",
        file_path.to_str().unwrap_or(DEFAULT_FALIURE)
    );

    println!("\nFile size: {file_size:.5} MB");

    println!("\nTotal number of lines: {line_count}");
    println!("Total number or words: {word_count}");
    println!("Total number of characters: {char_count}");
}

fn get_type_from_ext(ext: &str) -> &str {
    match ext {
        "rs" => "Rust (rs)",
        "py" => "Python (py)",
        "html" => "HyperText Markup Language (html)",
        "css" => "Cascading Style Sheets (css)",
        "scss" => "Syntactically Awesome Style Sheets (scss)",
        "jar" => "Java (jar)",
        "java" => "Java (java)",
        "cpp" => "C++ (cpp)",
        "js" => "JavaScript (js)",
        "ts" => "TypeScript (ts)",
        "php" => "PHP (php)",
        "swift" => "Swift (swift)",
        "go" => "Go (go)",
        "rb" => "Ruby (rb)",
        "c" => "C (c)",
        "cs" => "C# (cs)",
        "csx" => "C# (csx)",
        "vb" => "Visual Basic (vb)",
        "kt" => "Kotlin (kt)",
        "r" => "R (r)",
        "m" => "MATLAB (m)",
        "sql" => "SQL (sql)",
        "sh" => "Shell Script (sh)",
        "bat" => "Batch (bat)",
        "ps1" => "PowerShell (ps1)",
        "toml" => "Tom's Obvious Minimal Language (toml)",

        _ => DEFAULT_FALIURE,
    }
}
