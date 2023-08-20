use std::error::Error;

use super::super::{BaseArgs, Runnable};
use clap::{Parser, ValueEnum};

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use indoc::indoc;
use regex::Regex;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "the timestamper",
    about = "Parse and convert timestamps",
    after_help = "See `medea help timestamp` for details",
    long_about = indoc!{"
        Read a timestamp and convert it to the desired format.
        Both string and epoch (unix) timestamps are supported as input, and the type will be parsed automatically.\
        Omit the input to use the current time.
    "},
    after_long_help = indoc!{r#"
        Examples:

            $ medea ts -n --format=8601

    "#}
)]
pub struct TimeStampArgs {

    #[arg(
        long_help = "Input timestamp. Accepts unix (epoch) or iso8601 format. \
            If omitted, will default to now.",
        // required = false
    )]
    input: Option<String>,

    #[arg(short = 'z', long, long_help = "Timezone of the output")]
    timezone: Option<String>,

    #[arg(
        short,
        long,
        default_value = "iso",
        long_help = "Format for output",
    )]
    format: Format,
}

#[derive(ValueEnum, Debug, Clone)]
enum Format {
    Iso,
    Unix,
}

#[derive(Debug)]
enum TimestampError {
    ParseError(chrono::format::ParseError),
    InvalidTimeZoneError(String),
}
impl From<chrono::format::ParseError> for TimestampError {
    fn from(err: chrono::format::ParseError) -> Self {
        TimestampError::ParseError(err)
    }
}
impl std::fmt::Display for TimestampError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimestampError::ParseError(e) => {
                write!(f, "could not parse input as a naive datetime: {}", e)
            }
            TimestampError::InvalidTimeZoneError(s) => {
                write!(f, "Invalid Timezone: {}", s)
            }
        }
    }
}
impl Error for TimestampError {}

impl TimeStampArgs {
    const NUMERIC_TIMESTAMP_PATTERN: &str = r"^[0-9]+$";

    fn inner_run(
        &self,
        _: &BaseArgs,
        _: impl Fn() -> String,
    ) -> Result<String, Box<dyn Error>> {
        let ts = match &self.input {
            Some(input_string) => {
                let regex = Regex::new(Self::NUMERIC_TIMESTAMP_PATTERN)?;
                if regex.is_match(&input_string) {
                    let secs = input_string.parse::<i64>()?;
                    Utc.timestamp_opt(secs, 0).unwrap()
                } else {
                    DateTime::parse_from_str(&input_string.as_str(), "%+")
                        .map_err(|e| TimestampError::ParseError(e))?.with_timezone(&Utc)
                }

            },
            None => Utc::now()
        };

        let format_str = match self.format {
            Format::Unix => "%s",
            Format::Iso => "%+",
        };

        let output = match &self.timezone {
            Some(t) =>  {
                ts
                .with_timezone(&t.parse::<Tz>().map_err(|e| TimestampError::InvalidTimeZoneError(e))?)
                .format(format_str)
                .to_string()
            },
            None => ts.format(format_str).to_string(),
        };

        return Ok(output);
    }
}

impl Runnable for TimeStampArgs {
    fn run(
        &self,
        base_args: &BaseArgs,
        get_input: impl Fn() -> String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.inner_run(base_args, get_input)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::cli::{
        args::{BaseArgs, Runnable},
        ArgsEnum,
    };

    use super::TimeStampArgs;

    fn base_args(tsa: TimeStampArgs) -> BaseArgs {
        BaseArgs {
            colors: false,
            trim: false,
            command: ArgsEnum::Timestamp(tsa),
        }
    }

    fn spoof_input(input: String) -> Box<dyn Fn() -> String> {
        return Box::new(move || -> String { return input.clone() });
    }

    fn run(sut: TimeStampArgs) -> Result<String, Box<dyn Error>> {
        Ok(sut.run(&base_args(sut.clone()), spoof_input(String::new()))?)
    }

    #[test]
    fn will_generate_timestamp() -> Result<(), Box<dyn Error>> {
        let sut = TimeStampArgs {
            timezone: None,
            format: super::Format::Iso,
            input: None,
        };

        let ts = run(sut)?;
        assert!(!ts.is_empty());
        Ok(())
    }

    #[test]
    fn will_convert_from_unix_time() -> Result<(), Box<dyn Error>> {
        let sut = TimeStampArgs {
            timezone: None,
            format: super::Format::Iso,
            input: Some(String::from("1234567890")),
        };

        let ts = run(sut)?;
        assert_eq!(ts, "2009-02-13T23:31:30+00:00");
        Ok(())
    }


}
