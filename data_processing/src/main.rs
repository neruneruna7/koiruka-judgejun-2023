mod lib;

// use lib::json_get_text;

use lib::json_get_text;


const FOLDER_PATH: &str = "static/data";


use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};

fn main() {
    let folder = PathBuf::from(FOLDER_PATH);
    let file_path = folder.join("oasst1_89k_ja.json");
    let output_path = folder.join("texts1.csv");

    println!("file_path: {:?}", &file_path);

    let file = File::open(file_path).expect("Failed to open the file");
    let buf_reader = BufReader::new(file);
    let parsed_data =  serde_json::from_reader(buf_reader).expect("Failed to read the file");

    json_get_text(
        output_path.to_str().unwrap(),
        parsed_data,
        "text_ja"
    ).unwrap(
    );
}

