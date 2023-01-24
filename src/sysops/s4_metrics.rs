// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ s1_metrics  ι✧21․11․22✦10․08․26․ 🌎η ✧22․11․12․✧22․10․22․✧22․08․19․✧22․04․21․✧21․12․15․
// Folding g-metrrics on csv files 
#![allow(dead_code)]
use std::fs;
use lib3::q4_fold::{Fold, FMap};
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

///λ gen_fold_metrics(): y4g-metrrics_fold1/2.csv <- fold(x4raw.csv) 
///  generates g-metrrics based on counting and folding g-metrhods  i.e.: 
///  1. reads a csv file and generates a hashmap of each unique value and its count 
///  2. removes the first column and repeats the process but keeing the original count 
///  3. writes the result to another csv file 
pub fn gen_fold_metrics() -> Result<(), String> {

    let my_location = "s4_metrics::gen_fold_metrics";
    print!("\n🎡𐡋 running: {}\n", my_location);
    match fs::read_to_string("x4raw.csv") {
        Err(ee) => Err(format!("read_error[{ee}]@{my_location}")),

        Ok(in_string) => {
            print!("\n🎡𐡋 read file 👍υ OK! \n");
            let fmap1: lib3::q4_fold::FMap = FMap::new().from_string(in_string);
            match fmap1.to_file("y4g-metrrics_fold1.csv") {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => {
                    print!("\n🎡𐡋 fold 1 written 👍υ OK! \n");
                    match fmap1.fold().to_file("y4g-metrrics_fold2.csv") {
                        Err(ee) => Err(format!("write_key_error[{ee}]@{my_location}")),
                        _ => Ok(()),
                    }
                }
            }
        }
    }
}



/// check int-tests the active system as a whole
pub fn check() -> Result<(), String> {
    Ok(())
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
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

