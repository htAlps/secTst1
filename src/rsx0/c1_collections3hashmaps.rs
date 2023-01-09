//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ c1_collections3hashmaps.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․08․19․ ✧22․04․21․ ✧21․11․07․
#![allow(dead_code)]
use std::error::Error;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
// use std::fmt;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests
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

fn prt_chars(ss: &String) {
    print!("    ss.chars(): ");
    for val in ss.chars() {
        print!("{val} ");
    }
    print!("\n");
}

fn prt_graphemes(ss: &String) {
    print!("    ss.graphemes(): ");
    for val in ss.graphemes(true) {
        print!("{val} ");
    }
    print!("\n");
}

fn prt_map(map: &HashMap<&str, i32>) {
    print!("    HashMap: [ ");
    for (kk, vv) in map {
        print!("({}, {}), ", kk, vv);
    }
    print!("]\n");
}

// count_txt counts words in a string literal
fn count_txt(tt: &str) -> HashMap<&str, i32> {
    let mut res: HashMap<&str, i32> = HashMap::new();
    for txt in tt.split_whitespace() {
        match res.get(txt) {
            None        => res.insert(txt, 1),
            Some(count) => res.insert(txt, count+1),
        };
    }
    res
}

// count_str counts words in a String object
fn count_str(ss: &String) -> HashMap<&str, i32> {
    let mut res: HashMap<&str, i32> = HashMap::new();
    for txt in ss.split_whitespace() {
        let p_count: &mut i32 = res.entry(txt).or_insert(0);
        *p_count += 1;          // ⭐ pretty cool to change entry-values via a mutable pointer to our count value
    };
    res
}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Testing: a81_collections3hashmaps.rs \n\n", C_LL);


    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Inserting into HashMaps \n", C_LL);



    let t1 = String::from("Team1");
    let t2 = String::from("Team2");
    let t3 = String::from("Team3");

    let mut map1: HashMap<&str, i32> = HashMap::new();   // Now we have to worry about the lifetime of teams vars
    map1.insert("Team1", 13);
    map1.insert("Team2", 42);
    map1.insert("Team3", 21);

    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert(t1, 13);
    map2.insert(t2, 42);
    map2.insert(t3, 21);

    print!("🎡𐡋 getting values from HashMaps  \n");
    print!("    map1.get((`Team1`):  {:?}\n", map1.get(&"Team1"));
    print!("    map2.get((`Team1`):  {:?}\n", map2.get("Team1"));
    print!("    map2.get((`Team20`): {:?}\n", map2.get("Team20"));

    print!("🎡𐡋 Testing: iterating through a HashMap \n");
    prt_map(&map1);
    for (kk, vv) in map1 {
        print!("    (k, v): ({}, {})\n", kk, vv);
    }

    print!("🎡𐡋 Counting words using string literals \n");
    let txt1 = "aaa bbb ccc ddd aaa aaa bbb ccc ddd aaa aaa bbb ccc ddd aaa";
    prt_map(&count_txt(txt1));

    print!("🎡𐡋 Counting words using strings \n");
    let string1 = String::from("aaa bbb ccc ddd aaa aaa bbb ccc ddd aaa aaa bbb ccc ddd aaa");
    prt_map(&count_str(&string1));

    Ok(())
    // panic!("for: No Reason");
}



//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

