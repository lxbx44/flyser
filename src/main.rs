use clap::Parser;
use file_format::FileFormat;
use std::{
    env,
    fs::{metadata, File},
    io::{BufRead, BufReader},
};

use comfy_table::{
    presets::UTF8_FULL,
    modifiers::UTF8_ROUND_CORNERS,
    Attribute,
    Cell,
    ContentArrangement,
    Table,
    TableComponent
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


    let name = args.file_name.split('/').last().unwrap_or(DEFAULT_FALIURE).split('.').next().unwrap_or(DEFAULT_FALIURE);

    let f_size: f64 = round_num(file_size, 5);

    let mut table = Table::new();

    table.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80)
        .set_header(vec![
                    String::from("Flyser <3")
        ])
        .add_row(vec![
                 Cell::new("File name").add_attribute(Attribute::Bold).fg(comfy_table::Color::Red),
                 Cell::new("\u{ea9f}").fg(comfy_table::Color::Red),
                 Cell::new(&name)
        ])
        .add_row(vec![
                 Cell::new("File type").add_attribute(Attribute::Bold).fg(comfy_table::Color::Yellow),
                 Cell::new("\u{ea9f}").fg(comfy_table::Color::Yellow),
                 Cell::new(&file_type)
        ])
        .add_row(vec![
                Cell::new("File path").add_attribute(Attribute::Bold).fg(comfy_table::Color::Green),
                Cell::new("\u{ea9f}").fg(comfy_table::Color::Green),
                Cell::new(&file_path.to_str().unwrap_or(DEFAULT_FALIURE))
        ])
        .add_row(vec![
                Cell::new("File size").add_attribute(Attribute::Bold).fg(comfy_table::Color::Cyan),
                Cell::new("\u{ea9f}").fg(comfy_table::Color::Cyan),
                Cell::new(format!("{} MB", &f_size))
        ])
        .add_row(vec![
                 Cell::new("Lines").add_attribute(Attribute::Bold).fg(comfy_table::Color::Blue),
                 Cell::new("\u{ea9f}").fg(comfy_table::Color::Blue),
                 Cell::new(&line_count)
        ])
        .add_row(vec![
                 Cell::new("Words").add_attribute(Attribute::Bold).fg(comfy_table::Color::Magenta),
                 Cell::new("\u{ea9f}").fg(comfy_table::Color::Magenta),
                 Cell::new(&word_count)
        ])
        .add_row(vec![
                 Cell::new("Characters").add_attribute(Attribute::Bold).fg(comfy_table::Color::Red),
                 Cell::new("\u{ea9f}").fg(comfy_table::Color::Red),
                 Cell::new(&char_count)
        ]);
        
    

    let table = format_table(&mut table);
    println!("{table}")
}

fn format_table(table: &mut Table) -> &mut Table {
    table.set_style(TableComponent::VerticalLines, ' ');
    table.remove_style(TableComponent::HorizontalLines);
    table.remove_style(TableComponent::MiddleIntersections);
    table.set_style(TableComponent::MiddleHeaderIntersections, '─');
    table.remove_style(TableComponent::LeftBorderIntersections);
    table.remove_style(TableComponent::RightBorderIntersections);
    table.set_style(TableComponent::TopBorderIntersections, '─');
    table.set_style(TableComponent::BottomBorderIntersections, '─');
    table.set_style(TableComponent::HeaderLines, '─');
    table.set_style(TableComponent::LeftHeaderIntersection, '├');
    table.set_style(TableComponent::RightHeaderIntersection, '┤')
}

fn round_num(number: f64, decimal_places: u32) -> f64 {
    let multiplier = 10_f64.powi(decimal_places as i32);
    (number * multiplier).round() / multiplier
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
        "exe" => "Windows executable (exe)",

        _ => DEFAULT_FALIURE,
    }
}
