// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ lib3::q0_env    ι✧21․12․25✦16․50․24․  🌎η ✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․

use std::fmt;
use std::collections::HashSet;
use std::env;

const _USAGE_TABLE: &str = r#"👎ς ERROR: Invalid Command.

 ╔══════════════════════════════════════════════════════════════════════════════════════════╗ 
 ║ β Usage: enter a valid code from this table (codes are lower-case)                       ║ 
 ╠══════╤═════════════════════╤═════════════════════════════════════════════════════════════╣ 
 ║ Code │ Mod/Obj─Type/Func   │ Operation: Description                                      ║ 
 ╠══════╪═════════════════════╪═════════════════════════════════════════════════════════════╣ 
 ║ 2hs+ │ 2/HashSet/`+`       │ Add HashSets:     y2hs.csv <- x2hs1.csv + x2hs2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 2hs- │ 2/HashSet/`-`       │ Subtr HashSets:   y2hs.csv <- x2hs1.csv - x2hs2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 2hm+ │ 2/HashMap/`+`       │ Add HashMaps:     y2hm.csv <- x2hm1.csv + x2hm2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 2hm- │ 2/HashMap/`-`       │ Subtr HashMaps:   y2hm.csv <- x2hm1.csv - x2hm2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 3rmq │ 3/CSV/Clean         │ Clean CSV:        y3clean.csv <- clean(x3dirty.csv)         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 4met │ 4/CSV/Metrics       │ Get Fold Metrics: y4metrics_fold1/2.csv <- fold(x4raw.csv)  ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ chki │ All/Int─Tst/Check   │ Run INT-Tests: recurse through check() fns across all Mods  ║
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ help │ N/A                 │ Print more detail help on the system                        ║
 ╚══════╧═════════════════════╧═════════════════════════════════════════════════════════════╝ 
"#;


const _HELP_TABLE: &str = r#"
 ╔══════════════════════════════════════════════════════════════════════════════════════════════════╗ 
 ║ β HELP:  Below is the list of functions available and a quick description                        ║ 
 ╠══════╤═════════════════════════════════════════════╤═════════════════════════════════════════════╣ 
 ║ Code │           Operation                         │     Description                             ║ 
 ╠══════╪═════════════════════════════════════════════╪═════════════════════════════════════════════╣ 
 ║ 2hs+ │ y2hs.csv <- x2hs1.csv + x2hs2.csv           │ Add 2 HashSets                              ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 2hs- │ y2hs.csv <- x2hs1.csv - x2hs2.csv           │ Subtract 2 HashSets                         ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 2hm+ │ y2hm.csv <- x2hm1.csv + x2hm2.csv           │ Add 2 HashMaps                              ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 2hm- │ y2hm.csv <- x2hm1.csv - x2hm2.csv           │ Subtract 2 HashMaps                         ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 3rmq │ y3clean.csv <- clean(x3dirty.csv)           │ Clean Multiline Fields and Other Stuff      ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 4met │ y4metrics_fold[1/2].csv <- fold(x4raw.csv)  │ Generate Metrics on a Foldable CSV          ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ chki │ recurse through check() functions           │ Runs INT Testing of All Mods and Libs       ║
 ╚══════╧═════════════════════════════════════════════╧═════════════════════════════════════════════╝ 
"#;

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

#[cfg(test)]
mod check_tests {
    use super::*;
    #[test]
    fn test_mod() {
        assert_eq!(check(), Ok(()));
    }
}



/// get_cmd_code returns a code that tells the system what to do 
pub fn get_cmd_code() -> String {

    let codes: HashSet<String> = ["2hs+", "2hs-", "2hm+", "2hm-", "3rmq", "4met", "chki", "help"]
                                 .iter().map(|&ss| ss.to_owned()).collect();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("\n{_USAGE_TABLE} \n");
    }
    print!("🎡𐡋 len(args): {},  ARGS: {:?}\n", args.len(), args[1]); 
    match codes.get(&args[1]) {
        None =>  panic!("\n{_USAGE_TABLE} \n"),
        Some(code) => code.to_string(),
    }
}



/// prt_cmd prints the command line that was invoked to run this
pub fn prt_cmd() -> String {

    let args: Vec<String> = env::args().collect();
    let l = Lex::new();

    let mut res = l.ls1.v();
    for arg in args.iter() {
        res = res + &l.lb1.v() + arg + &l.rb1.v() + &l.sp.v();
    }
    // res = res + &l.rs.v();

    print!("\ncommand issued:   {}", res.clone() + &l.nl.v());
    res
}


///λ check int-tests lib3's q1_lex module 
pub fn check() -> Result<(), String> {

    /*
    */
    let my_location = "lib3::q0_env";
    print!("\n🎡𐡋 {my_location} \n");
    Ok(())                                                              // don't: panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit


