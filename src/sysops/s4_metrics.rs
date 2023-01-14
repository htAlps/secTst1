// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ s1_metrics  ι✧21․11․22✦10․08․26․ 🌎η ✧22․11․12․✧22․10․22․✧22․08․19․✧22․04․21․✧21․12․15․
// Folding metrics on csv files 
#![allow(dead_code)]
use std::fs;
use lib3::q4_fold::*;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`
#[cfg(test)]
mod test_regex {
    // use super::*;

}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// Traits, Constants, Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ gen_metrics() is the system's exec fn for sysops::s1_metrics sub-module; 
pub fn gen_metrics() -> Result<(), String> {

    print!("\n🎡𐡋 running: sysops::s1_metrics:run \n");
    
    let my_location = "s1_metrics::run";
    match fs::read_to_string("x4raw.csv") {
        Err(ee) => Err(format!("read_error[{ee}]@{my_location}")),

        Ok(in_string) => {
            print!("\n🎡𐡋 read file 👍υ OK! \n");
            let fmap1 = FMap::init(in_string);
            let table1 = format!("{}\n", fmap1.to_table());

            print!("\n🎡𐡋 writing first file 👍υ OK! \n");
            
            match fs::write("y4metrics_fold1.csv", &table1) {
                Err(ee) => Err(format!("write_fmap_error[{ee}]@{my_location}")),
                _ => {
                    print!("\n🎡𐡋 wrote first file 👍υ OK! \n");
                    let table2 = format!("{}\n", fmap1.fold().to_table());
                    match fs::write("y4metrics_fold2.csv", &table2) {
                        Err(ee) => Err(format!("write_key_error[{ee}]@{my_location}")),
                        _ => {
                            print!("\n🎡𐡋 wrote second file 👍υ OK! \n");
                            Ok(())
                        },
                    }
                },
            }
        },
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
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ fmap_iter does the first hashmap iteration of counting duplicates, i.e.:
///  1. reads table from x41_in_data.csv and runs the first hashmapping iteration 
///  2. writes the resulting hashmap to: y41_iter1_res.csv
///  3. removes the last column of the table and writes subtable to: x42_iter1_subtable.csv
pub fn fmap_iter()  -> Result<(), String> {

    let my_location = "s1_metrics::fmap_iter";
    match fs::read_to_string("/usr/local/sys/sys3rs/data/x41_input_data.csv") {
        Err(ee) => Err(format!("read_error[{ee}]@{my_location}")),

        Ok(in_string) => {
            let fmap1 = fmap_count(in_string);
            let csv1 = format!("{}\n", fmap_to_csv(true, "col1, col2, cnt1, cnt2", &fmap1));
            
            match fs::write("/usr/local/sys/sys3rs/data/y41_iter1_res.csv", &csv1) {
                Err(ee) => Err(format!("write_fmap_error[{ee}]@{my_location}")),
                _ => {
                    let csv1 = format!("{}\n", fmap_to_table(&fmap1));
                    
                    match fs::write("/usr/local/sys/sys3rs/data/x42_iter1_subtable.csv", &csv1) {
                        Err(ee) => Err(format!("write_key_error[{ee}]@{my_location}")),
                        _ => Ok(()),
                    }
                },
            }
        },
    }
}


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
            let csv1 = format!("{}\n", fmap1.to_csv("col1, col2, cnt1, cnt2"));
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
            let csv1 = format!("{}\n", fmap_to_csv(true, "col1, col2, cnt1, cnt2", &fmap1));
format!("{}\n", fmap_to_table(&fmap1));
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
            print!("fmap:  {:?}\n", fmap);
            print!("{}\n",lib3::q2_hash::fmap_to_csv(false, "Key", "Value", &fmap));
            Ok(())
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let my_location = "s1_metrics::check";
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

