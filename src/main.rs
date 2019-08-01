/**
 * Quick and dirty json to yaml converter
 */

extern crate clap;

use std::fs;
use std::fs::File;
use std::error::Error;
use std::fmt;
use clap::{Arg, App};
use serde_json::Value;


#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl From<serde_yaml::Error> for MyError {

  fn from(err:serde_yaml::Error) -> Self {
      MyError::new(err.description())
    }

}

impl From<serde_json::Error> for MyError {

  fn from(err:serde_json::Error) -> Self {
      MyError::new(err.description())
    }

}

impl From<std::io::Error> for MyError {

  fn from(err: std::io::Error) -> Self {
      MyError::new(err.description())
    }

}

fn json_to_yaml(src: &str, dest: Option<&str>) -> Result<(),MyError> {

  let json_str = fs::read_to_string(src)?;

  let json_value:Value = serde_json::from_str(json_str.as_ref())?;

  //let res= serde_yaml::to_string(&json_value)?;
  if dest.is_none() {
    serde_yaml::to_writer(std::io::stdout(),&json_value)?;
  } else {
    let f = File::create(dest.unwrap())?;
    serde_yaml::to_writer(&f,&json_value)?;
  }

  Ok(())

}

fn main() -> Result<(),MyError> {
    let matches = App::new("json2yaml")
                      .version("0.1")
                      .author("Luc Dew")
                      .arg(Arg::with_name("source")
                        .short("s")
                        .long("source")
                        .help("json source file")
                        .takes_value(true)
                      )
                      .arg(Arg::with_name("dest")
                        .short("d")
                        .long("dest")
                        .help("destination file")
                        .takes_value(true)
                      )
                      .get_matches();

    let src_json = matches.value_of("source").ok_or(MyError::new("Missing source file"))?;
    let dest_file_path = matches.value_of("dest");

    json_to_yaml(src_json, dest_file_path)?;

    Ok(())
}
