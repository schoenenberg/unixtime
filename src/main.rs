#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

#[macro_use]
extern crate clap;

use std::num::ParseIntError;

/// The possible output formats
#[derive(Debug, PartialOrd, PartialEq)]
enum OutputFormat {
    Seconds,
    Millis,
    Nanos,
    Rfc2822,
    Rfc3339,
}

fn main() {
    // Parse command line options
    let matches = cmdline_options().get_matches();

    // Try to parse the input
    let timestamp = match parse_input(&matches) {
        Ok(ts) => ts,
        Err(e) => {
            eprintln!("Could not parse input: {}", e);
            std::process::exit(1)
        }
    };
    // Determine the output format
    let format = parse_output_format(&matches);
    // Convert timestamp to output format
    let time_str = convert_to_output(&timestamp, format);

    // Print the timestamp
    println!("{}", time_str);
}

/// Setup of the command-line options
fn cmdline_options<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .long_about("unixtime is a small utility to work with the unix time, also known as \
        posix time or epoch time. It is the number of seconds elapsed since January 1, 1970 00:00:00 \
        UTC.\n\n\
        This utility provides the functionality to display the current unix time in different \
        formats and is able to reverse a unix timestamp into a human readable format.")
        .version(crate_version!())
        .after_help("EXAMPLES:\n    \
        # Current unix time in milliseconds\n    \
        unixtime --millis\n\n    \
        # Reverse a UNIX-timestamp into RFC 3339\n    \
        unixtime --from secs 1627497005 --rfc3339\n\n    \
        # Reverse a UNIX-timestamp in nanoseconds into RFC 2822\n    \
        unixtime --from secs 1627497005 --rfc2822\n\n    \
        # Short form of previous example\n    \
        unixtime -fn 1627497005123456789 --rfc2822")
        .arg(
            clap::Arg::with_name("millis")
                .long("millis")
                .short("m")
                .help("Unix-time in milliseconds as output format"),
        )
        .arg(
            clap::Arg::with_name("nanos")
                .long("nanos")
                .short("n")
                .help("Unix-time in nanoseconds as output format"),
        )
        .arg(
            clap::Arg::with_name("rfc3339")
                .long("rfc3339")
                .help("RFC 3339 as output format. Example: '2021-07-28T18:30:05.12+00:00'"),
        )
        .arg(
            clap::Arg::with_name("rfc2282")
                .long("rfc2822")
                .help("RFC 2822 as output format. Example: 'Wed, 28 Jul 2021 18:30:05 +0000'"),
        )
        .arg(
            clap::Arg::with_name("input")
                .help("The input value. Required if '--from' is not set to 'now'.")
                .allow_hyphen_values(false)
                .required_ifs(&[
                    ("from", "secs"),
                    ("from", "millis"),
                    ("from", "nanos"),
                    ("from", "s"),
                    ("from", "m"),
                    ("from", "n"),
                ]),
        )
        .arg(
            clap::Arg::with_name("from")
                .long("from")
                .short("f")
                .possible_values(&["now", "secs", "millis", "nanos", "s", "m", "n"])
                .hide_possible_values(true)
                .default_value("now")
                .hide_default_value(true)
                .takes_value(true)
                .required(true)
                .next_line_help(true)
                .long_help("Specifies the input format. Valid values are:\n\
                - now: The current time is used. No <input> necessary. [Default]\n\
                - secs, s: <input> is parsed in seconds since January 1st, 1970.\n\
                - millis, m: <input> is parsed in milliseconds since January 1st, 1970.\n\
                - nanos, n: <input> is parsed in nanoseconds since January 1st, 1970."),
        )
        .group(
            clap::ArgGroup::with_name("output")
                .arg("millis")
                .arg("nanos")
                .arg("rfc2282")
                .arg("rfc3339"),
        )
}

/// Parses the input value and returns a timestamp.
fn parse_input(
    matches: &clap::ArgMatches<'_>,
) -> Result<chrono::DateTime<chrono::Utc>, ParseIntError> {
    use chrono::TimeZone;

    match matches.value_of("from").expect("Checked by clap") {
        "now" => Ok(chrono::Utc::now()),
        "secs" | "s" => {
            let input = matches.value_of("input").expect("Enforced by clap");
            let timestamp: i64 = input.parse()?;
            Ok(chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(timestamp, 0)))
        }
        "millis" | "m" => {
            let input = matches.value_of("input").expect("Enforced by clap");
            let timestamp: i64 = input.parse()?;
            Ok(
                chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(
                    timestamp / 1000,
                    ((timestamp % 1000) * 1_000_000) as u32,
                )),
            )
        }
        "nanos" | "n" => {
            let input = matches.value_of("input").expect("Enforced by clap");
            let timestamp: i64 = input.parse()?;
            Ok(
                chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(
                    timestamp / 1_000_000_000,
                    (timestamp % 1_000_000_000) as u32,
                )),
            )
        }
        &_ => {
            unreachable!("Enforced by clap")
        }
    }
}

/// Determines the output format
fn parse_output_format(matches: &clap::ArgMatches<'_>) -> OutputFormat {
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

/// Converts a timestamp into the requested output format
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

    #[test]
    fn parse_output_format_secs() {
        let app = super::cmdline_options();
        let matches = app.get_matches_from(vec![clap::crate_name!()]);
        assert_eq!(
            super::OutputFormat::Seconds,
            super::parse_output_format(&matches)
        );
    }

    #[test]
    fn parse_output_format_millis() {
        let app = super::cmdline_options();
        let matches = app.get_matches_from(vec![clap::crate_name!(), "--millis"]);
        assert_eq!(
            super::OutputFormat::Millis,
            super::parse_output_format(&matches)
        );
    }

    #[test]
    fn parse_output_format_nanos() {
        let app = super::cmdline_options();
        let matches = app.get_matches_from(vec![clap::crate_name!(), "--nanos"]);
        assert_eq!(
            super::OutputFormat::Nanos,
            super::parse_output_format(&matches)
        );
    }

    #[test]
    fn parse_output_format_rfc2822() {
        let app = super::cmdline_options();
        let matches = app.get_matches_from(vec![clap::crate_name!(), "--rfc2822"]);
        assert_eq!(
            super::OutputFormat::Rfc2822,
            super::parse_output_format(&matches)
        );
    }

    #[test]
    fn parse_output_format_rfc3339() {
        let app = super::cmdline_options();
        let matches = app.get_matches_from(vec![clap::crate_name!(), "--rfc3339"]);
        assert_eq!(
            super::OutputFormat::Rfc3339,
            super::parse_output_format(&matches)
        );
    }
}
