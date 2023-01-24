// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ sysops::mod  ι✧21․12․25✦16․50․24․  🌎η ✧22․12․29․✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․
pub mod s2_hash;
pub mod s3_regex;
pub mod s4_metrics;
// use lib3::q0_env;
// use lib3::q3_regex;
// use lib3;


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// check runs each module integration tests 
pub fn check() -> Result<(), String> {

    let my_location = "sysops::check"; 
    print!("\n🎡𐡋 {my_location}::\n");
    
    match s2_hash::check() {
        Err(ee) => Err(format!("Trace: {ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}


/// check int-tests the active system as a whole
pub fn run() -> Result<(), String> {

    let my_location = "sysops::run"; 
    print!("\n🎡𐡋 running: {}\n", my_location);
    match lib3::q0_env::get_cmd_code() {
        None => Ok(()),
        Some(cmd_code) => {
            print!("cmd_code: {cmd_code} \n");
            match cmd_code.as_str() {
                "add-hs"  => {                                      // add-hs:  y2hs1.csv <- x2hs1.csv ∪ x2hs2.csv      + == ∪ (Union)                
                    print!("\n🎡𐡋 {my_location}::\n");                                                                       
                    match s2_hash::add_sets() {                                                                              
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),                                                      
                        _ => Ok(()),                                                                                         
                    }                                                                                                        
                },                                                                                                           
                "sub-hs"  => {                                      // sub-hs:  y2hs2.csv <- x2hs1.csv ∆ x2hs2.csv      - == ∆ (Symetric-Difference)
                    print!("\n🎡𐡋 {my_location}::\n");                                                                       
                    match s2_hash::sub_sets() {                                                                              
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),                                                      
                        _ => Ok(()),                                                                                         
                    }                                                                                                        
                },                                                                                                           
                "int-hs"  => {                                      // int-hs:  y2hs2.csv <- x2hs1.csv ∩ x2hs2.csv      % == ∩ (Intersection)
                    print!("\n🎡𐡋 {my_location}::\n");                                                 
                    match s2_hash::int_sets() {                                                        
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),                                
                        _ => Ok(()),                                                                   
                    }                                                                                  
                },                                                                                     
                "add-hm"  => {                                      // add-hm:  y2hm1.csv <- x2hm1.csv ∪ x2hm2.csv      + == ∪ (Union) 
                    print!("\n🎡𐡋 {my_location}::\n");                                                                       
                    match s2_hash::add_maps() {                                                                              
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),                                                      
                        _ => Ok(()),                                                                                         
                    }                                                                                                        
                },                                                                                                           
                "sub-hm"  => {                                      // sub-hm:  y2hm2.csv <- x2hm1.csv ∆ x2hm2.csv      - == ∆ (Symetric-Difference)
                    print!("\n🎡𐡋 {my_location}::\n");                                                                       
                    match s2_hash::sub_maps() {                                                                              
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),                                                      
                        _ => Ok(()),                                                                                         
                    }                                                                                                        
                },                                                                                                           
                "int-hm"  => {                                      // int-hm:  y3hm2.csv <- x2hm1.csv ∩ x2hm2.csv      % == ∩ (Intersection)
                    print!("\n🎡𐡋 {my_location}::\n");
                    match s2_hash::int_maps() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },                                   
                "rm-qt"  => {                                       // rm-qt:   y3clean.csv <- clean(x3raw.csv)  
                    print!("\n🎡𐡋 {my_location}::\n");
                    match s3_regex::clean_csv() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },                                   
                "g-metr"  => {                                      // g-metr:  y4g-metrrics_fold1/2.csv <- fold(x4raw.csv)
                    print!("\n🎡𐡋 {my_location}::\n");
                    match s4_metrics::gen_fold_metrics() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },                                    
                "check"  => {                                     // check: Run INT-Tests 
                    print!("\n🎡𐡋 {my_location}:: \n");
                    match check() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },                                    
                "help"  => {                                        // help:    Help
                    print!("\n🎡𐡋 {} \n", lib3::q0_env::_HELP_TABLE );
                    Ok(())
                },
                _ => {
                    Err(format!("⟸ something went wrong@{my_location}"))
                }
            }
        }
    }
}




//λ The Code Pit
/*
   gen_fold_metrics
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() gets the function-code to be executed and runs the corresponding fn
pub fn run() -> Result<(), String> {

    let my_location = "sysops::run"; 
    print!("\n🎡𐡋 running: {}\n", my_location);
    let cmd_code = get_cmd_code();
    
    let my_location = "s1_exec::run";
    match cmd_code {
        "2add-hs"  => {                // y2hs.csv <- x2hs1.csv + x2hs2.csv  
            match s2_hash::add_hashsets() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
        
        ...

}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// check int-tests the active system as a whole
pub fn check() -> Result<(), String> {

    let my_location = "sysops::check";
    match s1_metrics::check() {                                     // checking g-metrrics calculations 
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    match s1_metrics::run() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() is the system's exec fn for sysopss module; 
pub fn run() -> Result<(), String> {

    let my_location = "s1_exec::run";
    let lex1 = Lex::new();
    print!("lex1: \n{lex1}");
    
    match map_iter_2() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
λ Crates § Modules

mod sysops_exec;
mod a84_re_multiline;               use a84_re_multiline::{check};
mod a83_regex_basics;               use a83_regex_basics::{check};
mod a82_string_g-metrhods;             use a82_string_g-metrhods::{check};


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
mod sysops_exec;                    use sysops_exec::{check};
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

