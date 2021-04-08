#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap::App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(clap::Arg::with_name("millis").long("millis").short("m"))
        .arg(clap::Arg::with_name("nanos").long("nanos").short("n"))
        .group(clap::ArgGroup::with_name("exclusions").arg("millis").arg("nanos"))
        .get_matches();

    let timestamp = chrono::Utc::now();
    let time_str = if matches.is_present("millis") {
        timestamp.timestamp_millis()
    } else if matches.is_present("nanos") {
        timestamp.timestamp_nanos()
    } else {
        timestamp.timestamp()
    };

    println!("{}", time_str);
}
