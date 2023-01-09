//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ c1_collections4hashsets.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․08․19․ ✧22․04․21․ ✧21․11․07․
#![allow(dead_code)]
use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;
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

fn prt_set(label: &str, set: &HashSet<&str>) {
    print!("    {}: {{ ", label);         // ⭐ν \{ does not work use {{ to print curly braces
    for ee in set {
        print!("{}, ", ee);
    }
    print!("}}\n");
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
    }
    res
}

// count_str counts words in a String object
fn set_from_txt(tt: &str) -> HashSet<&str> {
    let mut res: HashSet<&str> = HashSet::new();
    for txt in tt.split_whitespace() {
        res.insert(txt);
    }
    res
}
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Testing: a81_collections3hashsets.rs \n\n", C_LL);

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Inserting into HashSets \n", C_LL);
    let _t1 = String::from("Team1");
    let _t2 = String::from("Team2");
    let _t3 = String::from("Team3");

    print!("🎡𐡋 Make a set from words in text and print \n");
    let set1 = &set_from_txt("set1 aa bb cc dd ee");
    let set2 = &set_from_txt("set2 dd ee ff gg hh");

    prt_set("set1", _set1);
    prt_set("set2", _set2);
    let set1union2 = set1.union(&set2);
    print!("    set1 ∪ set2: {:?}\n", set1union2);
    let set1intersec2 = set1.intersection(&set2);
    print!("    set1 ∩ set2: {:?}\n", set1intersec2);
    let set1minus2 = set1.difference(set2);
    print!("    set1 - set2: {:?}\n", set1minus2);
    let set1symdiff2 = set1.symmetric_difference(set2);
    print!("    set1 ∆ set2: {:?}\n", set1symdiff2);
    let set1equation = set1intersec2.u

    print!("{}🎡𐡋 Testing: a81_collections3hashsets.rs \n\n", C_LL);

    Ok(())
    // panic!("for: No Reason");
}



//λ The Code Pit
/*
🏛№ Glossary of Set Operators
    № ∪:    Union
    № ∩:    Insersection
    № ∇:    Backward Difference (regular difference)
    № ∆:    Forward Difference (Symetric difference)


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let setu = &set_from_txt("setu aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz");
    let set1 = &set_from_txt("set1 aa bb cc dd ee");
    let set2 = &set_from_txt("set2 dd ee ff gg hh");
    let set3 = &set_from_txt("set3 gg hh ii jj");
    let set4 = &set_from_txt("set4 jj kk ll mm");
    let set5 = &set_from_txt("set5 mm nn oo pp");
    let set6 = &set_from_txt("set6 pp qq rr ss");
    let set7 = &set_from_txt("set7 ss tt uu vv");
    let set8 = &set_from_txt("set8 vv ww xx yy zz");
*/
// End Of The Code Pit

