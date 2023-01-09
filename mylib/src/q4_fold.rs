// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ mylib::q4_fold  ι✧21․12․25✦16․50․24․  🌎η ✧23․01․03․✧22․11․16․✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․

extern crate regex;
use regex::Regex;
use lazy_static::lazy_static;
use std::fs;
use std::collections::HashMap;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// Traits, Constants, Types § Enums

/// FMap is a Folding Hashmap with 2 metrics
pub type FMap = HashMap<String, (i32, i32)>;


/// allows folding (collapsing) a list of things using methods like: count, sum, mult, etc.
pub trait Fold<FMap> {

    fn new() -> FMap;
    fn init(ss: String) -> FMap;
    fn fold(&self) -> FMap;

    fn to_csv(&self, header: &str) -> String;
    fn to_table(&self) -> String;                       // data only, no header 
}


impl Fold<FMap> for FMap {

    fn new() -> FMap {
        let myself: HashMap<String, (i32, i32)> = HashMap::new();
        myself
    }
    
    fn init(ss: String) -> FMap {
        let mut myself = FMap::new();
        for line in ss.lines() {
            let trimmed_stl = line.trim();
            match myself.get(trimmed_stl) {
                None => myself.insert(trimmed_stl.to_string(), (0, 1)),
                Some(count) => myself.insert(trimmed_stl.to_string(), (count.0, count.1+1)),
            };
        }
        myself
    }

    fn fold(&self) -> FMap {
        lazy_static! {
            static ref RE_SKIP_COLUMN_CSV: Regex = Regex::new(r",(.*)").unwrap();
        }
        let mut res = FMap::new();
        for (kk, vv) in self {
            let trimmed_key = kk.trim();
            let re_res = RE_SKIP_COLUMN_CSV.captures(&trimmed_key);
            match re_res {
                None => break,
                Some(obj) => {
                    let folded_key = obj.get(1).unwrap().as_str().trim();
                    match res.get(folded_key) {
                        None => res.insert(folded_key.to_string(), (1, vv.1)),
                        Some(val) => res.insert(folded_key.to_string(), (val.0+1, val.1+vv.1)),
                    };
                }
            };
        }
        res
    }

    fn to_csv(&self, header: &str) -> String {
        let cm: String = ",".to_string();
        let nl: String = "\n".to_string();
        let mut res: String = String::new();

        res = res + &header.to_string() + &nl;
        for (kk, vv) in self {
            res = res + &kk + &cm + &vv.0.to_string() + &cm + &vv.1.to_string() + &nl;
        }
        res
    }

    fn to_table(&self) -> String {
        let mut res: String = String::new();

        let cm: String = ",".to_string();
        let nl: String = "\n".to_string();
        for (kk, vv) in self {
            res = res + &kk + &cm + &vv.0.to_string() + &cm + &vv.1.to_string() + &nl;
        }
        res
    }
}


/// fmap_to_csv converts a hash map to a csv with headings in forward (k,v) or reverse (v,k) order
pub fn fmap_to_csv(fwd: bool, header: &str, fmap: &HashMap<String, (i32, i32)>) -> String {

    let mut res: String = String::new();

    let hd: String = header.to_string();
    let cm: String = ",".to_string();
    let nl: String = "\n".to_string();
    if fwd {
        res = res + &hd + &nl;
        for (kk, vv) in fmap {
            res = res + &kk + &cm + &vv.0.to_string() + &cm + &vv.1.to_string() + &nl;
        }
    } else {
        res = res + &hd + &nl;
        for (kk, vv) in fmap {
            res = res + &vv.0.to_string() + &cm + &vv.1.to_string() + &cm + &kk + &nl;
        }
    }
    res
}


/// fmap_to_table converts a hash fmap to a csv with headings in forward (k,v) or reverse (v,k) order
pub fn fmap_to_table(fmap: &HashMap<String, (i32, i32)>) -> String {

    let mut res: String = String::new();

    let cm: String = ",".to_string();
    let nl: String = "\n".to_string();
    for (kk, vv) in fmap {
        res = res + &kk + &cm + &vv.0.to_string() + &cm + &vv.1.to_string() + &nl;
    }
    res
}


///λ fmap_count inputs a csv, multiline string, counts the unique lines and returns a hashmap (key: unique_line, value: count_of_duplicates)
pub fn fmap_count(ss: String) -> HashMap<String, (i32, i32)> {

    let mut res: HashMap<String, (i32, i32)> = HashMap::new();
    for line in ss.lines() {
        match res.get(line) {
            None => res.insert(line.to_string(), (0, 1)),
            Some(count) => res.insert(line.to_string(), (count.0, count.1+1)),
        };
    }
    res
}


