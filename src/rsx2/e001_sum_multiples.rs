//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//  ✨λ e001_sum_multiples.rs Project Euler  ι✧22․06․13✦15․01․43․ 🌎η ✧22․06․15․    

#![allow(dead_code)]

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//λ unit tests
#[cfg(test)]
mod unit_tests{
    // use chrono::{};

    #[test]
    #[should_panic]
    fn fail_test() {

        let ok: bool = false;
        assert!(ok);
    }

    #[test]
    fn basic_tests1() {

        assert_eq!(super::add_multiples1(6), 8);
        assert_eq!(super::add_multiples1(7), 14);
        assert_eq!(super::add_multiples1(10), 23);
        assert_eq!(super::add_multiples1(11), 33);
        assert_eq!(super::add_multiples1(12), 33);

        assert_eq!(super::add_multiples1(31), 225);
        assert_eq!(super::add_multiples1(1000), 233168);
    }

    #[test]
    fn basic_tests2() {

        assert_eq!(super::add_multiples2(6), 8);
        assert_eq!(super::add_multiples2(7), 14);
        assert_eq!(super::add_multiples2(10), 23);
        assert_eq!(super::add_multiples2(11), 33);
        assert_eq!(super::add_multiples2(12), 33);

        assert_eq!(super::add_multiples2(31), 225);
        assert_eq!(super::add_multiples2(1000), 233168);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//λ functions

//  second attempt - a bit cleaner
pub fn add_multiples1(max: i64) -> i64 {

    let mut mult5 = 0;          //  multiple_of_5
    let mut mult3  = 0;         //  multiple_of_3
    let mut sum  = 0;

    loop {

        if mult3.min(mult5) >= max { break; }
        sum = sum + mult3.min(mult5);
        if mult3 == mult5 {
            mult3 += 3;
            mult5 += 5;
        } else if mult3 < mult5 {
            mult3 += 3;
        } else {
            mult5 += 5;
        }
    }
    sum
}

//  brute force attempt 
pub fn add_multiples2(max: i64) -> i64 {

    let mut mult5 = 0;      //  multiple_of_5
    let mut mult3 = 0;      //  multiple_of_3
    let mut sum = 0;

    'outer: loop {

        if mult3 == mult5 {
            if mult3 < max {
                sum = sum + mult3;
                mult3 = mult3 + 3;
                mult5 = mult5 + 5;
            } else {
                break 'outer;
            }
        } else if mult3 < mult5 {
            if mult3 < max {
                sum = sum + mult3;
                mult3 = mult3 + 3;
            } else {
                break 'outer;
            }
        } else {
            if mult5 < max {
                sum = sum + mult5;
                mult5 = mult5 + 5;
            } else {
                break 'outer;
            }
        }
    }
    sum
}


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {


    print!("{}📚 STARTING: t1_tdd.rs - Test Driven Development  \n\n", C_LL);
    let max1000 = add_multiples1(1000);
    print!("add_multiples1(1000) = {} \n\n", max1000);
    print!("\ndone!\n");

    Ok(())
    // panic!("for: No Reason");
}



/* The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════•
12:  3  6  9  5  10 = 33
31:  3  6  9  12  15  18  21  24  27  30    5  10      20  25 = 225

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════•

End of The Code Pit */

