// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ s2_operations.rs  ι✧22․05․20✦05․15․31․ 🌎η ✧23․01․17․✧23․01․10․✧22․11․25․✧22․10․11․✦06․✧22․07․05․✧22․05․22․✧22․05․21․✧22․05․20․
// Operations on HashSets and HashMaps like: `+` `-` `*` `/` 
#![allow(dead_code)]
use lib3::q2_hash::*;
use std::collections::HashSet;
// use std::fs;

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`
#[cfg(test)]
mod test_regex {
    // use super::*;

}

/// check runs each module integration tests 
pub fn check() -> Result<(), String> {

    let my_location = "sysops::check"; 
    print!("\n🎡𐡋 {my_location}::\n");
    
    let set1 = HashSet::from(["22", "44", "33"]);
    let set2 = HashSet::from(["77", "33", "55"]);
    print!("set1: {:?}\n", set1);
    print!("set2: {:?}\n", set2);

    let set1 = HSet::new().from_string("22\n44\n33".to_string());   // Check Add `+` (union)
    let set2 = HSet::new().from_string("77\n33\n55".to_string());
    let set3: HSet = set1 + set2;
    print!("set1 + set2: \n{}\n", set3.to_string());

    let set1 = HSet::new().from_string("22\n44\n33".to_string());   // Shadow-Check Sub `-` (difference)
    let set2 = HSet::new().from_string("77\n33\n55".to_string());
    let set3: HSet = set1 - set2;
    print!("set1 - set2: \n{}\n", set3.to_string());

    let set1 = HSet::new().from_string("22\n44\n33".to_string());   // Shadow-Check Rem `%` (intersection)
    let set2 = HSet::new().from_string("77\n33\n55".to_string());
    let set3: HSet = set1 % set2;
    print!("set1 % set2: \n{}\n", set3.to_string());
    Ok(())
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// Traits, Constants, Types § Enums


///λ add_sets() performs union: y2hs1.csv <- x2hs1.csv + x2hs2.csv
pub fn add_sets() -> Result<(), String> {
    Ok(())
}

///λ sub_sets() performs symetric-difference: y2hs2.csv <- x2hs1.csv - x2hs2.csv
pub fn sub_sets() -> Result<(), String> {
    Ok(())
}

///λ int_sets() performs intersection: y2hs2.csv <- x2hs1.csv - x2hs2.csv
pub fn int_sets() -> Result<(), String> {
    Ok(())
}

///λ add_maps() performs union: y2hm1.csv <- x2hm1.csv + x2hm2.csv
pub fn add_maps() -> Result<(), String> {
    Ok(())
}

///λ sub_maps() performs symetric-difference: y2hm2.csv <- x2hm1.csv - x2hm2.csv
pub fn sub_maps()  -> Result<(), String> {
    Ok(())
}

///λ int_maps() performs intersection: y2hm2.csv <- x2hm1.csv - x2hm2.csv
pub fn int_maps()  -> Result<(), String> {
    Ok(())
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() is the system's exec fn for sysops::s2_hash sub-module; 
pub fn run() -> Result<(), String> {

    let my_location = "s2_hash::run";
    print!("\n🎡𐡋 {my_location} \n");
    Ok(())
}



//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() is the system's exec fn for sysops::s2_hash sub-module; 
pub fn run() -> Result<(), String> {

    let my_location = "s2_hash::run";
    print!("\n🎡𐡋 {my_location} \n");
    match fs::read_to_string("/usr/local/sys/sys3rs/data/x41_input_data.csv") {
        Err(ee) => Err(format!("read_error[{ee}]@{my_location}")),

        Ok(in_string) => {
            let fmap1 = FMap::init(in_string);
            let table1 = format!("{}\n", fmap1.to_table());
            print!("\n🎡𐡋 file read -> map; 1st fold done: 👍υ OK! \n");

            
            match fs::write("/usr/local/sys/sys3rs/data/y42_iter1_subtable.csv", &table1) {
                Err(ee) => Err(format!("write_fmap_error[{ee}]@{my_location}")),
                _ => {
                    print!("\n🎡𐡋 2nd fold done; map written to file: 👍υ OK! \n");
                    let table2 = format!("{}\n", fmap1.fold().to_table());
                    match fs::write("/usr/local/sys/sys3rs/data/y42_iter2_subtable.csv", &table2) {
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


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ fmap_iter does the first hashmap iteration of counting duplicates, i.e.:
///  1. reads table from x41_in_data.csv and runs the first hashmapping iteration 
///  2. writes the resulting hashmap to: y41_iter1_res.csv
///  3. removes the last column of the table and writes subtable to: x42_iter1_subtable.csv
pub fn fmap_iter()  -> Result<(), String> {

    let my_location = "s2_hash::fmap_iter";
    print!("\n🎡𐡋 {my_location} \n");
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
    let my_location = "s2_hash::check";
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

