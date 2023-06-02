use std::{
    fs::{File,metadata},
    io::{BufRead, BufReader},
    env,
};
use clap::Parser;
use file_format::{FileFormat, Kind};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String,
}

fn main() {
    let args = Args::parse();
    let mut cur_path = env::current_dir().unwrap();
    cur_path.push(&args.name);

    let name = cur_path.file_name().unwrap().to_str().unwrap();
    let path = cur_path.as_os_str().to_str().unwrap();
    let in_ext = cur_path.extension().unwrap().to_str().unwrap();
    
    let file_p = &args.name;

    let file = match File::open(file_p) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file:\n{}", err);
            std::process::exit(1);
        }
    };
    
    let filer = BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    for line in filer.lines() {
        if let Ok(line) = line {
            line_count += 1;
            word_count += line.split_whitespace().count();
            char_count += line.chars().count();
        }
    }

    match std::fs::metadata(file_p) {
        Ok(metadata) => metadata,
        Err(err) => {
            panic!("Error retrieving file metadata:\n{}", err);
        }
    };

    let file_size_byte: u64 = metadata(file_p).unwrap().len();
    let file_size: f64 = file_size_byte as f64 / 1_048_576.0;

    let ext = get_ext(in_ext);


    println!("[Flyser :3]\n");
    println!("File name: {}", name);
    println!("File extension: {}", ext);
    println!("File path: {}", path);

    println!("\nFile size: {:.5} MB", file_size);

    println!("\nTotal number of lines: {}", line_count);
    println!("Total number or words: {}", word_count);
    println!("Total number of characters: {}", char_count);
}


fn get_ext(ext: &str) -> &str {
    match ext {
        "rs" => return "Rust (rs)",
        "py" => return "Python (py)",
        "html" => return "HyperText Markup Language (html)",
        "css" => return "Cascading Style Sheets (css)",
        "scss" => return "Syntactically Awesome Style Sheets (scss)",
        "jar" => return "Java (jar)",
        "java" => return "Java (java)",
        "cpp" => return "C++ (cpp)",
        "js" => return "JavaScript (js)",
        "ts" => return "TypeScript (ts)",
        "php" => return "PHP (php)",
        "swift" => return "Swift (swift)",
        "go" => return "Go (go)",
        "rb" => return "Ruby (rb)",
        "c" => return "C (c)",
        "cs" => return "C# (cs)",
        "csx" => return "C# (csx)",
        "vb" => return "Visual Basic (vb)",
        "kt" => return "Kotlin (kt)",
        "r" => return "R (r)",
        "m" => return "MATLAB (m)",
        "sql" => return "SQL (sql)",
        "sh" => return "Shell Script (sh)",
        "bat" => return "Batch (bat)",
        "ps1" => return "PowerShell (ps1)",

        _ => return "Unknow"
    }
}
