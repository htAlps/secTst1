//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ a83_regex_basics.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․12․21․✧22․12․21․✧22․08․19․ ✧22․04․21․ ✧21․11․07․
#![allow(dead_code)]
extern crate regex;
use regex::Regex;
use std::error::Error;
use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
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

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ common functions
fn prt_chars(ss: &String) {
    print!("    ss.chars(): ");
    for val in ss.chars() {
        print!("{val} ");
    }
    print!("\n\n");
}

fn prt_graphemes(ss: &String) {
    print!("    ss.graphemes(): ");
    for val in ss.graphemes(true) {
        print!("{val} ");
    }
    print!("\n\n");
}


///λ Regular Expressions Basics
fn regex_basics() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Regular Expressions Basics \n", C_LL);

    print!("🎡𐡋 Match a 5+ letter word (5 or more) \n");
    let res = Regex::new(r"\w{5}");         // new() returns a [Redult] Ok(re) or Error(ee) if RegEx Pattern is invalid
    let re  = res.unwrap();                 // safe to unwrap because we can catch invalid regexes during Checking

    let ss = "aaa ddddaa ccc ddddxx  eee f  g";
    let is_match = re.is_match(ss);
    print!("    is match found? {is_match}\n");

    print!("\n🎡𐡋 Capture words that begin with `d` (d\\w*) \n");
    let ss = "aaa ddddaa ccc ddddxx  eee f  g";
    let re = Regex::new(r"(d\w*)").unwrap();        // Captures are enclosed in parenthesis (like in Vim)
    let res = re.captures(ss);                      // re is an Option: Some(ss) or None

    match res {                                     // Print entire object
        Some(obj) => print!("\nObj: {:?}\n", obj),
        None      => print!("\nNone\n"),
    }

    print!("\n🎡𐡋 Capture words that begin with `d` (d\\w*) \n");
    let ss = "aaa ddddaa ccc ddddxx  eee f  g";
    let re = Regex::new(r"(d\w*)").unwrap();

    let res = re.captures(ss);                      // re is an Option: Some(ss) or None
    match res {                                     // Print first string
        Some(obj) => {
            let res0 = obj.get(0);
            print!("\nres0: {}\n", res0.unwrap().as_str());
            let res1 = obj.get(1);
            print!("\nres1: {}\n", res1.unwrap().as_str());
        },
        None      => print!("\nNone\n"),
    }

    print!("🎡𐡋 Capture words that begin with `d` (d\\w*) \n");
    let ss = "aaa ddddaa ccc ddddxx  eee f  g";
    let re = Regex::new(r"(d{4}\w\w)").unwrap();    // captures are enclosed in parenthesis (like in Vim)

    let res = re.captures(ss);                      // re is an Option: Some(ss) or None

    match res {                                     // print entire object
        Some(obj) => print!("\nObj: {:?}\n", obj),
        None      => print!("\nNone\n"),
    }

    let res = re.captures(ss);                      // re is an Option: Some(ss) or None
    match res {                                     // print first string
        Some(obj) => {
            let res0 = obj.get(0);
            print!("\nres0: {}\n", res0.unwrap().as_str());
            let res1 = obj.get(1);
            print!("\nres1: {}\n", res1.unwrap().as_str());
        },
        None      => print!("\nNone\n"),
    }

}


///λ RegEx Helpers
fn has_ipv4(ss: &str) -> bool {

    lazy_static! {
        static ref RE_IPV4: Regex = Regex::new(r" \d\d?\d?\.\d\d?\d?\.\d\d?\d?\.\d\d?\d? ").unwrap();
    }
    RE_IPV4.is_match(ss)
}

