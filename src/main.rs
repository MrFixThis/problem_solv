#![allow(dead_code)]

use std::env;

mod day1;
mod day2;

#[rustfmt::skip]
fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        panic!("too many args!\n just type the day's number")
    }
    _ = args.next();
    match args.next() {
        Some(d) => match d.parse::<i32>().unwrap() {
            1 => day1::start(),
            _ => todo!()
        },
        None => panic!("a day must be specified!"),
    }
}
