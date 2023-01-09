//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ g1_hash_maps_sets.rs  

#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};   // no need for self bec no io::BufReader, etc  
use std::path::Path;
use std::collections::{HashMap, HashSet};


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests

#[cfg(test)]
mod load_data_test {
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions: read_lines  print_lines

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {

    let ff = File::open(filename)?;
    Ok(BufReader::new(ff).lines())

}


fn print_lines(results: Result<Lines<BufReader<File>>>) {

    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                println!("    {}", val);
            }
        }
    }
}




///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    print!("{}📚 Starting: g1_hash_maps_sets.rs \n\n", C_LL);
    
    print!("{}📚 Testing HashMaps \n\n", C_LL);
    let mut map1: HashMap<&str, HashSet<String>> = HashMap::new();
    map1.insert("aaa", ["bbbb"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("bbb", ["cccc"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("ccc", ["dddd"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("ddd", ["eeee"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("eee", ["ffff"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("bbb", ["xxxx"].iter().map(|&ss| ss.to_owned()).collect());
    let val1 = map1.get("ccc");
    println!("map1:                   {:?}", map1);
    println!("val1:                   {:?}", val1);

    print!("{}📚 Testing HashMaps with Vectors \n\n", C_LL);
    let mut map2 = HashMap::new();
    map2.insert("aaa", HashSet::from(["top", "aaaaa"]));
    map2.insert("bbb", HashSet::from(["top", "bbbbb"]));
    let val2 = map2.get("bbb");
    println!("map2:                   {:?}", map2);
    println!("val2:                   {:?}", val2);

    print!("{}📚 Testing HashMaps with HashSets \n\n", C_LL);
    let aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
    let bb1: HashSet<String> = ["bbb11", "bbb22", "bbb33"].iter().map(|&ss| ss.to_owned()).collect();
    let cc1: HashSet<String> = ["ccc11", "ccc22", "ccc33"].iter().map(|&ss| ss.to_owned()).collect();
    let dd1: HashSet<String> = ["ddd11", "ddd22", "ddd33"].iter().map(|&ss| ss.to_owned()).collect();
    let ee1: HashSet<String> = ["eee11", "eee22", "eee33"].iter().map(|&ss| ss.to_owned()).collect();
    for xx in aa1.union(&bb1) {
        println!("{}", xx);
    }
    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    println!("map3 (insert some):      {:?}", map3);

    let aa3: HashSet<String> = ["aaa77", "aaa88", "aaa99"].iter().map(|&ss| ss.to_owned()).collect();
    let aa4: HashSet<String> = (aa3.union(&dd1)).map(|ss| ss.to_owned()).collect();    
    map3.insert("aa4", aa4);
    println!("map3:                    {:?}", map3);

    match map3.get("aa1") {
        Some(val) => {
            println!("val                      {:?}", val);
            let aa5: HashSet<String> = (val.union(&ee1)).map(|ss| ss.to_owned()).collect();
            println!("aa2 U aa1:               {:?}", aa5);
            map3.insert("aa1", aa5);
            println!("map3:                    {:?}", map3);
        }
        None => {}
    }

    

    Ok(())
    // panic!("for: No Reason");
}




/*λ The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
let s: String = "abcdefg".to_owned();
let s_slice: &str = &s[..];  // take a full slice of the string

    let mut aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
            //println!("aa2 U aa1:               {:?}", aa2.union(val).into_iter().collect::<HashSet>());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    let mut map3 = HashMap::new();
    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    println!("map3 (insert some):      {:?}", map3);

    let mut aa3 = HashSet::from(["aaa77", "aaa88", "aaa99"]);
    let aa4 = aa3.union(&aa1).collect().to_string();    
    println!("aa3 U aa1                {:?}", aa4);
    map3.insert("aa1", aa4);
    println!("map3 (insert some):      {:?}", map3);

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
use std::io::Read;

    let aa4: HashSet<_> = aa3.union(&aa1).to_string().collect();    
    let diff: HashSet<_> = a.difference(&b).collect();
How to type cast from std::collections::hash_set::Union  to HashSet 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Doesnt Work 
fn read_lines<P>(filename: &dyn AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>> {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
where P: AsRef<Result<Lines<BufReader<File>>>>, {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                println!("    {}", val);
            }
        }
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
#[derive(debug)]
struct One2many {
    one: 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Try all of these in the following example:

    use std::collections::HashSet;

    fn main() {
        let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
        let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

        assert!(a.insert(4));
        assert!(a.contains(&4));

        // `HashSet::insert()` returns false if there was a value already present.
        assert!(b.insert(4), "Value 4 is already in set B!");
        // FIXME ^ Comment out this line

        b.insert(5);

        // If a collection's element type implements `Debug`,
        // then the collection implements `Debug`.
        // It usually prints its elements in the format `[elem1, elem2, ...]`
        println!("A: {:?}", a);
        println!("B: {:?}", b);

        // Print [1, 2, 3, 4, 5] in arbitrary order
        println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

        // This should print [1]
        println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

        // Print [2, 3, 4] in arbitrary order.
        println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

        // Print [1, 5]
        println!("Symmetric Difference: {:?}",
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1 = HashSet::from(["aaa11", "aaa22", "aaa33"]);
    let mut bb1 = HashSet::from(["bbb11", "bbb22", "bbb33"]);
    let mut cc1 = HashSet::from(["ccc11", "ccc22", "ccc33"]);

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1: HashSet<String> = vec!["aaa11", "aaa22", "aaa33"].into_iter().collect();
    let mut bb1: HashSet<String> = vec!["bbb11", "bbb22", "bbb33"].into_iter().collect();
    let mut cc1: HashSet<String> = vec!["ccc11", "ccc22", "ccc33"].into_iter().collect();
    
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
    let mut bb1: HashSet<String> = ["bbb11", "bbb22", "bbb33"].iter().map(|&ss| ss.to_owned()).collect();
    let mut cc1: HashSet<String> = ["ccc11", "ccc22", "ccc33"].iter().map(|&ss| ss.to_owned()).collect();
    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
use std::collections::HashSet;
let a = HashSet::from([1, 2, 3]);
let b = HashSet::from([4, 2, 3, 4]);

// Print 1, 2, 3, 4 in arbitrary order.
for x in a.union(&b) {
    println!("{}", x);
}

let union: HashSet<_> = a.union(&b).collect();
assert_eq!(union, [1, 2, 3, 4].iter().collect());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    println!("map3 (empty):            {:?}", map3);

    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    println!("map3 (insert some):      {:?}", map3);

    let mut aa2: HashSet<String> = ["aaaa77", "aaaa88"].iter().map(|&ss| ss.to_owned()).collect();
    println!("map3 (insert some):      {:?}", map3);
    aa2.union(&aa1);    
    println!("aa2 U aa1                {:?}", map3);
    
    match map3.get("aa1") {
        Some(val) => {
            
            println!("val                      {:?}", val);
            println!("aa2 U aa1:               {:?}", aa2);
            //println!("aa2 U aa1:               {:?}", aa2.union(val).into_iter().collect::<HashSet>());
        }
        None => {}
    }

   // collect::<Vec<&i32>>() 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */



