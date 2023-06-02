use std::{
    fs::{File,metadata},
    io::{BufRead, BufReader},
    env,
};
use clap::Parser;
use file_format::FileFormat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_name: String,
}

fn main() {
    let args = Args::parse();
    let mut file_path = env::current_dir().unwrap();
    file_path.push(&args.file_name);

    let file_name = args.file_name;

    let file = File::open(&file_path).expect("Failed to open the file");
    let file_buffer = BufReader::new(&file);

    let (line_count, word_count, char_count) = file_buffer.lines().fold((0, 0, 0), |counts, line_result| {
        let line = line_result.unwrap_or_else(|_| String::new());

        let line_chars: Vec<char> = line.chars().collect();
        let line_words: Vec<&str> = line.split_whitespace().collect();

        (counts.0 + 1, counts.1 + line_words.len(), counts.2 + line_chars.len())
    });

    let file_size = metadata(&file_path).expect("Error retrieving file metadata").len();

    let file_type = match FileFormat::from_file(file_path.clone()) {
        Ok(format) => format.name().to_owned(),
        Err(_) => String::from("Unknown")
    };

    println!("[Flyser :3]\n");
    println!("File name: {}", file_name);
    println!("File type: {}", file_type);
    println!("File path: {:?}", file_path);

    println!("\nFile size: {:.5} MB", file_size);

    println!("\nTotal number of lines: {}", line_count);
    println!("Total number or words: {}", word_count);
    println!("Total number of characters: {}", char_count);
}
