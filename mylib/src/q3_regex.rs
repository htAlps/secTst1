// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ mylib::q3_regex sysops_exec  ι✧21․11․22✦10․08․26․ 🌎η ✧22․11․12․✧22․10․22․✧22․08․19․✧22․04․21․✧21․12․15․
#![allow(dead_code)]
extern crate regex;
use regex::Regex;
use lazy_static::lazy_static;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`
#[cfg(test)]
mod test_regex {
    // use super::*;

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
//λ RegEx Helper functions

///λ this RegEx Helper removes (," ... ",) within single lines
fn remove_1_inline_quotes(ss: String) -> String {

    lazy_static! {
        static ref RE_1_REMOVE_INLINE_QUOTES: Regex = Regex::new(r#"(?P<yes1>.*)(?P<no>,".*",)(?P<yes2>.*)"#).unwrap();
    }
    let res = RE_1_REMOVE_INLINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_inline_quotation--,≺2:$yes2≻");
    res.to_string()
}


///⭐νλ this RegEx Helper removes (," ... ",) across multiple lines - this one matches \n in entire regex
fn remove_2_multiline_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `(?m)` = multi-line mode
        static ref RE_2_REMOVE_MULTILINE_QUOTES: Regex = Regex::new(r#"((?P<yes1>.*)(?P<no>(?ms:,".*?",))(?P<yes2>.*))"#).unwrap();
    }
    let res = RE_2_REMOVE_MULTILINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_quoted_field--,≺2:$yes2≻");
    res.to_string()
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ RegEx functions

///λ Captures text patterns across multiple lines
fn capture_across_mult_lines(ss: String) {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 q3_regex::capture_across_mult_lines() \n", C_LL);

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


///λ check is the module suppervisor (an integration tester) to check functionality in the development vector (dev-vector)
pub fn check() -> Result<(), String> {

    let my_location = "q3_regex::check";
    print!("\n🎡𐡋 {my_location} \n");

    let string = "London, City of Westminster, Greater London, England, SW1A 2DX, United Kingdom";
    let re = Regex::new(r"^((?:[^,]*,){4})[^,]*,(.*)").unwrap();
    println!("{}", re.replace(string, "$1$2"));

    // => London, City of Westminster, Greater London, England, United Kingdom
    /*
    regex_basics();
    regex_helpers();
    regex_iterations();

    capture_across_mult_lines();
    */

    Ok(())                                                              // don't: panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
pub fn check() -> Result<(), String> {

    let my_location = "q3_regex::check";
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

