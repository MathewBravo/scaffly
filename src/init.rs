use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process::Command;
use std::{env, fs::File, path::Path};

#[derive(Serialize, Deserialize)]
struct Scaffolds {
    scaffolds: Vec<Scaffold>,
}

#[derive(Serialize, Deserialize)]
struct Scaffold {
    name: String,
    crates: Vec<String>,
}

pub fn init_scaff(name: String, crates: Vec<String>) -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Could not get your home directory");
    let scaffly_dir = format!("{}/.scaffly", home_dir);
    let scaf_json_loc = format!("{}/scaffly.json", &scaffly_dir);
    let scaffolds: Scaffolds = if Path::new(&scaf_json_loc).exists() {
        let file = File::open(&scaf_json_loc)?;
        serde_json::from_reader(file).expect("Could not read scaffly.json")
    } else {
        Scaffolds {
            scaffolds: Vec::new(),
        }
    };

    let new_scaffold = Scaffold { name, crates };

    let mut updated_scaffolds = scaffolds;
    updated_scaffolds.scaffolds.push(new_scaffold);

    let file = File::create(&scaf_json_loc)?;
    serde_json::to_writer_pretty(file, &updated_scaffolds).expect("Could not write scaffly.json");

    Ok(())
}

pub fn init_rs_proj(name: String, scaf: String) {
    if name == "test" {
        println!("The name test cannot be used it conflicts with Rust's built-in test library")
        return;
    }
    let home_dir = env::var("HOME").expect("Could not get your home directory");
    let scaffly_dir = format!("{}/.scaffly", home_dir);
    let scaf_json_loc = format!("{}/scaffly.json", &scaffly_dir);

    if let Ok(file) = File::open(&scaf_json_loc) {
        let scaffolds: Scaffolds =
            serde_json::from_reader(file).expect("Could not read scaffly.json");

        if let Some(scaffold) = scaffolds.scaffolds.iter().find(|s| s.name == scaf) {
            println!("Scaffolding {} with crates:", scaffold.name);
            for crate_name in &scaffold.crates {
                println!("{}", crate_name);
            }

            run_cargo_init(&name);

            run_cargo_add(&name, &scaffold.crates);
        } else {
            println!("Scaffold {} not found in scaffly.json", scaf);
        }
    } else {
        println!("scaffly.json not found. No scaffolds available.");
    }
}

fn run_cargo_init(name: &str) {
    println!("{name}");
    io::stdout().flush().unwrap(); // Flush the buffer
    Command::new("cargo")
        .arg("init")
        .arg(name)
        .output()
        .expect("Failed to run cargo init with your name");
}

fn run_cargo_add(name: &str, crates: &[String]) {
    if let Err(err) = env::set_current_dir(name) {
        eprintln!("Failed to change directory: {}", err);
        return;
    }

    for crate_name in crates {
        let status = Command::new("cargo").arg("add").arg(crate_name).status();

        match status {
            Ok(exit_status) => {
                if !exit_status.success() {
                    eprintln!("Failed to add crate: {}", crate_name);
                }
            }
            Err(err) => {
                eprintln!("Failed to run cargo add: {}", err);
            }
        }
    }
}
