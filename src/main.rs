use anyhow::{Context, Result};
use chrono::{Days, Local, NaiveDate, Weekday};
use clap::Parser;
use icalendar::{Calendar, Component, Event, EventLike};
use spd::parse_spd;
use std::{fs::File, io::{Read, Write}, path::PathBuf};

mod spd;

/// rota2ics - Rota calendar event generator.
///
/// Generate calendar events for a number of weeks given a particular shift
/// pattern.
#[derive(Parser)]
struct Args {
    /// shift pattern definition file.
    spd_file: PathBuf,

    /// The date where the event genereation should start.
    #[clap(short, long, default_value_t = Local::now().date_naive())]
    start_date: NaiveDate,

    /// The index (0-based) into the spd pattern list for the given
    /// `start_date`.
    #[clap(short = 'i', long, default_value_t = 0)]
    spd_start_index: u32,

    /// The number of weeks events should be generated over.
    weeks_to_gen: u32,

    /// The path to where the output should be written.
    out_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut spd = String::new();

    let mut spd_file = File::open(args.spd_file).context("Could not open spd file")?;

    spd_file.read_to_string(&mut spd)?;

    let shift_pattern = parse_spd(&spd)?;

    let mut cal = Calendar::new();
    let mut current_date = args.start_date.week(Weekday::Mon).first_day();

    for i in 0..args.weeks_to_gen {
        let shift_week = &shift_pattern[(i + args.spd_start_index) as usize % shift_pattern.len()];

        for day in 0..7 {
            match &shift_week[day as usize] {
                spd::ShiftDay::NotWorking => {}
                spd::ShiftDay::OnShift(shift) => {
                    let mut evt = Event::new();
                    let start = (current_date + Days::new(day)).and_time(shift.start_time);

                    evt.summary(&shift.name)
                        .starts(start)
                        .ends(start + shift.length);

                    cal.push(evt);
                }
            }
        }

        current_date = current_date + Days::new(7);
    }

    let mut out_file = File::create(args.out_path)
        .context("Could not open output file")?;


    write!(&mut out_file, "{}", cal)
        .context("Could not write to output file")?;

    Ok(())
}
