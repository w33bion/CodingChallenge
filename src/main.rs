mod generator;
mod reader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "generate" => generate(),
        "read" => read(),
        _ => panic!("Unexpected operator passed")
    }
}

fn generate() {
    println!("generating tokens...");
    match generator::write_tokens() {
        Err(why) => panic!("couldn't generate tokens: {}", why),
        Ok(_res) => println!("generated tokens.")
    }
}

fn read() {
    println!("reading tokens...");
    reader::read_tokens();
    println!("sucessfully read tokens.")
}
