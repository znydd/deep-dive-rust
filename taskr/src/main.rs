use std::env;
use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

enum Priority {
    Low,
    Mid,
    High,
}

struct Task {
    id: u32,
    title: String,
    priority: Priority,
    completed: bool,
    created_at: String,
    completed_at: Option<String>,
}

fn setup_store(folder_path: &Path, file_path: &Path) -> io::Result<PathBuf> {
    if !folder_path.exists() {
        println!("Folder does not exist. Creating '{}'...", folder_path.display());
        fs::create_dir_all(folder_path)?;
    } else {
        println!("Folder '{}' already exists.", folder_path.display());
    }

    if !file_path.exists() {
        println!("File does not exist. Creating '{}'...", file_path.display());
        let _task_file = File::create(file_path)?;
    } else {
        println!("File '{}' already exists.", file_path.display());
    }
    Ok(PathBuf::from(file_path))
}

fn main()-> Result<()>{
    let home_dir = env::var("HOME")?;
    let folder_path = PathBuf::from(&home_dir).join(".taskr");
    let file_path = folder_path.join("task.json");

    let path = setup_store(&folder_path, &file_path).context("Failed to setup store")?;

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path).context("Failed to open file")?;
   

    writeln!(file, "New task appended!").context("Failed to write to file")?;
    println!("Wrote to file successfully.");

    let args: Vec<String> = env::args().collect();
    println!("ProgName: {}", args[0]);

    if args.len() > 1 {
        println!("First Argument: {}", args[1]);
    }

    for i in 1..args.len() {
        println!("Arg: {}", args[i]);
    }

    Ok(())
}
