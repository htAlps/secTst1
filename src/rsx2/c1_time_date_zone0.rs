// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ c1_time_date_zone0.rs  [ιδ21.12.22 τ08:48:42]

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;
use chrono_tz::US::Pacific;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types 

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Some simple functions 

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



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// read, parse § write files 

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


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ unit tests 
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


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {


    print!("{}📚 STARTING: c1-time-date-zone0.rs  \n\n", C_LL);

    // read_lines_from_file();
    
    print!("📚 Testing time, date and zones \n\n");
    println!("{:?}", chrono::offset::Local::now());
    println!("{:?}", chrono::offset::Utc::now());

    print!("📚 Testing time \n\n");
    let tnow = Local::now();
    let tutc = Utc::now();

    print!("local time: {}\n", tnow);
    print!("UTC time:   {}\n", tutc);

    print!("local time: {:?}\n", tnow);
    print!("UTC time:   {:?}\n", tutc);

    print!("📚 Testing Converting from one time-zone to another \n\n");

    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);


    print!("{}📚 Testing parsing from rfc2822 \n\n", C_LL);

    let tt = DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_rfc2822("28 Nov 2014 21:00:09 +0900");
    print!("28 Nov 2014 21:00:09 +0900:         {}\n", tt.unwrap());
        

    print!("{}📚 Testing DateTime parsing-from-string \n\n", C_LL);

    let tt = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    print!("2014-11-28 21:00:09 +09:00:         {}\n", tt.expect("oops!"));
        
    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0900", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.expect("oops!"));
        
    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0000", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 :         {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 +0000", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z");
    print!("5.8.1994 8:00 am +0000:             {}\n", tt.unwrap());

    // Note that the Z-Offset of: -0800, refers to the local time and the time diference to UTC
    // e.g., 1200 -08:00, means that it's noon in the local zone and 4:00 am in UTC. 
    print!("{}📚 Testing parsing-from-string in CST Time Zone (UTC - 6) \n\n", C_LL);

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 -0600", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());
    
    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 -0600", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());
    

    print!("{}📚 Testing NaiveDateTime parsing-from-string \n\n", C_LL);
    
    let tt = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S");
    print!("23:56:04:                           {}\n", tt.unwrap());

    let tt = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d");
    print!("2015-09-05:                         {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
    print!("2015-09-05 23:56:04:                {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("23:56:04 2015-09-05", "%H:%M:%S %Y-%m-%d");
    print!("23:56:04 2015-09-05:                {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("23 56 04 2015 Aug 05", "%H %M %S %Y %b %d");
    print!("23 56 04 2015 Aug 05:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("23 56 04 Aug 05 2015", "%H %M %S %b %d %Y");
    print!("23 56 04 Aug 05 2015:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("Aug 05 23 56 04 2015", "%b %d %H %M %S %Y");
    print!("Aug 05 23 56 04 2015:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("Aug 05 23:56:04 2015", "%b %d %H:%M:%S %Y");
    print!("Aug 05 23:56:04 2015:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("Nov 27 12:00:09 2021", "%b %d %H:%M:%S %Y");
    print!("Nov 27 12:00:09 2021:               {}\n", tt.unwrap());
        
        
    let tt = NaiveDateTime::parse_from_str("Fri Nov 28 12:00:09 2014", "%a %b %d %T %Y");
    print!("Fri Nov 28 12:00:09 2014:           {}\n", tt.unwrap());
        
    let tt = NaiveDateTime::parse_from_str("Sat Nov 29 12:00:09 2014", "%a %b %d %T %Y");
    print!("Sat Nov 29 12:00:09 2014:           {}\n", tt.unwrap());
        
    let tt = NaiveDateTime::parse_from_str("Dec 18, 1998 19.08;", "%b %d, %Y %H.%M;");
    print!("Dec 18, 1998 19.08:                 {}\n", tt.unwrap());


    print!("\ndone!\n");
    

    Ok(())
    // panic!("for: No Reason");
}


/* THE CODE PIT 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Moved  => c0_..




END OF THE CODE PIT */
