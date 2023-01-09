// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ c0_time_date_zone0.rs  [ιδ21.12.22 τ08:48:42] 

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;


fn say_hello(to: &str) {
    print!("hello {}\n", to);
}

fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
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




///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {


    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

    // method 1
    assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

    // method 2
    assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

    // method 3
    assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
    assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

    // oops, the year is missing!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
    // oops, the format string does not include the year at all!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
    // oops, the weekday is incorrect!
    assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());


    Ok(())
    // panic!("for: No Reason");
}


/* THE CODE PIT 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.07] 
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let tt = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", tt);

    let tt = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", tt);

    let tt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", tt);

    Ok(())
}



•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.05] 
    let tt = DateTime::parse_from_str("Fri Nov 28 12:00:09 2021", "%a %b %e %T %Y");
    print!("Fri Nov 28 12:00:09:                {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y");
    print!("Fri Nov 28 12:00:09:                {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08", "%b %d, %Y %H.%M%Z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

    // method 1
    assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

    // method 2
    assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

    // method 3
    assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
    assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

    // oops, the year is missing!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
    // oops, the format string does not include the year at all!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
    // oops, the weekday is incorrect!
    assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.05] 
Jan-10-1990-00.00-P-0003000
Jan-11-1991-00.01-P-0003011
Jan-12-1992-00.02-P-0003022
Jan-13-1993-00.03-P-0003033
Jan-14-1994-00.04-P-0003044
Jan-15-1995-00.05-P-0003055
Jan-16-1996-00.06-P-0003066
Jan-16-1996-00.06-P-0003066
Jan-17-1997-00.07-P-0003077
Jan-18-1998-00.08-P-0003088
Jan-19-1999-00.09-P-0003099
Jan-20-2000-00.10-P-0003000
Jan-21-2211-00.11-P-0003011

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.07] 

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ c0_time_date_zone0.rs  [ιδ21.12.22 τ08:48:42] 

#![allow(dead_code)]
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Local};
use chrono_tz::US::Pacific;


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main
pub fn mod_main() {
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: c0-time-date-zone0.rs  \n\n", C_LL);

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

    print!("\ndone!\n");
}




use dateparser::DateTimeUtc;

use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Local};
use chrono_tz::US::Pacific;
use std::error::Error;

use chrono_tz::America::Chicago;

fn main() {
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: c1-time-date-zone0.rs  \n\n", C_LL);

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
    "May 25, 2021",
    "oct 7, 1970",
    "oct 7, 70",
    "oct. 7, 1970",
    "oct. 7, 70",
    "October 7, 1970",
    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);

    print!("\ndone!\n");
}
fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
*/



/*
fn main() {
    let parsed = "Wed, 02 Jun 2021 06:31:39 GMT".parse::<DateTimeUtc>()?.0;
    println!("{:#?}", parsed.with_timezone(&Pacific));
    Ok(());


    read_lines_from_file();
    if let Ok(usrn) = read_username_from_file() {
        println!("{}", usrn);
    }

    print!("📚 Testing Converting from one time-zone to another \n\n");
        "May 25, 2021",
        "oct 7, 1970",
        "oct 7, 70",
        "oct. 7, 1970",
        "oct. 7, 70",
        "October 7, 1970",
    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);
}

US::
    Alaska	
    Aleutian	
    Arizona	
    Central	
    EastIndiana	
    Eastern	
    Hawaii	
    IndianaStarke	
    Michigan	
    Mountain	
    Pacific	
    Samoa

America::
    Adak	
    Anchorage	
    Anguilla	
    Antigua	
    Araguaina	
    Aruba	
    Asuncion	
    Atikokan	
    Atka	
    Bahia	
    Bahia_Banderas	
    Barbados	
    Belem	
    Belize	
    BlancSablon	
    Boa_Vista	
    Bogota	
    Boise	
    Buenos_Aires	
    Cambridge_Bay	
    Campo_Grande	
    Cancun	
    Caracas	
    Catamarca	
    Cayenne	
    Cayman	
    Chicago	
    Chihuahua	
    Coral_Harbour	
    Cordoba	
    Costa_Rica	
    Creston	
    Cuiaba	
    Curacao	
    Danmarkshavn	
    Dawson	
    Dawson_Creek	
    Denver	
    Detroit	
    Dominica	
    Edmonton	
    Eirunepe	
    El_Salvador	
    Ensenada	
    Fort_Nelson	
    Fort_Wayne	
    Fortaleza	
    Glace_Bay	
    Godthab	
    Goose_Bay	
    Grand_Turk	
    Grenada	
    Guadeloupe	
    Guatemala	
    Guayaquil	
    Guyana	
    Halifax	
    Havana	
    Hermosillo	
    Indianapolis	
    Inuvik	
    Iqaluit	
    Jamaica	
    Jujuy	
    Juneau	
    Knox_IN	
    Kralendijk	
    La_Paz	
    Lima	
    Los_Angeles	
    Louisville	
    Lower_Princes	
    Maceio	
    Managua	
    Manaus	
    Marigot	
    Martinique	
    Matamoros	
    Mazatlan	
    Mendoza	
    Menominee	
    Merida	
    Metlakatla	
    Mexico_City	
    Miquelon	
    Moncton	
    Monterrey	
    Montevideo	
    Montreal	
    Montserrat	
    Nassau	
    New_York	
    Nipigon	
    Nome	
    Noronha	
    Nuuk	
    Ojinaga	
    Panama	
    Pangnirtung	
    Paramaribo	
    Phoenix	
    Port_of_Spain	
    PortauPrince	
    Porto_Acre	
    Porto_Velho	
    Puerto_Rico	
    Punta_Arenas	
    Rainy_River	
    Rankin_Inlet	
    Recife	
    Regina	
    Resolute	
    Rio_Branco	
    Rosario	
    Santa_Isabel	
    Santarem	
    Santiago	
    Santo_Domingo	
    Sao_Paulo	
    Scoresbysund	
    Shiprock	
    Sitka	
    St_Barthelemy	
    St_Johns	
    St_Kitts	
    St_Lucia	
    St_Thomas	
    St_Vincent	
    Swift_Current	
    Tegucigalpa	
    Thule	
    Thunder_Bay	
    Tijuana	
    Toronto	
    Tortola	
    Vancouver	
    Virgin	
    Whitehorse	
    Winnipeg	
    Yakutat	
    Yellowknife	

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */

