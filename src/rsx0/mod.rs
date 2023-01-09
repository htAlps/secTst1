// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ sysop::mod.rs  ι✧21․12․25✦16․50․24․  🌎η ✧22․11․27․✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․
mod p2_apis_on_traits;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•


/// check_mod is this system's exec fn; it selects a module from list below and runs it's suppervisor fn
pub fn check_mod() -> Result<(), String> {

    let my_location = "rsx0::check_mod"; 
    match p2_apis_on_traits::check_mod() {
        Err(ee) => Err(format!("{ee}@{my_location}.")),
        _       => Ok(()),
    }
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

mod p2_apis_on_traits;
mod p1_apis_on_traits;
mod na_enums_pretty;
mod n8_enums_basic;
use std::error::Error;
mod a82_string_methods;                             use a82_string_methods::{check_mod};                             

pub fn run_system() -> Result<(), Box<dyn Error>> {

    match check_mod() {
        Err(ee) => panic!("{}", ee),
        _       => Ok(()),
    }
}

??? Check

  • T = TDD,  U = DATA,   
  • mod u__ PREFIX IS OPEN for DATA § DBs
  • V (ω) = ALL KATA Excersizes  

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
OLD STYLE Crate Defs  

mod rsx3;

                                                    use std::error::Error;
mod a83_regex_basics;                               use a83_regex_basics::{check_mod};                               
mod a82_string_methods;                             use a82_string_methods::{check_mod};                             
                                                    
mod s1_pg_sqlx1;                                    use s1_pg_sqlx1::{check_mod};                                    
                                                    
mod j2_generic_types_vs_oper_overload;              use j2_generic_types_vs_oper_overload::{check_mod};              
mod j1_traits_assoc_types_vs_generics;              use j1_traits_assoc_types_vs_generics::{check_mod};              
mod j1_adv_traits1;                                 use j1_adv_traits1::{check_mod};                                 
mod n3_gen_gen_trait2;                              use n3_gen_gen_trait2::{check_mod};                              
mod n3_gen_gen_trait1;                              use n3_gen_gen_trait1::{check_mod};                              
mod d2_trait_iter3;                                 use d2_trait_iter3::{check_mod};                                 
mod d2_trait_iter2;                                 use d2_trait_iter2::{check_mod};                                 
mod d2_trait_iter1;                                 use d2_trait_iter1::{check_mod};                                 
                                                    
mod a83_regex_basics;                               use a83_regex_basics::{check_mod};                               
mod a82_string_methods;                             use a82_string_methods::{check_mod};                             
mod a81_collections1vectors;                        use a81_collections1vectors::{check_mod};                        
mod a81_collections2strings;                        use a81_collections2strings::{check_mod};                        
mod a81_collections3hashmaps;                       use a81_collections3hashmaps::{check_mod};                       
mod a81_collections4hashsets;                       use a81_collections4hashsets::{check_mod};                       
                                                    
mod a42_cust_tp_enum1;                              use a42_cust_tp_enum1::{check_mod};                              
mod a41_cust_tp_structs1;                           use a41_cust_tp_structs1::{check_mod};                           

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("🎡𐡋 {my_location} \n");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit


