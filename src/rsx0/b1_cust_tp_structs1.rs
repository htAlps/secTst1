//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ b1_cust_tp_structs1.rs
//  ✨λ c1_cust_tp_structs1.rs  structs (Ch 5) ι✧21․08․07✦15․51․01․   🌎η ✧22․07․04․ ✧22․01․28․
#![allow(dead_code)]
use std::error::Error;
use std::fmt;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests
#[cfg(test)]
mod fail_test {
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

struct DevTeam {                // a regular struct type
    id:             i64,
    name_id:        String,
    active:         bool,
    official_name:  String,
    tech_lead:      String,
}

struct Color(i64, i64, i64);    // a tuple struct

struct Point(i64, i64, i64);    // to prevent you from assigning a Color value to a Point variable

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ type implementations

impl Point {
    fn mag_squared(&self) -> i64 {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }
}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ trait implementations

pub trait HasVolume {
    fn volume(&self) -> i64;
}

impl HasVolume for Point {
    fn volume(&self) -> i64 {
        self.0 * self.1 * self.2
    }
}

impl fmt::Display for DevTeam {

    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        write!(ff, "{},  {},  {},  {},  {} \n",
            self.id,
            self.name_id,
            self.active,
            self.official_name,
            self.tech_lead,
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        write!(ff, "({}, {}, {})\n", self.0, self.1, self.2)
    }
}

impl fmt::Display for Point{
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        write!(ff, "({}, {}, {})\n", self.0, self.1, self.2)
    }
}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ functions

fn new_team() -> DevTeam {
    DevTeam {
        id:               3,
        name_id:          String::from("team3"),
        active:           true,
        official_name:    String::from("Team #3"),
        tech_lead:        String::from("Carl3 Lead"),
    }
}


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Testing: c1_cust_tp_structs1.rs \n\n", C_LL);

    print!("🎡𐡋 Testing: structs \n");

    let t1: DevTeam = DevTeam {
        id:               1,
        name_id:          String::from("team1"),
        active:           true,
        official_name:    String::from("Team #1"),
        tech_lead:        String::from("Carl1 Lead"),
    };
    print!("t1: {t1}");

    let mut t2: DevTeam = DevTeam {
        id:               2,
        name_id:          String::from("team2"),
        active:           true,
        official_name:    String::from("Team #2"),
        tech_lead:        String::from("Carl2 Lead"),
    };
    print!("t2: {t2}");

    // t1.official_name = String::from("Team #x");      // 👎ς error[E0594]: cannot assign to `t1.official_name`, as `t1` is not declared as mutable
    t2.name_id          = String::from("teamx");        // 👍υ OK ! bec t2 is Mutable
    t2.official_name    = String::from("Team #x");
    print!("t2: {t2}");

    let t3 = new_team();
    print!("t3: {t3}");

    print!("\n🎡𐡋 Testing: tuple structs \n");
    let c1: Color = Color(1, 2, 3);
    print!("c1: {c1}");

    let p1: Point = Point(1, 2, 3);
    print!("p1: {p1}");
    print!("p1.volume(): {}\n", p1.volume());
    print!("p1.mag_squared(): {}\n", p1.mag_squared());

    Ok(())
    // panic!("for: No Reason");
}



//λ The Code Pit
/*

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

*/
// End Of The Code Pit
