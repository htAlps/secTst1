// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ s1_metrics  ι✧21․11․22✦10․08․26․ 🌎η ✧22․11․12․✧22․10․22․✧22․08․19․✧22․04․21․✧21․12․15․
// Folding metrics on csv files 
#![allow(dead_code)]
use std::fs;
use lazy_static::lazy_static;
use lib3::q0_env::{EvType, log_event};
use lib3::q1_lex;
use lib3::q4_map::{Fold, FMap, FHMap};
use lib3::q5_node::{FNode, FSet};
// use lib3::q3_regex::Clean;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`
#[cfg(test)]
mod test_regex {
    // use super::*;

}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// Traits, Constants, Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

///λ gen_fold_metrics_v0(): y40metrics_fold1/2.csv <- fold(x40raw.csv) 
///  generates metrics based on counting and folding methods  i.e.: 
///  1. reads a csv file and generates a hashmap of each unique value and its count 
///  2. removes the first column and repeats the process but keeing the original count 
///  3. writes the result to another csv file 
pub fn gen_fold_metrics_v0() -> Result<(), String> {

    let my_location = "s4_metrics::gen_fold_metrics_v0";
    log_event(EvType::Trace, "", my_location, true);

    match fs::read_to_string("x40raw.csv") {
        Err(ee) => Err(format!("read_error[{ee}]@{my_location}")),

        Ok(in_string) => {
            let fmap1: lib3::q4_map::FMap = FMap::new().fold1(in_string);
            match fmap1.to_file("y40metrics_fold1.csv") {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => {
                    print!("\n🎡𐡋 fold 1 written 👍υ OK! \n");
                    match fmap1.fold2().to_file("y40metrics_fold2.csv") {
                        Err(ee) => Err(format!("write_key_error[{ee}]@{my_location}")),
                        _ => Ok(()),
                    }
                }
            }
        }
    }
}


///λ gen_fold_metrics_v1(): y41metrics_fold1/2.csv <- fold(x41raw.csv) 
///  generates metrics based on counting and folding methods  i.e.: 
///  1. reads a csv file and generates a hashmap of each unique value and a set of metrics  
///  2. removes the first column and repeats the process the metrics include:
///     • iter1_cnt (𑑑) running count of key on first fold (iteration = 1)
///     • iter2_cnt (𑑒) running count of sub-key on second fold (iteration = 2)
///     • avg       (μ) cnt1 / cnt2 - calculated at the end
///     • max_cnt   (↑) current max-cnt found thus far during second fold
///     • max_key   (∧) last sub-key holding the max-count above
///     • min_cnt   (↓) current min-cnt found thus far during second fold
///     • min_key   (∨) Last sub-key holding the min-count above
///  3. writes the result to another csv file 
pub fn gen_fold_metrics_v1() -> Result<(), String> {

    let my_location = "s4_metrics::gen_fold_metrics_v1";
    log_event(EvType::Trace, "", my_location, true);
    
    match fs::read_to_string("x41raw.csv") {
        Err(ee) => Err(format!("read_error[{ee}]@{my_location}")),

        Ok(in_string) => {
            let fhmap1: lib3::q4_map::FHMap = FHMap::new().fold1(in_string);
            match fhmap1.to_file("y41metrics_fold1.csv") {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => {
                    print!("\n🎡𐡋 fold 1 written 👍υ OK! \n");
                    match fhmap1.fold2().to_file("y41metrics_fold2.csv") {
                        Err(ee) => Err(format!("write_key_error[{ee}]@{my_location}")),
                        _ => Ok(()),
                    }
                }
            }
        }
    }
}


///λ gen_node_metrics_v0(): y50metrics_node1/2.csv <- fold_node(x50raw.csv) 
pub fn gen_node_metrics_v0() -> Result<(), String> {

    let my_location = "s4_metrics::gen_node_metrics_v0";
    log_event(EvType::Trace, "", my_location, true);

    match fs::read_to_string("x50raw.csv") {
        Err(ee) => Err(format!("read_error[{ee}]@{my_location}")),
        Ok(in_string) => {
            let fnode1: lib3::q5_node::FNode = FNode::new().fold(in_string);
            Ok(())
        },
    }
}



///λ check_print_fnode_v0() -> Result<(), String> {
pub fn check_print_fnode_v0() -> Result<(), String> {

    
    log_event(EvType::Trace, "", "s4_metrics::check_print_fnode_v0", true);

    let mut set = FSet::new();                      // check - FSet
    set.insert(&LX.val1.v());
    set.insert(&LX.val2.v());
    set.insert(&LX.val3.v());

    print!("\n🎡𐡋 set after inserted 3 values \n{}\n", set);
    
    let mut node = FNode::new();                // check - FNode 
    node.insert(&LX.key1.v(), &LX.val1.v());   // insert a (k, v) (stg == string)
    node.insert(&LX.key1.v(), &LX.val2.v());

    print!("\n🎡𐡋 node after inserted 2 more key, value pairs with 'key 1' as key \n{}\n", node);
    
    node.insert(&LX.key2.v(), &LX.val1.v());
    node.insert(&LX.key2.v(), &LX.val3.v());

    print!("\n🎡𐡋 node after inserted 2 more key, value pairs with 'key 2' as key \n{}\n", node);
    
    node.insert(&LX.key3.v(), &LX.val2.v());
    node.insert(&LX.key3.v(), &LX.val3.v());

    print!("\n🎡𐡋 node after inserted 2 more key, value pairs with 'key 3' as key \n{}\n", node);
    
    Ok(())
}


/// check int-tests the active system
pub fn check() -> Result<(), String> {
    let my_location = "s4_metrics::check";
    match lib3::q5_node::check() {
        Err(ee) => Err(format!("Trace: {ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//ξ Bulky Support Data that I want out of the way because it obfuscatess readability 

lazy_static! {                                                          // cheating to limit verbosity
    static ref LX: q1_lex::Lex = q1_lex::Lex::new();
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    node.insert("key-111".to_string(), "val-111".to_string());       // insert a (k, v) (stg == string)
    node.insert("key-111".to_string(), "val-222".to_string());
    node.insert("key-111".to_string(), "val-333".to_string());
                                         
    node.insert("key-222".to_string(), "val-111".to_string());
    node.insert("key-222".to_string(), "val-222".to_string());
                                         
    node.insert("key-333".to_string(), "val-111".to_string());
    node.insert("key-333".to_string(), "val-222".to_string());

    node.insert(&LX.key1.v(), &LX.val1.v());       // insert a (k, v) (stg == string)
    node.insert(&LX.key1.v(), &LX.val2.v());
    node.insert(&LX.key1.v(), &LX.val3.v());

    node.insert(&LX.key2.v(), &LX.val1.v());
    node.insert(&LX.key2.v(), &LX.val2.v());

    node.insert(&LX.key3.v(), &LX.val1.v());
    node.insert(&LX.key3.v(), &LX.val2.v());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
            let csv1 = format!("{}\n", fmap_to_csv(true, "col1, col2, cnt1, cnt2", &fmap1));
format!("{}\n", fmap_to_table(&fmap1));
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
            print!("fmap:  {:?}\n", fmap);
            print!("{}\n",lib3::q2_hash::fmap_to_csv(false, "Key", "Value", &fmap));
            Ok(())
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    match lib3::check() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _       => Ok(()),
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{C_LL}🎡𐡋 {my_location} \n", );
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

