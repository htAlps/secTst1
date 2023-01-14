// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ lib3::lib  ι✧21․12․25✦16․50․24․  🌎η ✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․

// use std::error::Error;
pub mod q0_env;
pub mod q1_lex;
pub mod q2_hash;
pub mod q3_regex;
pub mod q4_fold;
pub mod c2_string;
pub mod c3_regex;
use std::env;

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

/// check int-tests lib3's q-modules 
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


/// check int-tests lib3's c-modules 
pub fn check1() -> Result<(), String> {      // Shortening Check to regex only; full version in The Code Pit 

    let my_location = "lib::c_check";
    print!("\n🎡𐡋 {my_location} \n");
    match c3_regex::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}

/// check int-tests lib3 <-> active system as a whole
pub fn check() -> Result<(), String> {

    let my_location = "lib::check";
    print!("\n🎡𐡋 {my_location} \n");
    match q0_env::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => match q1_lex::check() {
            Err(ee) => Err(format!("{ee}⟸ {my_location}")),
            _ => match q2_hash::check() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => match q3_regex::check() {
                    Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                    _ => match q4_fold::check() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                }
            }
        }
    }
}



//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// check int-tests lib3's c-modules 
pub fn check() -> Result<(), String> {      // Shortening Check to regex only; full version in The Code Pit 

    let my_location = "lib::c_check";
    print!("\n🎡𐡋 {my_location} \n");
    match c3_regex::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}

/// check int-tests lib3 <-> active system as a whole
pub fn check() -> Result<(), String> {

    let my_location = "lib::check";
    print!("\n🎡𐡋 {my_location} \n");
    match q0_env::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => match q1_lex::check() {
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
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("🎡𐡋 {my_location} \n");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

