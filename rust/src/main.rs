use std::io;
use time::{
    macros::format_description,
    format_description::FormatItem,
    PrimitiveDateTime,
    Date,
    Time,
};

const MAX_HEIGHT: u32 = 10000;
const WIN_CAPACITY: usize = 120;
const FMT_LINE: &'static [FormatItem<'static>] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
const FMT_DATE: &'static [FormatItem<'static>] = format_description!("[year]-[month]-[day]");
//const FMT_TIME: &'static [FormatItem<'static>] = format_description!("[hour]:[minute]:[second]");

#[derive(Debug,Eq)]
struct WxVal {
    time: Time,
    height: u32,
}

impl PartialOrd for WxVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WxVal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq for WxVal {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.height == other.height
    }
}

struct WxDay {
    date: Date,
    heights: [u32; 24],
}

struct WxWin {
    start: Time,
    vals: Vec<WxVal>,
}

/* Meters to feet conversion */
fn mtof(m: u32) -> u32 {
    (m as f64 * 3.2808399) as u32
}

fn parse_line(line: &str) -> (PrimitiveDateTime, u32) {
    let v: Vec<&str> = line.split_whitespace().collect();

    (
        PrimitiveDateTime::parse(&line[..19], FMT_LINE).unwrap(),
        v[4].parse::<u32>().unwrap_or(MAX_HEIGHT),
    )
}

fn add_win(day: &mut WxDay, win: &mut WxWin) {
    win.vals.sort();
    let median = win.vals.get(win.vals.len() / 2).unwrap();
    day.heights[win.start.hour() as usize] = median.height;
}

fn main() {
    let mut cur_day: Option<WxDay> = None;
    let mut days: Vec<WxDay> = Vec::with_capacity(365);
    let mut cur_win: Option<WxWin> = None;

    loop {
        let mut line = String::with_capacity(64);
        let mut win: WxWin;

        match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let (time, height) = parse_line(&line as &str);

                let new_day = cur_day.is_none() || cur_day.as_ref().unwrap().date != time.date();
                let new_hour = cur_win.is_none() || cur_win.as_ref().unwrap().start.hour() != time.time().hour();

                if new_day || new_hour {
                    if cur_day.is_some() {
                        let mut day = cur_day.unwrap();
                        add_win(&mut day, &mut cur_win.unwrap());
                        cur_day = Some(day);
                    }

                    if new_day {
                        if cur_day.is_some() {
                            days.push(cur_day.unwrap());
                        }

                        cur_day = Some(WxDay {
                            date: time.date(),
                            heights: [MAX_HEIGHT; 24]
                        });
                    }

                    win = WxWin {
                        start: time.time(),
                        vals: Vec::with_capacity(WIN_CAPACITY),
                    };
                } else {
                    win = cur_win.unwrap();
                }

                win.vals.push(WxVal { time: time.time(), height });
                cur_win = Some(win);
            }

            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
    }

    if cur_day.is_some() {
        days.push(cur_day.unwrap());
    }

    let (mut num_days, mut below_2000, mut below_1000, mut below_500, mut below_400, mut below_300, mut below_200) = (0, 0, 0, 0, 0, 0, 0);

    for day in days {
        num_days += 1;

        let min = mtof(*day.heights.iter().min().unwrap());
        if min < 2000 {
            below_2000 += 1;
            if min < 1000 {
                below_1000 += 1;
                if min < 500 {
                    below_500 += 1;
                    if min < 400 {
                        below_400 += 1;
                        if min < 300 {
                            below_300 += 1;
                            if min < 200 {
                                below_200 += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("{} {}", day.date.format(FMT_DATE).unwrap(), min);
    }

    println!("\n{} days", num_days);
    println!("< 2000: {}, < 1000: {}, < 500: {}, < 400: {}, < 300: {}, < 200: {}",
        below_2000, below_1000, below_500, below_400, below_300, below_200);
}

#[test]
fn test_measurement() {
    let str = "2020-12-30 16:11:25,230 cloudLowSig 1609344685.231 330";
    let (time, height) = parse_line(str);
    assert_eq!(height, 330);
    assert_eq!(time, PrimitiveDateTime::parse("2020-12-30 16:11:25", FMT_LINE).unwrap());

    let str = "2020-12-31 10:58:35,241 cloudLowSig 1609412315.242 None";
    let (time, height) = parse_line(str);
    assert_eq!(height, 10000);
    assert_eq!(time, PrimitiveDateTime::parse("2020-12-31 10:58:35", FMT_LINE).unwrap());
}
