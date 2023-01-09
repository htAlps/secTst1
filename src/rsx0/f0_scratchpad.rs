// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ f0_scratchpad.rs  polymorphism and Operator overloading   ι✧21․11․12✦06․56․11․ 🌎η ✧22․09․16․ ✧22․07․04․ ✧22․06․28․ ✧22․06․24․
#![allow(dead_code)]
use std::error::Error;
use std::ops::Add;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ tests 
#[cfg(test)]
mod test_system {

    #[test]
    #[should_panic]
    fn fail_test_system() {
        let ok: bool = false;
        assert!(ok);
    }
}

///λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;` 
#[cfg(test)]
mod test_traits {
    use super::*;

    #[test]
    fn test_add() {
        let p1 = Point{ xx: 2, yy: 3 };
        let p2 = Point{ xx: 2, yy: 2 };
        let p3 = Point{ xx: 1, yy: 1 };
        let p4 = Point{ xx: 4, yy: 5 };
        let p5 = Point{ xx: 5, yy: 6 };
        assert_eq!(p4, p1 + p2);
        assert_eq!(p5, p3 + p4);
    }
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ code
#[derive(Debug, PartialEq)]
struct Point {
    xx: i32,
    yy: i32,
}

impl Add for Point {
    type Output = Point;

    // nice and short, no need for mutable 
    fn add(self, other: Point) -> Point {
        let res = Point{ xx: self.xx + other.xx, yy: self.yy + other.yy };
        res
    }
}



///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 j2_generic_types_vs_oper_overload.rs \n\n", C_LL);

    let p1 = Point{ xx: 2, yy: 3 };
    let p2 = Point{ xx: 2, yy: 2 };
    let p3 = Point{ xx: 1, yy: 1 };
    
    let p4 = p1.add(p2);
    print!("p4: {:?}\n", p4);

    let p5 = p3 + p4;                       // Love This !
    print!("p5: {:?}\n", p5);

    Ok(())
    // panic!("for: No Reason");
}



//λ The Code Pit
/*

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    // add function, a longer uglier version 
    fn add(self, other: Point) -> Point {
        let mut res = Point{ xx: 0, yy: 0 };
        res.xx = self.xx + other.xx;
        res.yy = self.yy + other.yy;
        res
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    use super::*;                    // REALLY HARD TO TEST WITHOUT THIS

*/
// End Of The Code Pit


