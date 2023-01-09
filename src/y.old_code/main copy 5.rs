//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ main  ι✧21․12․25✦16․50․24․  🌎η ✧22․08․22․ ✧22․08․19․ ✧22․08․16․ ✧22․08․07․ ✧22․08․05․ ✧22․07․04․ ✧22․06․22․

use std::error::Error;
use crate::a83_regex_basics::{mod_main};                                mod a83_regex_basics;

fn main() -> Result<(), Box<dyn Error>> {

    match mod_main() {
        Err(e)  => panic!("{}", e),
        _       => Ok(()),
    }
}

//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Recent Crates 

use crate::s1_pg_sqlx1::{mod_main};                                     mod s1_pg_sqlx1;                               

use crate::j2_generic_types_vs_oper_overload::{mod_main};               mod j2_generic_types_vs_oper_overload;
use crate::j1_traits_assoc_types_vs_generics::{mod_main};               mod j1_traits_assoc_types_vs_generics;
use crate::j1_adv_traits1::{mod_main};                                  mod j1_adv_traits1;
use crate::p1_api_design2::{mod_main};                                  mod p1_api_design2;
use crate::p1_api_design1::{mod_main};                                  mod p1_api_design1;
use crate::n3_gen_gen_trait2::{mod_main};                               mod n3_gen_gen_trait2;
use crate::n3_gen_gen_trait1::{mod_main};                               mod n3_gen_gen_trait1;
use crate::d2_trait_iter3::{mod_main};                                  mod d2_trait_iter3;
use crate::d2_trait_iter2::{mod_main};                                  mod d2_trait_iter2;
use crate::d2_trait_iter1::{mod_main};                                  mod d2_trait_iter1;

use crate::a83_regex_basics::{mod_main};                                mod a83_regex_basics;
use crate::a82_string_methods::{mod_main};                              mod a82_string_methods;
use crate::a81_collections1vectors::{mod_main};                         mod a81_collections1vectors;
use crate::a81_collections2strings::{mod_main};                         mod a81_collections2strings;
use crate::a81_collections3hashmaps::{mod_main};                        mod a81_collections3hashmaps;
use crate::a81_collections4hashsets::{mod_main};                        mod a81_collections4hashsets;

use crate::a42_cust_tp_enum1::{mod_main};                               mod a42_cust_tp_enum1;
use crate::a41_cust_tp_structs1::{mod_main};                            mod a41_cust_tp_structs1;

*/
// End Of The Code Pit

