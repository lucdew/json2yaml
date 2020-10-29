/**
 * Quick and dirty json to yaml converter
 */
extern crate clap;

use clap::{App, Arg};
use serde_json::Value;
use std::fmt;
use std::fs;
use std::fs::File;
use std::process::exit;

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<serde_yaml::Error> for MyError {
    fn from(err: serde_yaml::Error) -> Self {
        MyError::new(err.to_string().as_str())
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        MyError::new(err.to_string().as_str())
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::new(err.to_string().as_str())
    }
}

fn json_to_yaml(src: &str, dest: Option<&str>) -> Result<(), MyError> {
    let json_str = fs::read_to_string(src)?;

    let json_value: Value = serde_json::from_str(json_str.as_ref())?;

    //let res= serde_yaml::to_string(&json_value)?;
    if dest.is_none() {
        serde_yaml::to_writer(std::io::stdout(), &json_value)?;
    } else {
        let f = File::create(dest.unwrap())?;
        serde_yaml::to_writer(&f, &json_value)?;
    }

    Ok(())
}

fn yaml_to_json(src: &str, dest: Option<&str>) -> Result<(), MyError> {
    let yaml_str = fs::read_to_string(src)?;

    let yaml_value: Value = serde_yaml::from_str(yaml_str.as_ref())?;

    //let res= serde_yaml::to_string(&json_value)?;
    if dest.is_none() {
        serde_json::to_writer(std::io::stdout(), &yaml_value)?;
    } else {
        let f = File::create(dest.unwrap())?;
        serde_json::to_writer(&f, &yaml_value)?;
    }

    Ok(())
}

fn do_main() -> Result<(), MyError> {
    let matches = App::new("json2yaml")
        .version("0.1")
        .author("Luc Dew")
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .help("json source file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dest")
                .short("d")
                .long("dest")
                .help("destination file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("invert")
                .short("i")
                .long("invert")
                .help("invert so performs yaml to json")
                .takes_value(false),
        )
        .get_matches();

    let src_file = matches
        .value_of("source")
        .ok_or(MyError::new("Missing source file"))?;
    let dest_file_path = matches.value_of("dest");

    if matches.is_present("invert") {
        yaml_to_json(src_file, dest_file_path)?;
    } else {
        json_to_yaml(src_file, dest_file_path)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = do_main() {
        eprintln!("error: {}", err.details);
        exit(1);
    }
}
