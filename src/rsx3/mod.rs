//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ rsx3::mod.rs  ι✧21․12․25✦16․50․24․  🌎η ✧22․08․22․ ✧22․08․19․ ✧22․08․16․ ✧22․08․07․ ✧22․08․05․ ✧22․07․04․ ✧22․06․22․

use std::error::Error;
mod s1_exec;                    use s1_exec::{mod_main};                               

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;` 



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ exec function 
pub fn run_system() -> Result<(), Box<dyn Error>> {

    match mod_main() {
        Err(ee) => panic!("{}", ee),
        _       => Ok(()),
    }
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Recent Crates 

mod s1_exec;                    use s1_exec::{mod_main};                               
mod a83_regex_basics;           use a83_regex_basics::{mod_main};                               
mod a82_string_methods;         use a82_string_methods::{mod_main};                             

*/
// End Of The Code Pit

