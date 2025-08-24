// Accept command line arguments
use std::env;
// Read file
use std::fs::File;
// Save struct to file
use savefile::save_file;
// Deserialize json from file
use serde::Deserialize;
use serde_json::from_reader;
// Serialize to file
use savefile_derive::Savefile;
// The struct defination of the hitokotos.
#[derive(Deserialize, Savefile)]
struct Hitokoto {
    /// The id of the hitokoto.
    id: i64,
    /// The content of the hitokoto.
    content: String,
    /// The source of the hitokoto, author or book etc.
    from: String,
    /// The optional character of the hitokoto.
    from_who: Option<String>,
    /// The user id of the hitokoto, be used to query the owner.
    user_id: i64,
    /// The create time of the hitokoto.
    created_at: u64,
}

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // Init file path
    let json_path = &args[1];
    let output_path = &args[2];

    // Deserialize struct from json file
    let file = File::open(json_path).expect(format!("Failed to read: {}", json_path).as_str());

    let structs: Vec<Hitokoto> = from_reader(file).unwrap();
    save_file(output_path, 1, &structs).expect("Failed to serialize structs");
    println!("Convert finished.")
}
