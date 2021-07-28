#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

#[macro_use]
extern crate clap;

use std::convert::TryInto;
use std::error::Error;

enum OutputFormat {
    Seconds,
    Millis,
    Nanos,
    Rfc2822,
    Rfc3339,
}

fn main() {
    // Parse command line options
    let matches = parse_cmdline();

    // Try to parse the input
    let timestamp = match parse_input(&matches) {
        Ok(ts) => ts,
        Err(e) => {
            eprintln!("Could not parse input: {}", e);
            std::process::exit(1)
        }
    };
    // Determine the output format
    let format = determine_output_format(&matches);
    // Convert timestamp to output format
    let time_str = convert_to_output(&timestamp, format);

    // Print the timestamp
    println!("{}", time_str);
}

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    clap::App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(clap::Arg::with_name("secs").long("secs").short("s"))
        .arg(clap::Arg::with_name("millis").long("millis").short("m"))
        .arg(clap::Arg::with_name("nanos").long("nanos").short("n"))
        .arg(
            clap::Arg::with_name("rfc3339")
                .long("rfc3339")
                .help("Uses RFC 3339 as output format. Example: '2021-07-28T18:30:05.12+00:00'"),
        )
        .arg(
            clap::Arg::with_name("rfc2282")
                .long("rfc2822")
                .help("Uses RFC 2822 as output format. Example: 'Wed, 28 Jul 2021 18:30:05 +0000'"),
        )
        .arg(
            clap::Arg::with_name("from-secs")
                .long("from-secs")
                .visible_alias("from")
                .default_value("now")
                .takes_value(true)
                .help("Provide an input in seconds since epoch. There is a special case 'now' which takes the current "),
        )
        .arg(
            clap::Arg::with_name("from-millis")
                .long("from-millis")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("from-nanos")
                .long("from-nanos")
                .takes_value(true),
        )
        .group(
            clap::ArgGroup::with_name("output")
                .arg("secs")
                .arg("millis")
                .arg("nanos")
                .arg("rfc2282")
                .arg("rfc3339")
                .required(true),
        )
        .group(
            clap::ArgGroup::with_name("input")
                .arg("from-secs")
                .arg("from-millis")
                .arg("from-nanos")
                .required(true),
        )
        .get_matches()
}

fn parse_input(
    matches: &clap::ArgMatches<'_>,
) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn Error>> {
    use chrono::TimeZone;

    if let Some(unix_timestamp) = matches.value_of("from-secs") {
        if unix_timestamp == "now" {
            Ok(chrono::Utc::now())
        } else {
            let timestamp: i64 = unix_timestamp.parse()?;
            Ok(chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(timestamp, 0)))
        }
    } else if let Some(millis_timestamp) = matches.value_of("from-millis") {
        let timestamp: i64 = millis_timestamp.parse()?;
        Ok(
            chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(
                timestamp / 1000,
                ((timestamp % 1000) * 1_000_000).try_into()?,
            )),
        )
    } else if let Some(nanos_timestamp) = matches.value_of("from-nanos") {
        let timestamp: i64 = nanos_timestamp.parse()?;
        Ok(
            chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(
                timestamp / 1_000_000_000,
                (timestamp % 1_000_000_000).try_into()?,
            )),
        )
    } else {
        unreachable!("Checked by clap-group")
    }
}

fn determine_output_format(matches: &clap::ArgMatches<'_>) -> OutputFormat {
    if matches.is_present("millis") {
        OutputFormat::Millis
    } else if matches.is_present("nanos") {
        OutputFormat::Nanos
    } else if matches.is_present("rfc2282") {
        OutputFormat::Rfc2822
    } else if matches.is_present("rfc3339") {
        OutputFormat::Rfc3339
    } else {
        OutputFormat::Seconds
    }
}

fn convert_to_output(timestamp: &chrono::DateTime<chrono::Utc>, format: OutputFormat) -> String {
    match format {
        OutputFormat::Seconds => timestamp.timestamp().to_string(),
        OutputFormat::Millis => timestamp.timestamp_millis().to_string(),
        OutputFormat::Nanos => timestamp.timestamp_nanos().to_string(),
        OutputFormat::Rfc2822 => timestamp.to_rfc2822(),
        OutputFormat::Rfc3339 => timestamp.to_rfc3339(),
    }
}

#[cfg(test)]
mod test {
    use super::OutputFormat;

    #[test]
    fn convert_to_output() {
        use chrono::TimeZone;

        let ts = chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(
            1627497005, 123456789,
        ));

        let secs = super::convert_to_output(&ts, OutputFormat::Seconds);
        assert_eq!("1627497005", secs);
        let millis = super::convert_to_output(&ts, OutputFormat::Millis);
        assert_eq!("1627497005123", millis);
        let nanos = super::convert_to_output(&ts, OutputFormat::Nanos);
        assert_eq!("1627497005123456789", nanos);
        let rfc2822 = super::convert_to_output(&ts, OutputFormat::Rfc2822);
        assert_eq!("Wed, 28 Jul 2021 18:30:05 +0000", rfc2822);
        let rfc3339 = super::convert_to_output(&ts, OutputFormat::Rfc3339);
        assert_eq!("2021-07-28T18:30:05.123456789+00:00", rfc3339);
    }
}
