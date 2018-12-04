const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day04.txt");

use hashbrown::HashMap;

pub fn day04() -> (u32, u32) {
    let logs = parse_input(RAW_INPUT);

    (part1(&logs), part2(&logs))
}

pub fn part1(entries: &[LogEntry]) -> u32 {
    struct GuardAnalysisResult {
        guard_id: GuardID,
        most_slept_minute: usize,
        total_slept: u32,
    };

    let sleep_schedules = process_entries(entries);

    let best_result = sleep_schedules.iter()
        .map(|(&guard_id, minutes)| {
            let most_slept_minute = minutes.iter()
                .enumerate()
                .max_by_key(|(_, &minute_count)| minute_count)
                .expect("No minutes data for guard").0;

            GuardAnalysisResult {
                guard_id,
                most_slept_minute,
                total_slept: minutes.iter().sum(),
            }
        })
        .max_by_key(|result| result.total_slept)
        .expect("No shifts");

    best_result.guard_id * best_result.most_slept_minute as u32
}

pub fn part2(entries: &[LogEntry]) -> u32 {
    struct GuardAnalysisResult {
        guard_id: GuardID,
        most_slept_minute: usize,
        slept_for_minute: u32,
    };

    let sleep_schedules = process_entries(entries);

    let best_result = sleep_schedules.iter()
        .map(|(&guard_id, minutes)| {
            let (most_slept_minute, &slept_for_minute) = minutes.iter()
                .enumerate()
                .max_by_key(|(_, &minute_count)| minute_count)
                .expect("No minutes data for guard");

            GuardAnalysisResult {
                guard_id,
                most_slept_minute,
                slept_for_minute,
            }
        })
        .max_by_key(|result| result.slept_for_minute)
        .expect("No shifts");

    best_result.guard_id * best_result.most_slept_minute as u32
}

fn process_entries(entries: &[LogEntry]) -> SleepSchedules {
    let mut entries = entries.iter();

    let mut current_guard_id = match entries.next() {
        Some(LogEntry { event: Event::StartsShift(guard_id), .. }) => *guard_id,
        _ => panic!("Invalid first log")
    };
    let mut last_fell_asleep = match entries.next() {
        Some(LogEntry { event: Event::FallsAsleep, date_time }) => date_time.minute,
        _ => panic!("Invalid second log")
    };

    let mut shift_data = HashMap::with_capacity(64);

    while let Some(LogEntry { event, date_time }) = entries.next() {
        match event {
            Event::StartsShift(guard_id) => current_guard_id = *guard_id,
            Event::FallsAsleep => last_fell_asleep = date_time.minute,
            Event::WakesUp => {
                let minutes = shift_data
                    .entry(current_guard_id)
                    .or_insert([0; 60]);

                let minute_span = last_fell_asleep..date_time.minute;

                minute_span.for_each(|minute| minutes[minute as usize] += 1);
            }
        }
    }

    shift_data
}

type SleepSchedules = HashMap<GuardID, SleepPerMinute>;
type GuardID = u32;
type SleepPerMinute = [u32; 60];

pub fn parse_input(input: &[u8]) -> Vec<LogEntry> {
    let mut entries = input.split(|&c| c == b'\n')
        .map(parse_log_entry)
        .collect::<Vec<_>>();

    entries.sort_by(|log1, log2| log1.date_time.cmp(&log2.date_time));

    entries
}

fn parse_log_entry(raw_entry: &[u8]) -> LogEntry {
    use nom::{*, types::CompleteByteSlice as Input};
    use std::str::{FromStr, from_utf8};

    named!(parse_u32<Input, u32>, map!(
        digit,
        |d| FromStr::from_str(from_utf8(&d).unwrap()).unwrap()
    ));

    named!(parse_shift<Input, Event>, delimited!(
        tag!("Guard #"),
        map!(parse_u32, Event::StartsShift),
        tag!(" begins shift")
    ));
    named!(parse_asleep<Input, Event>, map!(
        tag!("falls asleep"), |_| Event::FallsAsleep
    ));
    named!(parse_wakes_up<Input, Event>, map!(
        tag!("wakes up"), |_| Event::WakesUp
    ));

    named!(parse_event<Input, Event>, alt!(parse_shift | parse_asleep | parse_wakes_up));

    named!(parse_date_time<Input, DateTime>, do_parse!(
        year:   parse_u32 >> tag!("-") >>
        month:  parse_u32 >> tag!("-") >>
        day:    parse_u32 >> tag!(" ") >>
        hour:   parse_u32 >> tag!(":") >>
        minute: parse_u32 >>
        (DateTime { year, month, day, hour, minute })
    ));

    named!(parse_entry<Input, LogEntry>, do_parse!(
        tag!("[") >>
        date_time: parse_date_time >>
        tag!("] ") >>
        event: parse_event >>
        (LogEntry { date_time, event })
    ));

    match parse_entry(Input(raw_entry)) {
        Ok((_remaining, parsed)) => parsed,
        _ => panic!("Bad entry format: {:?}", raw_entry)
    }
}

#[derive(Debug)]
pub struct LogEntry {
    date_time: DateTime,
    event: Event,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
pub enum Event {
    StartsShift(GuardID),
    FallsAsleep,
    WakesUp,
}
