//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ t1_tdd.rs  ι․22․05․20✦08․10․28․ 🌎η  

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;
use chrono_tz::US::Pacific;


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]
mod parse_test {
    // use chrono::{};

    #[test]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }


}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";





///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    print!("{}📚 STARTING: a0_hello.rs \n\n", C_LL);


    Ok(())
    // panic!("for: No Reason");
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}


fn exp_2() -> i32 {
    2
}


fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


struct Rectangle {

    ll: u8,
    ww: u8
}


impl Rectangle {

    fn is_square(&self) -> bool {
        self.ll == self.ww
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  read, parse § write files

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn read_lines_from_file() {

    if let Ok(ff) = open_file("./data0.txt") {

        let lines = ff.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]                                        // only compiles module if: ∞ cargo test
mod parse_test {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    // use chrono::{DateTime, FixedOffset, TimeZone, NaiveDateTime, Utc, Local};
    // use chrono_tz::US::Pacific;
    // use chrono_tz::America::Chicago;

    #[test]
    #[should_panic]
    fn test_basic1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }


    #[test]
    fn test_basic2() {

        assert_eq!(super::exp_2(), super::add(1, 1));
        assert_ne!(super::exp_2(), super::add(1, 2));
    }


    #[test]
    fn test_struct1() {

        let rr = super::Rectangle {
            ll: 20,
            ww: 20
        };

        assert!(rr.is_square());
    }


    #[test]
    fn test_parse_method() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));
    }


    #[test]
    #[ignore]
    // test DateTime::parse_from_{ str, rfc2822, rfc3339}
    fn test_parse_from() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

        assert_ne!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+08:00"), Ok(fixed_dt.clone()));
    }


    #[test]
    // test chrono::Utc.datetime_from_strm
    fn test_datetime_from_str() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);

        assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
        assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

        // oops, the year is missing!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
        // oops, the format string does not include the year at all!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
        // oops, the weekday is incorrect!
        assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {

    print!("{}📚 STARTING: t1_tdd.rs - Test Driven Development  \n\n", C_LL);

    // read_lines_from_file();

    print!("{}📚 Testing  \n\n", C_LL);

    print!("\ndone!\n");



}

/* THE CODE PIT

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */

