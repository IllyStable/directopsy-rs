use std::fs;
use directopsy::foundfile::{FoundFile, FileType};
use clap::Parser;

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum Filter {
    File, Directory, All
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short)]
    #[clap(help = "What to display")]
    #[clap(value_enum, default_value_t=Filter::All)]
    filter: Filter
}

fn main() {
    let args = Args::parse();
    let mut paths: Vec<FoundFile> = fs::read_dir("./").unwrap().map(|pathbuf| FoundFile::from(pathbuf.unwrap().path())).collect();

    if args.filter != Filter::All {
        paths = paths.into_iter().filter(|path| {
            match args.filter {
                Filter::File => {
                    return path.file_type == FileType::File;
                },
                Filter::Directory => {
                    return path.file_type == FileType::Directory;
                },
                _ => {
                    panic!("Unknown filter");
                }
            }  
        }).collect();
    }

    let longest_name_length = paths.iter().map(|file| file.name.len()).max().unwrap_or(0).max("File name".len());

    let mut row = "┌".to_string();

    row += &"─".repeat(longest_name_length + 1);
    row += "┬";
    row += "──────";
    row += "┐";

    println!("{}", row);

    row = "│File name".to_string();

    row += &" ".repeat(longest_name_length + 1 - "File name".len());
    row += "│ ";
    row += "Type │ ";

    println!("{}", row);

    row = "├".to_string();
    row += &"─".repeat(longest_name_length + 1);
    row += "┼──────┤";

    println!("{}", row);

    for path in paths {
        let file_type: &str = (&path.file_type).into();
        row = "│".to_string();

        row += &path.name;
        row += &" ".repeat(longest_name_length + 1 - path.name.len());
        row += "│ ";
        row += &file_type;
        row += &" ".repeat(5 - file_type.len());

        row += "│";

        println!("{}", row)
    }

    row = "└".to_string();

    row += &"─".repeat(longest_name_length + 1);
    row += "┴";
    row += "──────";
    row += "┘";

    println!("{}", row);
}
