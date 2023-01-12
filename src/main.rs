use serde_json::{Result, Value};
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use std::path::Path;
use base64::{encode, decode};

/// Program to convert base64 files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Complete path to the file data
    #[clap(short, long)]
    path: String,

    /// Complete path where the file output should be created
    #[clap(short, long, default_value = "./output")]
    output: String,

    /// Original encoding of the data
    #[clap(short, long, default_value = "base64")]
    encoding: String,
}


fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let output = Path::new(&args.output);
    let encoding = args.encoding;
    let mut contents= String::new();
    let mut file_in = File::open(&path).expect("Error opening File");
    file_in.read_to_string(&mut contents).expect("Unable to read to string");
    let mut file_out = File::create(output).expect("Error when creating the output file");
    match encoding.as_str() {
        "base64" => {
            if path.extension().expect("Error getting the extention") == "json" {
                let val: Value = serde_json::from_str(&contents).expect("Error deserializing");
                let mut data_b64 = val["dato"].as_str().unwrap(); // contains the base64 data
                if data_b64.is_empty() {
                    data_b64 = val["archivo"][0].as_str().unwrap(); // contains the base64 data
                }
                let data_decoded = &decode(data_b64.as_bytes()).expect("There was an error converting the data to bytes");
                file_out.write_all(data_decoded).expect("Error when writing to output file");

            } else {
                let data_decoded = &decode(contents.as_bytes()).expect("There was an error converting the data to bytes");
                file_out.write_all(data_decoded).expect("Error when writing to output file");

            }
        },
        "hexadecimal" => {
            let data_decoded = &contents.as_bytes();
            file_out.write_all(data_decoded).expect("Error when writing to output file");
        }
        _ => {
            println!("Encoding not recognized. File not converted!!");
        }
    };
    println!("Program terminated successfully.");
}