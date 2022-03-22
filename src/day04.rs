use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Display, FromStr, Eq, PartialEq, Copy, Clone, Debug)]
#[display("[1518-{month}-{day} {hour}:{minute}]")]
pub struct Date {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl PartialOrd<Self> for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.month < other.month {
            return Ordering::Less;
        }

        if self.month == other.month {
            if self.day < other.day {
                return Ordering::Less;
            }

            if self.day == other.day {
                if self.hour < other.hour {
                    return Ordering::Less;
                }

                if self.hour == other.hour {
                    return self.minute.cmp(&other.minute);
                }
            }
        }

        Ordering::Greater
    }
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
pub enum Event {
    #[display("{date} Guard #{guard_id} begins shift")]
    ShiftBegin { date: Date, guard_id: u32 },

    #[display("{date} falls asleep")]
    FallsAsleep { date: Date },

    #[display("{date} wakes up")]
    WakesUp { date: Date },
}

#[aoc_generator(day04)]
pub fn generate(inp: &str) -> Vec<Event> {
    let mut parsed_data = inp.lines().filter_map(|it| it.parse().ok()).collect_vec();
    parsed_data.sort_by_key(|it| match it {
        Event::ShiftBegin { date, .. } | Event::FallsAsleep { date } | Event::WakesUp { date } => {
            *date
        }
    });

    parsed_data
}

fn build_guard_schedule(events: &[Event]) -> (u32, Vec<(u32, u32)>) {
    let mut cur_guard = None;
    if let Event::ShiftBegin { guard_id, .. } = events[0] {
        cur_guard = Some(guard_id);
    }

    let guard_events = events
        .iter()
        .skip(1)
        .peekable()
        .peeking_take_while(|it| !matches!(it, Event::ShiftBegin { .. }))
        .collect_vec();

    let mut sleep_times = vec![];
    if !guard_events.is_empty() {
        // Even number of sleep/wake
        assert_eq!(guard_events.len() % 2, 0);
        let mut idx = 0;
        while idx < guard_events.len() {
            let mut sleep_from = None;
            if let Event::FallsAsleep { date } = guard_events[idx] {
                sleep_from = Some(date.minute);
            }

            let mut sleep_until = None;
            if let Event::WakesUp { date } = guard_events[idx + 1] {
                sleep_until = Some(date.minute);
            }

            assert!(sleep_from.is_some() && sleep_until.is_some());
            idx += 2;

            sleep_times.push((sleep_from.unwrap(), sleep_until.unwrap()));
        }
    }

    assert!(cur_guard.is_some());
    (cur_guard.unwrap(), sleep_times)
}

fn get_guard_schedule(events: &[Event]) -> HashMap<u32, Vec<usize>> {
    let mut schedule = HashMap::new();

    let mut events = events.to_vec();

    while !events.is_empty() {
        let (id, sleep_times) = build_guard_schedule(&events);

        events = events
            .iter()
            .dropping(sleep_times.len() * 2 + 1)
            .copied()
            .collect_vec();

        for (from, until) in sleep_times {
            let times = schedule.entry(id).or_insert_with(|| vec![0usize; 60]);

            for minute in from..until {
                times[minute as usize] += 1;
            }
        }
    }

    schedule
}

#[aoc(day04, part1)]
pub fn part1(events: &[Event]) -> Option<usize> {
    let schedule = get_guard_schedule(events);

    let (guard_id, _) = schedule
        .iter()
        .map(|(k, v)| (k, v.iter().sum::<usize>()))
        .max_by_key(|(_, time)| *time)?;

    let (_, sleep_times) = schedule.iter().find(|&(k, _)| k == guard_id)?;
    let (minute_most_asleep, _) = sleep_times
        .iter()
        .enumerate()
        .max_by_key(|(_, value)| **value)?;

    Some(*guard_id as usize * minute_most_asleep)
}

#[aoc(day04, part2)]
pub fn part2(events: &[Event]) -> Option<usize> {
    let schedule = get_guard_schedule(events);

    let (guard_id, minute, _) = schedule
        .iter()
        .map(|(id, times)| {
            let (max_minute, times_slept) = times
                .iter()
                .enumerate()
                .max_by_key(|(_, number_asleep)| **number_asleep)
                .unwrap();
            (id, max_minute, times_slept)
        })
        .max_by_key(|(_, _, times_slept)| **times_slept)?;

    Some(*guard_id as usize * minute)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift\n\
                              [1518-11-01 00:05] falls asleep\n\
                              [1518-11-01 00:25] wakes up\n\
                              [1518-11-01 00:30] falls asleep\n\
                              [1518-11-01 00:55] wakes up\n\
                              [1518-11-01 23:58] Guard #99 begins shift\n\
                              [1518-11-02 00:40] falls asleep\n\
                              [1518-11-03 00:05] Guard #10 begins shift\n\
                              [1518-11-03 00:24] falls asleep\n\
                              [1518-11-03 00:29] wakes up\n\
                              [1518-11-04 00:02] Guard #99 begins shift\n\
                              [1518-11-02 00:50] wakes up\n\
                              [1518-11-04 00:36] falls asleep\n\
                              [1518-11-04 00:46] wakes up\n\
                              [1518-11-05 00:03] Guard #99 begins shift\n\
                              [1518-11-05 00:45] falls asleep\n\
                              [1518-11-05 00:55] wakes up";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res.unwrap(), 240);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res.unwrap(), 4455);
    }
}
