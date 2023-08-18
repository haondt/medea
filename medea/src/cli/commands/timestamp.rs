use std::error::Error;

use super::super::{BaseArgs, Runnable};
use clap::{Parser, ValueEnum};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use indoc::indoc;
use regex::Regex;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "the timestamper",
    about = "Parse and convert timestamps",
    after_help = "See `medea help timestamp` for details",
    long_about = indoc!{"
        Read a timestamp from stdin and convert it to the desired format.
        Both string and epoch (unix) timestamps are supported as input, and the type will be parsed automatically.\
    "},
    after_long_help = indoc!{r#"
        Examples:

            $ medea ts -n --format=8601

    "#}
)]
pub struct TimeStampArgs {
    #[arg(
        short,
        long,
        help = "Use the current time as input instead of reading from stdin"
    )]
    now: bool,

    #[arg(
        long = "iz",
        long_help = "Timezone of the input.",
        conflicts_with = "now"
    )]
    input_timezone: Option<String>,

    #[arg(
        long = "it",
        long_help = "Custom template string for parsing input",
        conflicts_with = "now"
    )]
    input_template_string: Option<String>,

    #[arg(short = 'z', long = "oz", long_help = "Timezone of the output")]
    output_timezone: Option<String>,

    #[arg(
        short,
        long,
        conflicts_with = "format",
        long_help = "Custom template string for formatting output"
    )]
    template_string: Option<String>,

    #[arg(
        short,
        long,
        default_value = "iso",
        long_help = "Standard format for output. Can be used instead of `--template-string`"
    )]
    format: Format,
}

#[derive(ValueEnum, Debug, Clone)]
enum Format {
    Iso,
    Unix,
    Date,
    Time,
    Datetime,
}

impl TimeStampArgs {
    const NUMERIC_TIMESTAMP_PATTERN: &str = r"^[0-9]+$";

    fn inner_run(
        &self,
        _: &BaseArgs,
        get_input: impl Fn() -> String,
    ) -> Result<String, Box<dyn Error>> {
        let mut ts = Utc::now();
        if !self.now {
            let message = get_input().trim().to_string();

            let regex = Regex::new(Self::NUMERIC_TIMESTAMP_PATTERN)?;
            if regex.is_match(&message) {
                let secs = message.parse::<i64>()?;
                ts = Utc.timestamp_opt(secs, 0).unwrap();
            } else {
                if self.input_timezone.is_some() {
                    let parsed_input_timezone =
                        self.input_timezone.clone().unwrap().parse::<Tz>()?;
                    ts = match &self.input_template_string {
                        Some(t) => {
                            let offset_ts = parsed_input_timezone
                                .datetime_from_str(message.as_str(), t.as_str())?;
                            offset_ts.with_timezone(&Utc)
                        }
                        None => {
                            let naive_date = message.parse::<NaiveDateTime>()?;
                            parsed_input_timezone
                                .from_local_datetime(&naive_date)
                                .unwrap()
                                .with_timezone(&Utc)
                        }
                    };
                } else {
                    ts = match &self.input_template_string {
                        Some(t) => {
                            let offset_ts = DateTime::parse_from_str(message.as_str(), t.as_str())?;
                            offset_ts.with_timezone(&Utc)
                        }
                        None => message.parse::<DateTime<Utc>>()?,
                    };
                }
            }
        }

        let format_str = match self.format {
            Format::Unix => "%s",
            Format::Iso => "%+",
            Format::Date => "%x",
            Format::Time => "%X",
            Format::Datetime => "%c",
        };

        let output = match &self.output_timezone {
            Some(t) => ts
                .with_timezone(&t.parse::<Tz>()?)
                .format(format_str)
                .to_string(),
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

    fn run_with_input(sut: TimeStampArgs, input: String) -> Result<String, Box<dyn Error>> {
        Ok(sut.run(&base_args(sut.clone()), spoof_input(input))?)
    }
    fn run_without_input(sut: TimeStampArgs) -> Result<String, Box<dyn Error>> {
        Ok(sut.run(&base_args(sut.clone()), spoof_input(String::new()))?)
    }

    #[test]
    fn will_generate_timestamp() -> Result<(), Box<dyn Error>> {
        let sut = TimeStampArgs {
            now: true,
            input_timezone: None,
            output_timezone: None,
            format: super::Format::Iso,
            template_string: None,
            input_template_string: None,
        };

        let ts = run_without_input(sut)?;
        assert!(!ts.is_empty());
        Ok(())
    }

    #[test]
    fn will_convert_from_unix_time() -> Result<(), Box<dyn Error>> {
        let sut = TimeStampArgs {
            now: false,
            input_timezone: None,
            output_timezone: None,
            format: super::Format::Iso,
            template_string: None,
            input_template_string: None,
        };

        let input = String::from("1234567890");

        let ts = run_with_input(sut, input)?;
        assert_eq!(ts, "2009-02-13T23:31:30+00:00");
        Ok(())
    }
}
