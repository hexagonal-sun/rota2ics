use anyhow::{Context, Result, anyhow};
use chrono::NaiveTime;
use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{
        alpha1, alphanumeric1, line_ending, multispace0, space0, space1, u32 as parse_u32,
    },
    combinator::{map, map_res, opt, recognize},
    error::ParseError,
    multi::{count, many1},
    sequence::{delimited, terminated},
};
use std::{collections::HashMap, time::Duration};

#[derive(Clone)]
pub struct Shift {
    pub name: String,
    pub start_time: NaiveTime,
    pub length: Duration,
}

enum ParsedShiftDay {
    NotWorking,
    OnShift(String),
}

pub enum ShiftDay {
    NotWorking,
    OnShift(Shift),
}

pub type ShiftPattern = Vec<Vec<ShiftDay>>;

fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(space0, inner, space0)
}

fn parse_start_time(input: &str) -> IResult<&str, NaiveTime> {
    map_res(
        (parse_u32, tag(":"), parse_u32, tag(":"), parse_u32),
        |(h, _, m, _, s)| {
            NaiveTime::from_hms_opt(h, m, s).ok_or(nom::error::Error::new(
                "Invalid hour. minute or second digit",
                nom::error::ErrorKind::Verify,
            ))
        },
    )
    .parse(input)
}

fn parse_duration(input: &str) -> IResult<&str, Duration> {
    map(
        ((parse_u32, tag("h")), opt((parse_u32, tag("m")))),
        |((h, _), m)| {
            let mut dur = Duration::from_secs(h as u64 * 60 * 60);

            if let Some((m, _)) = m {
                dur += Duration::from_secs(m as u64 * 60);
            }

            dur
        },
    )
    .parse(input)
}

fn parse_name(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            tag("\""),
            recognize(many1(alt((alphanumeric1, space1)))),
            tag("\""),
        ),
        |x: &str| x.to_string(),
    )
    .parse(input)
}

fn parse_shift_definition(input: &str) -> IResult<&str, (String, Shift)> {
    map(
        (
            alpha1,
            ws(parse_start_time),
            ws(parse_duration),
            ws(parse_name),
        ),
        |(ident, start_time, length, name)| {
            (
                ident.to_string(),
                Shift {
                    name,
                    start_time,
                    length,
                },
            )
        },
    )
    .parse(input)
}

fn parse_day(input: &str) -> IResult<&str, ParsedShiftDay> {
    alt((
        map(tag("_"), |_| ParsedShiftDay::NotWorking),
        map(alpha1, |x: &str| ParsedShiftDay::OnShift(x.to_string())),
    ))
    .parse(input)
}

fn parse_week(input: &str) -> IResult<&str, Vec<ParsedShiftDay>> {
    count(ws(parse_day), 7).parse(input)
}

pub fn parse_spd(input: &str) -> Result<ShiftPattern> {
    let (_, (def, _, pat)) = (
        many1(delimited(multispace0, parse_shift_definition, multispace0)),
        delimited(multispace0, tag_no_case("pattern"), multispace0),
        many1(terminated(parse_week, line_ending)),
    )
        .parse(input)
        .map_err(|e| e.to_owned())
        .finish()
        .context("Failed to parse shift pattern definition file")?;

    let mut lut = HashMap::new();

    def.into_iter().for_each(|(k, v)| {
        lut.insert(k, v);
    });

    let mut ret: ShiftPattern = Vec::new();

    for week in pat.into_iter() {
        ret.push(
            week.into_iter()
                .map(|x| -> Result<ShiftDay> {
                    match x {
                        ParsedShiftDay::NotWorking => Ok(ShiftDay::NotWorking),
                        ParsedShiftDay::OnShift(name) => {
                            let shift = lut
                                .get(&name)
                                .ok_or_else(|| anyhow!("{name} is not a defined shift pattern."))?
                                .clone();
                            Ok(ShiftDay::OnShift(shift))
                        }
                    }
                })
                .collect::<Result<Vec<_>>>()?,
        );
    }

    Ok(ret)
}
