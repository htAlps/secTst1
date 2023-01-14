// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ `sys3rs::main` ι✧21․12․25✦16․50․24․  🌎η ✧22․12․29․✧22․11․27․✧22․11․12․✧22․10․31․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․
pub mod sysops;

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

/// Testing module dependency etc 
#[cfg(test)]
mod int_tests {                    
    use super::*;
    #[test]
    fn test_sys() {
        assert_eq!(check_sys(), Ok(()));
    }
    #[test]
    fn test_lib() {
        assert_eq!(check_lib(), Ok(()));
    }
}


/// check int-tests the active system as a whole
pub fn check() -> Result<(), String> {

    let my_location = "main::check"; 
    match sysops::check() {                                             // checking sysopss
        Err(ee) => Err(format!("Trace: {ee}⟸ {my_location}")),
        _ => match lib3::check() {                                     // checking library 
            Err(ee) => Err(format!("{ee}⟸ {my_location}")),
            _ => Ok(()),
        }
    }
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ main is the framework's executive for sys3rs; it selects a module from below and runs it
fn main() -> Result<(), String> {

    let my_location = "main"; 
    match lib3::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),                 // checking lib3
        _ => {
            match sysops::check() {                                     // running system
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
    }
}

//λ✧23․01․04․ The Code Pit: sys3rs::main
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
λ Crates § Modules

use std::error::Error;

mod lib3;
mod sysops;
        _       => match sysops::check() {
mod rsx0;
        _       => match rsx0::check() {
mod rsx1;
        _       => match rsx0::check() {
mod rsx2;
        _       => match rsx0::check() {
mod rsx3;
        _       => match rsx0::check() {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ main is the framework's executive for sys3rs; it selects a module from below and runs it
fn main() -> Result<(), String> {

    let my_location = "main"; 
    match sysops::run() {                                               // running system
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ main is the framework's executive for sys3rs; it selects a module from below and runs it
fn main() -> Result<(), String> {

    let my_location = "main"; 
    match sysops::run() {                                               // running system
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ main is the overall framework's executor for sys3rs; it selects a module from below and runs it's exec fn
fn main() -> Result<(), String> {

    lib3::check().expect("some error in lib3::c_check");
    Ok(())
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    lib3::check().expect("some error in lib3::check");                // quick check of lib3
    lib3::q_check().expect("some error in lib3::q_check");
    lib3::c_check().expect("some error in lib3::c_check");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let my_location = "main"; 
    match sysops::run() {                                               // running system
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
*/
// End Of The Code Pit

