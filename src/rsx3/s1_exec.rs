//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ s1_exec.rs  ι✧21․11․22✦10․08․26․ 🌎η ✧22․10․22․ ✧22․08․19․ ✧22․04․21․ ✧21․12․15․
#![allow(dead_code)]
extern crate regex;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use lazy_static::lazy_static;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;` 
#[cfg(test)]
mod test_regex {
    use super::*;

    const TEST_STR: &str = r#"\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln03: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln04: aa,   ddd3,   bb,   ccc,   ddd44,   eee,"  f,  \n\
ln05: a",   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln06: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln07: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln08: aa,   ddd3,   bb,   ccc,   ddd44,   eee,"  f,  \n\
ln09: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln10: a",   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln11: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln12: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln13: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln14: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln15: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln16: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln17: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
Downloaded Data (x. Prefix)                          \n\
Controlled Text for Testing RegEx Captures
"#;
    
    #[test]
    fn test_preamble() {
        print!("TEST_STR: {TEST_STR} \n")
    }
    
}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ common functions
fn prt_chars(ss: &String) {
    print!("    ss.chars(): ");
    for val in ss.chars() {
        print!("{val} ");
    }
    print!("\n\n");
}


///λ Read a file the old way
fn read_file(file_path: &str) -> String {

    let path = Path::new(file_path);
    print!("path.display(): {}\n", path.display());

    let mut ff = match File::open(&path) {
        Err(ee) => panic!("Open Error: {ee}\n"),
        Ok(ff)  => ff,
    };


    let mut ss = String::new();
    match ff.read_to_string(&mut ss) {
        Err(ee) => panic!("read_file::Read Error: {ee}\n"),
        Ok(_) => ss
    }
}


///λ Capture across multiple lines
fn capture_across_mult_lines(ss: String) {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Capturing across multiple lines \n", C_LL);

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


///λ RegEx Helper
fn has_ipv4(ss: &str) -> bool {

    lazy_static! {
        static ref RE_IPV4: Regex = Regex::new(r" \d\d?\d?\.\d\d?\d?\.\d\d?\d?\.\d\d?\d? ").unwrap();
    }
    RE_IPV4.is_match(ss)
}


///λ RegEx Helper removes quoted fields within the same line 
fn remove_short_quotes(ss: String) -> String {

    lazy_static! {
        static ref RE_REMOVE_SHORT_QUOTES: Regex = Regex::new(r#"(?P<yes1>.*)(?P<no>,".*",)(?P<yes2>.*)"#).unwrap();
    }
    let res = RE_REMOVE_SHORT_QUOTES.replace_all(&ss, "⫷-$yes1-⫸,--removed_short_quotation--,⫷-$yes2-⫸");
    res.to_string()
}


///λ RegEx Helper removes quoted fields that span across multiple lines 
fn remove_long_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `\s` = ignore line breaks 
        static ref RE_REMOVE_LONG_QUOTES: Regex = Regex::new(r#"((?m)(?P<yes1>(?m).*)(?P<no>(?m),".*",)(?P<yes2>(?m).*))"#).unwrap();
    }
    let res = RE_REMOVE_LONG_QUOTES.replace_all(&ss, "⫷-$yes1-⫸,--removed_long_quotation--,⫷-$yes2-⫸");
    res.to_string()
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ mod_main
pub fn mod_main() -> Result<(), Box<dyn Error>> {
    // print!("{}🎡𐡋 Checking: s1_exec.rs  \n\n", C_LL);

    let loaded_string1 = fs::read_to_string("/usr/local/sys/rust/data/q0_x_data.csv").expect("mod_main::Read Error");
    let parsed_string1 = remove_short_quotes(loaded_string1.clone());
    fs::write("/usr/local/sys/rust/data/q1_p_data.csv", &parsed_string1).expect("mod_main::Write Parse1 Error");

    let parsed_string2 = remove_long_quotes(loaded_string1);
    fs::write("/usr/local/sys/rust/data/q2_p_data.csv", &parsed_string2).expect("mod_main::Write Parse2 Error");

    let re = Regex::new(r"(?m)^line \d+").unwrap();
    let m = re.find("line one\nline 2\n").unwrap();
    assert_eq!(m.as_str(), "line 2");
    
    let re = Regex::new(r"(?m)^line \d+").unwrap();
    let m = re.find("line one\nline 2\n").unwrap();
    assert_eq!(m.as_str(), "line 2");
    
    Ok(())
    // panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let re = Regex::new(r"(?P<y>\d*)-(?P<m>\d*)-(?P<d>\d*)").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    print!("before: {before}\n");
    print!("after:  {after}\n");

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let re = Regex::new(r"(?m)^line \d+").unwrap();
    let m = re.find("line one\nline 2\n").unwrap();
    assert_eq!(m.as_str(), "line 2");
    
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        static ref RE_REMOVE_SHORT_QUOTES: Regex = Regex::new("(?P<a>......).......(?P<b>.*)").unwrap();
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        static ref RE_REMOVE_SHORT_QUOTES: Regex = Regex::new(r#"(?P<yes1>\^.*)(?P<no>,".*",)(?P<yes2>.*$)"#).unwrap();
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    capture_across_mult_lines(loaded_string1.clone());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    match res {
        Err(ee) => panic!("remove_quoted_fields::RegEx Error: {ee}\n"),
        Ok(ss)  => ss
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ replace aaa with a1a
fn repl_aaa_w_a1a(ss: String) -> String {

    lazy_static! {
     
        static ref RE_REPL_AAA_W_A1A: Regex = Regex::new(

    }
    
}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    fs::write("/usr/local/sys/rust/data/q1_p_data.csv", ss).expect("mod_main::Write Error");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// use unicode_segmentation::UnicodeSegmentation;
// use lazy_static::lazy_static;
// use std::io::prelude::Path;
// use std::fmt;
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

    let ss = read_file("q0_data.csv");      // wherever you are when you run   ∞ cargo run
    print!("File Content: \n{}\n", ss);

    let ss = read_file("../q1_data.csv");   // parent dir of curr clocation when you run   ∞ cargo run
    print!("File Content: \n{}\n", ss);

*/
// End Of The Code Pit