///λ fmap_reduce inputs a csv, multiline string, counts the unique lines and returns a hashmap (key: unique_line, value: count_of_duplicates)
pub fn fmap_reduce(hm: HashMap<String, (i32, i32)>) -> HashMap<String, (i32, i32)> {

    let mut res: HashMap<String, (i32, i32)> = HashMap::new();
    for (kk, _) in hm {
        match res.get(&kk) {
            None => res.insert(kk, (0, 1)),
            Some(count) => res.insert(kk, (count.0, count.1+1)),
        };
    }
    res
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ check is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
/// It does the first hashmap iteration of counting duplicates, i.e.:
///  1. reads table from x41_in_data.csv and runs the first hashmapping iteration 
///  2. writes the resulting hashmap to: y41_iter1_res.csv
///  3. removes the last column of the table and writes subtable to: x42_iter1_subtable.csv
pub fn check()  -> Result<String, String> {

    let my_location = "q4_fold::check";
    match fs::read_to_string("/usr/local/sys/sys3rs/data/x41_input_data.csv") {
        Err(ee)       => Err(format!("read_error[{ee}]@{my_location}")),

        Ok(in_string) => {
            let fmap1 = fmap_count(in_string);
            let csv1 = format!("{}\n", fmap_to_csv(true, "col1, col2, cnt1, cnt2", &fmap1));
            
            match fs::write("/usr/local/sys/sys3rs/data/y41_iter1_res.csv", &csv1) {
                Err(ee) => Err(format!("write_fmap_error[{ee}]@{my_location}")),
                _ => {
                    let csv1 = format!("{}\n", fmap_to_table(&fmap1));
                    
                    match fs::write("/usr/local/sys/sys3rs/data/x42_iter1_subtable.csv", &csv1) {
                        Err(ee) => Err(format!("write_key_error[{ee}]@{my_location}")),
                        Ok(()) => Ok(csv1),
                    }
                },
            }
        },
    }
}




//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        
        let res = RE_1_REMOVE_INLINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_inline_quotation--,≺2:$yes2≻");
        res.to_string()
        




    let res = re.captures(stl);
    match res {                                     // Print string [0] and [1]
        Some(obj) => {
            print!("\nres0: {}", obj.get(0).unwrap().as_str().trim());
            print!("\nres1: {}", obj.get(1).unwrap().as_str().trim());
        },
        None => print!("\nNone"),
    }


        
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        lazy_static! {
            static ref RE_1_REMOVE_INLINE_QUOTES: Regex = Regex::new(r#"(?P<yes1>.*)(?P<no>,".*",)(?P<yes2>.*)"#).unwrap();
        }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    fn fold(&self, ss: String) -> Option<&FMap>;
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

    fn fold_stg(mut self, ss: String) -> Option<&FMap> {
        let mut res = FMap::init();
        for line in ss.lines() {
            match self.get(line) {
                None => self.insert(line.to_string(), (0, 1)),
                Some(count) => self.insert(line.to_string(), (count.0, count.1+1)),
            };
        }
        Some(self)
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    fn fold_by_count(&self) -> Option<&FMap>;
    fn fold_by_sum(&self) -> Option<&FMap>;
    fn fold_by_count(self) -> Option<FMap> {

        let res = FMap::init();
        Some(res)
    }



    fn fold_by_sum(self) -> Option<FMap> { 
        let res = FMap::init();
        Some(res)
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    fn fold(&self) -> Option<FMap> {
        let res = FMap::init();
        Some(res)
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ check is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
pub fn check() -> Result<(), String> {

    let my_location = "q4_fold::check";
    print!("\n🎡𐡋 {my_location} \n");
    match fs::read_to_string("/usr/local/sys/sys3rs/data/x32_in_count_duplicates.csv") {
        Err(ee)       => Err(format!("read_error[{ee}]@{my_location}")),
        Ok(in_string) => {
            let fmap1 = fmap_count(in_string);
            print!("fmap1:  {:?}\n", fmap1);
            print!("{}\n", fmap_to_csv(true, "Key1, Key2, cnt1, cnt2", &fmap1));
            Ok(())
        },
    }
    /*
    */
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}




use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

type Result<T> = std::result::Result<T, std::io::Error>;

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

