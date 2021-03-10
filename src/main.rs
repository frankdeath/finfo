use std::path::Path;
use std::fs;
extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};
use chrono::offset::Local;
use chrono::DateTime;

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
            // Get ctime, mtime, and atime
            let _result = get_times(&filename);
    }
}

fn get_times(filename : &str) -> Result<(), std::io::Error> {
    let filepath = Path::new(&filename);
    if filepath.exists() {
        let metadata = fs::metadata(filename)?;
        //println!("Metadata: {:?}", metadata);
        
        println!("Getting times for {:#?}", filepath);
        
        let fmt_str = "%Y-%m-%d %H:%M:%S";
        
        let ctime_sys = metadata.created()?;
        let ctime: DateTime<Local> = ctime_sys.into();
        println!("created:  {}", ctime.format(fmt_str));
        let mtime_sys = metadata.modified()?;
        let mtime: DateTime<Local> = mtime_sys.into();
        println!("modified: {}", mtime.format(fmt_str));
        let atime_sys = metadata.accessed()?;
        let atime: DateTime<Local> = atime_sys.into();
        println!("accessed: {}", atime.format(fmt_str));
    }
    Ok(())
}
