
#![allow(unused)]
pub mod database_manager;

use clap::Parser;
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use indicatif::ProgressBar;

// TODO: tests (example below)
// #[test]
// fn test_check_content() {
//     assert_eq!(function, expected_resault);
// }

#[derive(Parser)]
struct Cli {
    sql_dialect: String,
    path: std::path::PathBuf,
}

// fn check_content(content : String, pattern : String) -> String {
//     for line in content.lines() {
//         if line.contains(&pattern) {
//             // println!("{}", line);
//             return line.to_string();
//         }
//     }
//     return "No match found".to_string();
// }

fn run_db_script() -> Result<()> {
    // parse sql instructions from file
    // connect to database
    // execute sql instructions
    // close connection



    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let sql_dialect = args.sql_dialect.to_lowercase();
    // path is optional arg, if not provided - current dir
    let path = args.path.parent().unwrap_or(std::path::Path::new("."));

    if sql_dialect == "mysql" || sql_dialect == "sqlite" || sql_dialect == "postgresql" {
        info!("{} language selected", sql_dialect);
    } else {
        error!("Unknown dialect selected");
        std::process::exit(1);
    }

    // get number of files in the directory
    let num_files = std::fs::read_dir(path)
        .context("Failed to count files in directory")?
        .count()
        // convert to u64
        .try_into()
        .context("Failed to convert to u64")?;


    // get all files in the directory with .sql extension
    // let files = std::fs::read_dir(path
    //     .join("*.sql"))
    //     .context("Failed to found files")?;


    let pb = ProgressBar::new(num_files);
    pb.println(format!("[1/3] We are getting the work done..."));
    for i in 0..num_files {
        // work with files to database
        std::thread::sleep(std::time::Duration::from_millis(100));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    Ok(())
}