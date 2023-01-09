//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ a0_aws1.rs  ι․22․05․14✦20․14․47․    

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
                print!("    {}\n", val);
            }
        }
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 Starting: g1_hash_maps_sets.rs \n\n", C_LL);
    
    print!("📚 1. Testing HashMaps with &strs \n\n");

    let mut map0: HashMap<&str, &str> = HashMap::new();
    map0.insert("k_a", "aaaa");
    map0.insert("k_b", "bbbb");
    map0.insert("k_c", "cccc");
    map0.insert("k_d", "dddd");
    map0.insert("k_e", "eeee");
    print!("map0: \n    {:?}\n", map0);
    map0.insert("k_b", "xxxx");
    print!("map0 after insert('k_b', 'xxxx') \n    {:?}\n", map0);
    let val0 = map0.get("ccc");
    print!("getting - val0 = map0.get('ccc'): \n    {:?}\n", val0);

    print!("{}📚 2. Testing HashMaps with Arrays \n\n", C_LL);

    let mut map1: HashMap<&str, [&str; 4]> = HashMap::new();
    map1.insert("k1", ["arr1", "aa11", "bb11", "cc11"]);
    map1.insert("k2", ["arr2", "aa22", "bb22", "cc22"]);
    map1.insert("k3", ["arr3", "aa33", "bb33", "cc33"]);
    let val1 = map1.get("k2");
    print!("map1:         \n    {:?}\n", map1);
    print!("getting - val1 = map1.get('k2'): \n    {:?}\n", val1);
    let arr4 = ["arr4", "aaa44", "bbb44", "ccc44"];
    print!("inserting - arr4 into map1:     \n    {:?}\n", arr4);
    map1.insert("aa4", arr4);
    print!("map1:         \n    {:?}\n", map1);

    print!("{}📚 3. Testing HashMaps with Vectors \n\n", C_LL);

    let mut map2: HashMap<&str, Vec<&str>> = HashMap::new();
    map2.insert("k1", vec!["vec11", "aa11", "bb11", "cc11"]);
    map2.insert("k2", vec!["vec22", "aa22", "bb22", "cc22"]);
    map2.insert("k3", vec!["vec33", "aa33", "bb33", "cc33"]);
    let val2 = map2.get("k3");
    print!("map2: \n    {:?}\n", map2);
    print!("getting - val2 = map2.get('k3'): \n    {:?}\n", val2);

    print!("{}📚 4. Testing HashSets Union Difference Intersection § Symmetric Difference \n\n", C_LL);

    let hs_a:   HashSet<String> = ["aa11", "aa22", "aa33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_b:   HashSet<String> = ["bb11", "bb22", "bb33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_c:   HashSet<String> = ["cc11", "cc22", "cc33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_d:   HashSet<String> = ["dd11", "dd22", "dd33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_big: HashSet<String> = ["aa11", "aa22", "aa33", "bb11", "bb22", "bb33", "cc11", "cc22",
                                   "cc33", "dd11", "dd22", "dd33", "ee11", "ee22", "ee33"].iter().map(|&ss| ss.to_owned()).collect();

    print!("HashSets hs_a:    {:?}\nHashSets hs_b:    {:?}\nHashSets hs_big:  {:?}\n", hs_a, hs_b, hs_big);
    print!("hs_a.union(hs_b): \n    {:?}\n", hs_a.union(&hs_b));

    print!("hs_big.difference(&hs_a): \n    {:?}\n", hs_big.difference(&hs_a));

    print!("Iterating through hs_a.union(&hs_b): \n    ");

    for xx in hs_a.union(&hs_b) {
        print!("{}, ", xx);
    }
    print!("\n\n");

    
    print!("{}📚 6. Getting a vaue from map3, changing it, and updating map with new value \n\n", C_LL);

    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    map3.insert("k_a", hs_a);
    map3.insert("k_b", hs_b);
    map3.insert("k_c", hs_c);
    print!("map3: \n    {:?}\n", map3);

    let hs_e: HashSet<String> = ["ee77", "ee88", "ee99"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_x: HashSet<String> = (hs_e.union(&hs_d)).map(|ss| ss.to_owned()).collect();    
    map3.insert("k_e", hs_e);
    map3.insert("k_x", hs_x);
    print!("map3: \n    {:?}\n", map3);


    print!("Iterating through map3: \n");
    match map3.get("k_c") {
        Some(val) => {
            print!("val: \n    {:?},    ", val);

            let hs_y: HashSet<String> = (val.union(&hs_d)).map(|ss| ss.to_owned()).collect();
            print!("val U hs_d:   {:?}\n", hs_y);
            map3.insert("k_y", hs_y);
        }
        None => {
            print!("key: 'k_c' Not Found\n");
        }
    }
    print!("map3: \n    {:#?}\n", map3);
}



/*λ The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */




