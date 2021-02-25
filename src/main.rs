extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

fn file_exists(s: &str) -> bool {
    println!("fileExists: {}", s);
    true
}

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

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("sum") {
            let filename = matches.value_of("FILENAME").unwrap();
            if file_exists(filename) {
                // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
                // required we could have used an 'if let' to conditionally get the value)
                println!("Calculating shasums of {}", filename);
            }
    }
    if let Some(matches) = matches.subcommand_matches("time") {
            let filename = matches.value_of("FILENAME").unwrap();
            if file_exists(filename) {
                // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
                // required we could have used an 'if let' to conditionally get the value)
                println!("Getting times of {}", filename);
            }
    }


}
