// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ mylib::lib  ι✧21․12․25✦16․50․24․  🌎η ✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․

// use std::error::Error;
pub mod q1_lex;
pub mod q2_hash;
pub mod q3_regex;
pub mod q4_fold;
pub mod c2_string;
pub mod c3_regex;
// use std::env;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`
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

#[cfg(test)]
mod check_tests {
    use super::*;
    #[test]
    fn test_mod() {
        assert_eq!(check(), Ok(()));
    }
}

///λ q_check is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
pub fn q_check() -> Result<(), String> {

    let my_location = "lib::q_check";
    print!("\n🎡𐡋 {my_location} \n");
    match q1_lex::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => match q2_hash::check() {
            Err(ee) => Err(format!("{ee}⟸ {my_location}")),
            _ => match q3_regex::check() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => match q4_fold::check() {
                    Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                    _ => Ok(()),
                },
            }
        },
    }
}


///λ c_check is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
pub fn c_check() -> Result<(), String> {

    let my_location = "lib::c_check";               // Shortening Check to regex only; full version in The Code Pit 
    print!("\n🎡𐡋 {my_location} \n");
    match c3_regex::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}

///λ check is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
pub fn check() -> Result<(), String> {

    let my_location = "lib::check";
    print!("\n🎡𐡋 {my_location} \n");
    match q1_lex::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => match q2_hash::check() {
            Err(ee) => Err(format!("{ee}⟸ {my_location}")),
            _ => match q3_regex::check() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => match q4_fold::check() {
                    Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                    _ => Ok(()),
                },
            }
        },
    }
}

//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ c_check is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
pub fn c_check() -> Result<(), String> {

    let my_location = "lib::c_check";
    print!("\n🎡𐡋 {my_location} \n");
    match c2_string::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => match c3_regex::check() {
            Err(ee) => Err(format!("{ee}⟸ {my_location}")),
            _ => Ok(()),
        }
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// use std::error::Error;
pub mod q1_lex;
pub mod q2_hash;
pub mod q3_regex;
// use std::env;

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
mod sysop_exec;                     use sysop_exec::{check};
mod a84_re_multiline;               use a84_re_multiline::{check};
mod a83_regex_basics;               use a83_regex_basics::{check};
mod a82_string_methods;             use a82_string_methods::{check};

mod q1_lex;                         use q1_lex::{check};
mod q2_hash;                    use q2_hash::{check};

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Default New Library

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
λ Crates § Modules ← ⟵  ⬅  ⤶  ← ⟸   ⬅️   ⬅  ⇦ ↢  ⪻  𐧎   𒀪       [cun] aleph   
Error: "TRACE: read_error[No such file or directory (os error 2)]@check_regex_helpers⟸ q2_hash::check⟸ lib::check⟸ sysop::mod::check<-main"
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("🎡𐡋 {my_location} \n");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

