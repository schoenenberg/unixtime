#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]

use chrono::{DateTime, TimeZone, Utc};

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap::App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(clap::Arg::with_name("millis").long("millis").short("m"))
        .arg(clap::Arg::with_name("nanos").long("nanos").short("n"))
        .arg(clap::Arg::with_name("timestamp").index(1))
        .group(
            clap::ArgGroup::with_name("exclusions")
                .arg("millis")
                .arg("nanos"),
        )
        .get_matches();

    if matches.is_present("timestamp") {
        let timestamp: i64 = matches.value_of("timestamp").unwrap().parse().unwrap();
        let dt: DateTime<Utc> = if matches.is_present("millis") {
            Utc.timestamp_millis(timestamp)
        } else if matches.is_present("nanos") {
            Utc.timestamp_nanos(timestamp)
        } else {
            Utc.timestamp(timestamp, 0)
        };

        println!("{}", dt.to_rfc2822());
    } else {
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
}
