#![allow(dead_code)]

use std::env;

mod day1;
mod day2;
mod day3;

#[rustfmt::skip]
fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        panic!("too many args!\n just type the day's number")
    }
    _ = args.next();
    match args.next() {
        Some(d) => match d.as_str() {
            "1" => day1::start(),
            "2" => day2::start(),
            "3" => day3::start(),
            _ => unimplemented!("no more days, baddy")
        },
        None => panic!("a day must be specified!"),
    }
}
