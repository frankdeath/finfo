use std::path::Path;
use std::fs;
extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};
//use chrono::NaiveDateTime;

fn main() {
    let matches = App::new("File Info (finfo)")
                          .version("0.1")
                          .about("Would be very helpful if I understood rust syntax")
                          /*.arg(Arg::with_name("v")
                               .short("v")
                               .multiple(false)
                               .help("Be verbose"))*/
                          .setting(AppSettings::SubcommandRequiredElseHelp)
                          .subcommand(SubCommand::with_name("sum")
                                      .about("Calculate sha1 and sha256 checksums")
                                      .arg(Arg::with_name("FILENAME")
                                          .required(true)
                                          .index(1)
                                          .help("Name of file")))
                          .subcommand(SubCommand::with_name("time")
                                      .about("Display creation, modification, and access times")
                                      .arg(Arg::with_name("FILENAME")
                                          .required(true)
                                          .index(1)
                                          .help("Name of file")))
                         .get_matches();

    if let Some(matches) = matches.subcommand_matches("sum") {
            let filename = matches.value_of("FILENAME").unwrap();
            let filepath = Path::new(filename);
            if filepath.exists() {
                println!("Calculating shasums of {:?}", filepath);
            }
    }
    if let Some(matches) = matches.subcommand_matches("time") {
            let filename = matches.value_of("FILENAME").unwrap();
            let filepath = Path::new(filename);
            if filepath.exists() {
                println!("Getting times of {:?}", filepath);
                if let Ok(metadata) = fs::metadata(filename) {
                  println!("Metadata: {:?}", metadata);
                  if let Ok(mtime) = metadata.modified() {
                      //let mtimef = NaiveDateTime::from_timestamp(mtime);
                      //println!("modified {:?}", mtimef);
                      println!("modified {:?}", mtime);
                  } else {
                      println!("Problem getting mtime");
                  }
                } else {
                  println!("Problem getting metadata");
                }
            }
    }


}
