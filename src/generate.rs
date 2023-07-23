use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Scaffolds {
    scaffolds: Vec<Scaffold>,
}

#[derive(Serialize, Deserialize)]
struct Scaffold {
    name: String,
    crates: Vec<String>,
}

pub fn initial_startup() {
    let home_dir = env::var("HOME").expect("Could not get home directory");
    let scaffly_dir = format!("{}/.scaffly", home_dir);

    fs::create_dir_all(&scaffly_dir).expect("Could not initialize scaffly directory");

    let scaf_json_loc = format!("{}/scaffly.json", &scaffly_dir);
    if !Path::new(&scaf_json_loc).exists() {
        // Only create the JSON file and write the data if it doesn't exist already
        let scaffolds = Scaffolds {
            scaffolds: Vec::new(),
        };
        let file = File::create(&scaf_json_loc).expect("Could not create scaffly.json");
        serde_json::to_writer_pretty(file, &scaffolds).expect("Could not write scaffly.json");
    }
}
