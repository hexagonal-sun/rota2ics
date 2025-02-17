# rota2ics

`rota2ics` is a command-line tool designed to convert a shift pattern (or rota)
into a series of calendar events in the iCalendar (.ics) format. It allows users
to define shift patterns, specify the number of weeks to generate, and create
calendar events to help plan around work schedules, making it easy to visualize
shifts in any calendar application.

## Motivation

This tool was developed out of personal need. My wife, receives a new rota for
her work every 6 months. Each time that happens, she spends a lot of time
entering her new rota into her calendar. This tool allows her to simply convert
the rota into a shift pattern definition file, and generate multiple weeks of
calendar events automatically, saving her time and effort.

## How it works

### Shift Pattern Definition File (SPD)

The core of `rota2ics` is the Shift Pattern Definition (SPD) file. This file
contains a list of shifts and the pattern for each day of the week. A shift
pattern consists of:

- A shift code (e.g., `D`, `LD`, `NS`).
- A start time and duration for each shift.
- A description of the shift.

An example SPD file:

```
D 08:30:00 8h15m "Standard Day"
LD 08:30:00 12h "Long Day"
NS 20:30:00 11h "Night Shift"

pattern

_ _  D  D  D  _  _
_ _  LD LD LD LD _
_ NS NS NS _  _  _
_ _  _  D  D  D  D

```

In this example:

- `D` represents a standard day shift starting at 08:30 AM for 8 hours 15
  minutes.
- `LD` represents a long day shift starting at 08:30 AM for 12 hours.
- `NS` represents a night shift starting at 8:30 PM for 11 hours.

The pattern section defines which shifts occur on each day of the week, starting
from the leftmost position which represents Monday for that week. The
underscores (`_`) represent off-days, where no shift occurs.

### Generating Calendar Events

Once the SPD file is ready, `rota2ics` can be run to generate the corresponding
calendar events. The program will convert the shift patterns into calendar
entries for the specified number of weeks.

### Arguments

`rota2ics` accepts the following arguments:

```
Usage: rota2ics [OPTIONS] <SPD_FILE> <WEEKS_TO_GEN> <OUT_PATH>

Arguments:
  <SPD_FILE>
          shift pattern definition file
  <WEEKS_TO_GEN>
          The number of weeks events should be generated over
  <OUT_PATH>
          The path to where the output should be written
Options:
  -s, --start-date <START_DATE>
          The date where the event genereation should start
  -i, --spd-start-index <SPD_START_INDEX>
          The index (0-based) into the spd pattern list for the given `start_date`
  -h, --help
          Print help (see a summary with '-h')
```


### Example

Given the SPD file above, to generate 4 weeks of calendar events starting on
February 17, 2025, and output them to a file called `rota.ics`, you would run:


```
rota2ics shift_pattern.spd 4 rota.ics --start-date 2025-02-17
```

This command will create a `.ics` file with the shift schedule for the next 4
weeks.

## Contribution

If you'd like to contribute to `rota2ics`, feel free to fork the repository,
submit issues, or open pull requests. All contributions are welcome!
