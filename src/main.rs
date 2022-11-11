use serde::{Deserialize, Serialize};

extern crate serde_json;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    id: String,
    firstName: String,
    lastName: String,
    group: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    // .expect("something went wrong reading the file");
    // println!("With text:\n{}", contents);

    // let config: Config = serde_json::from_reader(reader).unwrap();

    // println!("Desrieialized Config = {:?}", config);

    let values: Config = serde_yaml::from_reader(reader).unwrap();

    println!("Desrieialized valus = {:?}", values);

    let new_f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("new_config.yml")
        .expect("Couldn't open file");

    serde_yaml::to_writer(new_f, &values).unwrap();
}
