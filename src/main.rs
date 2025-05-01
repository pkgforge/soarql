use std::{
    env,
    fs::{self, File},
    io,
    path::Path,
};

use models::RemotePackage;
use repository::PackageRepository;
use rusqlite::Connection;
use statements::Statements;

mod models;
mod repository;
mod statements;

fn create_database<P: AsRef<Path>>(
    path: P,
    repo_name: &str,
    packages: &[RemotePackage],
) -> Result<(), rusqlite::Error> {
    let path = path.as_ref();

    if path.exists() {
        if path.is_symlink() || path.is_file() {
            fs::remove_file(path).unwrap();
        } else {
            println!("{} exists but is not a file.", path.display());
        }
    }

    File::create(path).unwrap();

    let mut conn = Connection::open(path).unwrap();
    let tx = conn.transaction()?;

    let sql = include_str!("metadata.sql");
    match tx.execute_batch(sql) {
        Ok(_) => {
            tx.pragma_update(None, "user_version", 1)?;
            tx.commit()?;
        }
        Err(err) => return Err(err),
    };

    let tx = conn.transaction()?;
    {
        let statements = Statements::new(&tx)?;

        let mut repo = PackageRepository::new(&tx, statements, repo_name);
        repo.import_packages(packages, "unknown")?;
    }

    tx.commit()?;

    Ok(())
}

fn abort(msg: &str) -> ! {
    eprintln!("{msg}");
    let usage = r#"
Soar JSON metadata to SQLite converter

Options:
   --input, -i                  Input JSON file
   --output, -o                 Output SQLite file
   --repo, -r                   Name of the repository
    "#;
    eprintln!("{usage}");
    std::process::exit(1);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut input_file = None;
    let mut output_file = None;
    let mut repo_name = None;

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--input" | "-i" => {
                if let Some(next) = iter.next() {
                    if next.starts_with("-") {
                        abort("Expected input file name, got flag instead.");
                    }
                    input_file = Some(next);
                } else {
                    abort("Input file name is not provided");
                }
            }
            "--output" | "-o" => {
                if let Some(next) = iter.next() {
                    if next.starts_with("-") {
                        abort("Expected output file name, got flag instead.");
                    }
                    output_file = Some(next);
                } else {
                    abort("Output file name is not provided");
                }
            }

            "--repo" | "-r" => {
                if let Some(next) = iter.next() {
                    if next.starts_with("-") {
                        abort("Expected repository name, got flag instead.");
                    }
                    repo_name = Some(next);
                } else {
                    abort("Repository name is not provided");
                }
            }
            arg => {
                if arg.starts_with("-") {
                    abort(&format!("Unknown argument '{arg}'"));
                }
            }
        }
    }

    if input_file.is_none() || output_file.is_none() || repo_name.is_none() {
        abort("Missing required options.");
    }

    let input_file = input_file.unwrap();
    let output_file = output_file.unwrap();
    let repo_name = repo_name.unwrap();

    let file = fs::read_to_string(input_file)?;
    let packages: Vec<RemotePackage> = serde_json::from_str(&file)?;

    create_database(output_file, repo_name, packages.as_slice()).unwrap();

    Ok(())
}