fn regex_helpers() {

    assert!(has_ipv4("j fsl djfh 4.3.4.7 kh osfkjfh"));
    assert!(has_ipv4("jhf lkdjh 4.3.4.27 kh osfkjfh"));
    assert!(has_ipv4("jh slkdfh 4.3.54.27 kh o sj h"));
    assert!(has_ipv4("jhfsl jfh 4.13.54.27 kh fosfkjh"));
    assert!(has_ipv4("jhfsldj h 24.13.54.27 khf ofsjh"));
    assert!(has_ipv4("jhfskdjfh 24.13.54.127 kfh foskh"));
    assert!(has_ipv4("jh lkdjfh 24.13.254.127  kh  osjh"));
    assert!(has_ipv4("jhsl djfh 24.132.254.127  ksh okjh"));
    assert!(has_ipv4("jfslk jfh 228.132.254.127 skh okjh"));
}


///λ RegEx Iteration
fn regex_iterations() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: RegEx on successive non-overlapping matches \n", C_LL);

    let ss = "ln1: aa dddaaba bb cdc  dddddddxyx eee dddyy f \n\
              ln2: aa ddd1 bb ccc  ddd22 eee f \n\
              ln3: aa ddd3 bb ccc  ddd44 eee f \n";
    print!("\nMultiline String:\n{ss}\n");

    // first a simple patern
    let re = Regex::new(r"(ddd)(\w* )").unwrap();       // separating ddd from the trailing letters
    let res = re.captures(ss);

    match res {
        Some(matches) => print!("\nMatches: {:?}\n\n", matches),
        None          => print!("\nNone Found."),
    }

    // then all the patterns
    let re = Regex::new(r"(ddd)(\w*) ").unwrap();

    for cap in re.captures_iter(ss) {
        print!("\nMatch:   {:?}\n", cap);
    }
}


///λ Capture across multiple lines
fn capture_across_mult_lines() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Capturing across multiple lines \n", C_LL);

    let ss =
r#"Controlled Text for Testing RegEx Captures\
ln01: aa,   ddd1,'  bb,   ccc,   ddd22,   ee',   f,   \n\
ln02: aa,   ddd1,"  bb,   ccc,   ddd2",   eee,   f,   \n\
ln03: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln04: aa,   ddd3,   bb,   ccc,   ddd44,   eee,"  f,   \n\
ln05: a",   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln06: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln07: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln08: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln09: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln10: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln11: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln12: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln13: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln14: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln15: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln16: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
ln17: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,   \n\
"#;


    print!("\nMultiline String:\n{ss}\n");

    // first a simple patern
    let re = Regex::new(r#"(dd1)"#).unwrap();

    for cap in re.captures_iter(ss) {
        print!("\nMatch:   {:?}\n", cap);
    }

    print!("\n🎡𐡋 Capture all text inside: (,' text ',)  \n");

    // ignoring all delimiters between single quotes in one line (,'  text  ',)
    let re = Regex::new(r#",'(.*)',"#).unwrap();

    for cap in re.captures_iter(ss) {
        print!("\nMatch:   {:?}\n", cap);
    }

    print!("\n🎡𐡋 Capture all text inside: (,\" text \",)  \n");

    // ignoring all delimiters between double quotes in multiple lines (,"  text  ",)
    let re = Regex::new(r#"(?s),"(.*)","#).unwrap();

    for cap in re.captures_iter(ss) {
        print!("\n\nMatch:   {:?}\n\n", cap);
    }

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ check_mod
pub fn check_mod() -> Result<(), Box<dyn Error>> {
    print!("{}🎡𐡋 Checking: a83_regex_basics.rs \n\n", C_LL);

    capture_across_mult_lines();


    Ok(())
    // panic!("for: No Reason");
}


//λ The Code Pit
/*
// use std::fmt;
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    // regex_basics();
    // regex_helpers();
    // regex_iterations();

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    // first a simple patern
    let re = Regex::new(r#"(,\".*\",)"#).unwrap();                        // ignoring commas inside quotes
    let res = re.captures(ss);

    for cap in re.captures_iter(ss) {
        print!("\nMatch:   {:?}\n", cap);
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

