use std::{
    error::Error,
    io::{self, Read},
};

use super::super::{BaseArgs, Runnable};
use clap::{Parser, ValueEnum};

use chrono::{Date, DateTime, Local, NaiveDate, TimeZone, Utc, NaiveDateTime};
use chrono_tz::{Tz, UTC};
use indoc::indoc;
use regex::Regex;

#[derive(Parser, Debug)]
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

    #[arg(short = 'z', long = "oz", long_help = "Timezone of the output")]
    output_timezone: Option<String>,

    #[arg(short, long, default_value = "iso", long_help = "Format of the output")]
    format: Format,

    #[arg(
        short,
        long,
        conflicts_with = "format",
        long_help = "Custom template string for output"
    )]
    template_string: Option<String>,
}

#[derive(ValueEnum, Debug, Clone)]
enum Format {
    Iso,
    Unix,
    Date,
    Time,
    Datetime,
}

impl TimeStampArgs {}

impl Runnable for TimeStampArgs {
    fn run(&self, _: &BaseArgs) -> Result<String, Box<dyn Error>> {
        let mut ts = Utc::now();
        if !self.now {
            let mut message = String::new();
            let _ = io::stdin().read_to_string(&mut message);
            message = message.trim().to_string();

            let pattern = r"^[0-9]+$";
            let regex = Regex::new(pattern)?;
            if regex.is_match(&message) {
                let secs = message.parse::<i64>()?;
                ts = Utc.timestamp_opt(secs, 0).unwrap();
            } else {
                if self.input_timezone.is_some() {
                    let naive_date = message.parse::<NaiveDateTime>()?;
                    let parsed_input_timezone = self.input_timezone.clone().unwrap().parse::<Tz>().unwrap();
                    ts = parsed_input_timezone.from_local_datetime(&naive_date).unwrap().with_timezone(&Utc);
                } else {
                    ts = message.parse::<DateTime<Utc>>()?;
                }
            }
        }

        return Ok(ts.with_timezone(&Local).to_string());
    }
}
