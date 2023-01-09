//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ main  ι✧21․12․25✦16․50․24․  🌎η ✧22․08․22․ ✧22․08․19․ ✧22․08․16․ ✧22․08․07․ ✧22․08․05․ ✧22․07․04․ ✧22․06․22․

use std::error::Error;
mod rsx3;   use rsx3::{run_checks};

fn main() -> Result<(), Box<dyn Error>> {

    match run_checks() {
        Err(ee) => panic!("{}", ee),
        _       => Ok(()),
    }
}

//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Recent Crates 

use std::error::Error;

mod rsx0;   use rsx0::{run_checks};
mod rsx1;   use rsx1::{run_checks};
mod rsx2;   use rsx2::{run_checks};
mod rsx3;   use rsx3::{run_checks};

*/
// End Of The Code Pit

