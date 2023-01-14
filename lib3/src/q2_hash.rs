// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ lib3::q2_hash - hashsets, hashmaps etc ι✧21․11․22✦10․08․26․ 🌎η ✧22․11․12․✧22․10․22․✧22․08․19․✧22․04․21․✧21․12․15․
#![allow(dead_code)]
extern crate regex;
use regex::Regex;
use std::fs;
use std::collections::HashMap;

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


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// Traits, Constants, Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ common functions
fn prt_chars(ss: &String) {
    print!("    ss.chars(): ");
    for val in ss.chars() {
        print!("{val} ");
    }
    print!("\n\n");
}



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ hashmap functions

///λ Captures text patterns across multiple lines
fn capture_across_mult_lines(ss: String) {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 q2_hash::capture_across_mult_lines() \n", C_LL);

    // first a simple patern
    let re = Regex::new(r#"(dd1)"#).unwrap();

    for cap in re.captures_iter(&ss) {
        print!("\nMatch:   {:?}\n", cap);
    }

    print!("\n🎡𐡋 Capture all text inside: (,' text ',)  \n");

    // ignoring all delimiters between single quotes in one line (,'  text  ',)
    let re = Regex::new(r#",'(.*)',"#).unwrap();

    for cap in re.captures_iter(&ss) {
        print!("\nMatch:   {:?}\n", cap);
    }

    print!("\n🎡𐡋 Capture all text inside: (,\" text \",)  \n");

    // ignoring all delimiters between double quotes in multiple lines (,"  text  ",)
    let re = Regex::new(r#"(?s),"(.*)","#).unwrap();

    for cap in re.captures_iter(&ss) {
        print!("\n\nMatch:   {:?}\n\n", cap);
    }
}


/// map_to_csv1 converts a hash map to a csv with headings in forward (k,v) or reverse (v,k) order
pub fn map_to_csv1(fwd: bool, header: &str, map: &HashMap<String, i32>) -> String {

    let mut res: String = String::new();

    let hd: String = header.to_string();
    let cm: String = ",".to_string();
    let nl: String = "\n".to_string();
    if fwd {
        res = res + &hd + &nl;
        for (kk, vv) in map {
            res = res + &kk + &cm + &vv.to_string() + &nl;
        }
    } else {
        res = res + &hd + &nl;
        for (kk, vv) in map {
            res = res + &vv.to_string() + &cm + &kk + &nl;
        }
    }
    res
}


/// map_to_csv2 converts a hash map to a csv with headings in forward (k,v) or reverse (v,k) order
pub fn map_to_csv2(fwd: bool, header: &str, map: &HashMap<String, (i32, i32)>) -> String {

    let mut res: String = String::new();

    let hd: String = header.to_string();
    let cm: String = ",".to_string();
    let nl: String = "\n".to_string();
    if fwd {
        res = res + &hd + &nl;
        for (kk, vv) in map {
            res = res + &kk + &cm + &vv.0.to_string() + &cm + &vv.1.to_string() + &nl;
        }
    } else {
        res = res + &hd + &nl;
        for (kk, vv) in map {
            res = res + &vv.0.to_string() + &cm + &vv.1.to_string() + &cm + &kk + &nl;
        }
    }
    res
}


/// map_to_table1 converts a hash map to a csv with headings in forward (k,v) or reverse (v,k) order
pub fn map_to_table1(map: &HashMap<String, i32>) -> String {

    let mut res: String = String::new();

    let cm: String = ",".to_string();
    let nl: String = "\n".to_string();
    for (kk, vv) in map {
        res = res + &kk + &cm + &vv.to_string() + &nl;
    }
    res
}

/// map_to_table2 converts a hash map to a csv with headings in forward (k,v) or reverse (v,k) order
pub fn map_to_table2(map: &HashMap<String, (i32, i32)>) -> String {

    let mut res: String = String::new();

    let cm: String = ",".to_string();
    let nl: String = "\n".to_string();
    for (kk, vv) in map {
        res = res + &kk + &cm + &vv.0.to_string() + &cm + &vv.1.to_string() + &nl;
    }
    res
}


///λ map_count1 inputs a csv, multiline string, counts the unique lines and returns a hashmap (key: unique_line, value: count_of_duplicates)
pub fn map_count1(ss: String) -> HashMap<String, i32> {

    let mut res: HashMap<String, i32> = HashMap::new();
    for line in ss.lines() {
        match res.get(line) {
            None => res.insert(line.to_string(), 1),
            Some(count) => res.insert(line.to_string(), count+1),
        };
    }
    res
}

///λ map_count2 inputs a csv, multiline string, counts the unique lines and returns a hashmap (key: unique_line, value: count_of_duplicates)
pub fn map_count2(ss: String) -> HashMap<String, (i32, i32)> {

    let mut res: HashMap<String, (i32, i32)> = HashMap::new();
    for line in ss.lines() {
        match res.get(line) {
            None => res.insert(line.to_string(), (0, 1)),
            Some(count) => res.insert(line.to_string(), (count.0, count.1+1)),
        };
    }
    res
}


///λ map_reduce1 inputs a csv, multiline string, counts the unique lines and returns a hashmap (key: unique_line, value: count_of_duplicates)
pub fn map_reduce1(hm: HashMap<String, (i32, i32)>) -> HashMap<String, (i32, i32)> {

    let mut res: HashMap<String, (i32, i32)> = HashMap::new();
    for (kk, vv) in hm {
        res.insert(kk, vv);
    }
    res
}

///λ map_reduce2 inputs a csv, multiline string, counts the unique lines and returns a hashmap (key: unique_line, value: count_of_duplicates)
pub fn map_reduce2(hm: HashMap<String, (i32, i32)>) -> HashMap<String, (i32, i32)> {

    let mut res: HashMap<String, (i32, i32)> = HashMap::new();
    for (kk, _) in hm {
        match res.get(&kk) {
            None => res.insert(kk, (0, 1)),
            Some(count) => res.insert(kk, (count.0, count.1+1)),
        };
    }
    res
}



///λ check int-tests lib3's q2_hash module 
pub fn check() -> Result<(), String> {

    let my_location = "q2_hash::check";
    print!("\n🎡𐡋 {my_location} \n");
    match fs::read_to_string("/usr/local/sys/sys3rs/data/x32_in_count_duplicates.csv") {
        Err(ee)       => Err(format!("read_error[{ee}]@{my_location}")),
        Ok(in_string) => {
            let map1 = map_count1(in_string);
            print!("map1:  {:?}\n", map1);
            print!("{}\n", map_to_csv1(true, "Key1, Key2, dupl_count", &map1));
            Ok(())
        },
    }
    /*
    */
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
pub fn check() -> Result<(), String> {

    let my_location = "q2_hash::check";
    match check_regex_helpers() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _       => Ok(()),                                              // don't: panic!("for: No Reason");
    }
}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let in_string = fs::read_to_string("/usr/local/sys/rust/data/x2_count_duplicates.csv").expect("check::Read Error");
    let map = map_count(in_string);
    print!("map:  {:?}\n", map);
    print!("{}\n",map_to_csv1(false, "Key", "Value", &map));

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    // With the return type rewritten, we use pattern matching without `unwrap()`.
    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        match first_number_str.parse::<i32>() {
            Ok(first_number)  => {
                match second_number_str.parse::<i32>() {
                    Ok(second_number)  => {
                        Ok(first_number * second_number)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{C_LL}🎡𐡋 {my_location} \n");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

