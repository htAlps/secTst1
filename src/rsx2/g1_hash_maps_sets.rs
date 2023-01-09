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
                print!("    {}\n", val);
            }
        }
    }
}




///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

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

    Ok(())
    // panic!("for: No Reason");
}




/*λ The Code Pit

        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        print!("A: {:?}\n", a);
        print!("B: {:?}\n", b);

        // Print [1, 2, 3, 4, 5] in arbitrary order
        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());

        // This should print [1]
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());

        // Print [2, 3, 4] in arbitrary order.
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());

        // Print [1, 5]
        print!("Symmetric Difference: {:?}\n",
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());

        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    map2.insert("bb1", HashSet::from(vec!(["top", "aaa11", "bbb11", "ccc11"])));
    map2.insert("bb2", HashSet::from(vec!(["top", "aaa22", "bbb22", "ccc22"])));
    map2.insert("bb3", HashSet::from(vec!(["top", "aaa33", "bbb33", "ccc33"])));

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
let s: String = "abcdefg".to_owned();
let s_slice: &str = &s[..];  // take a full slice of the string

    let mut aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
            //print!("aa2 U aa1:    {:?}\n", aa2.union(val).into_iter().collect::<HashSet>());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    let mut map3 = HashMap::new();
    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    print!("map3 (insert some):      {:?}\n", map3);

    let mut aa3 = HashSet::from(["aaa77", "aaa88", "aaa99"]);
    let aa4 = aa3.union(&aa1).collect().to_string();    
    print!("aa3 U aa1                {:?}\n", aa4);
    map3.insert("aa1", aa4);
    print!("map3 (insert some):      {:?}\n", map3);

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
                print!("    {}\n", val);
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
        print!("A: {:?}\n", a);
        print!("B: {:?}\n", b);

        // Print [1, 2, 3, 4, 5] in arbitrary order
        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());

        // This should print [1]
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());

        // Print [2, 3, 4] in arbitrary order.
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());

        // Print [1, 5]
        print!("Symmetric Difference: {:?}\n",
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
    print!("{}\n", x);
}

let union: HashSet<_> = a.union(&b).collect();
assert_eq!(union, [1, 2, 3, 4].iter().collect());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    print!("map3 (empty):            {:?}\n", map3);

    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    print!("map3 (insert some):      {:?}\n", map3);

    let mut aa2: HashSet<String> = ["aaaa77", "aaaa88"].iter().map(|&ss| ss.to_owned()).collect();
    print!("map3 (insert some):      {:?}\n", map3);
    aa2.union(&aa1);    
    print!("aa2 U aa1                {:?}\n", map3);
    
    match map3.get("aa1") {
        Some(val) => {
            
            print!("val                      {:?}\n", val);
            print!("aa2 U aa1:               {:?}\n", aa2);
            //print!("aa2 U aa1:               {:?}\n", aa2.union(val).into_iter().collect::<HashSet>());
        }
        None => {}
    }

   // collect::<Vec<&i32>>() 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
A variable already borrowed as immutable was borrowed as mutable.

Erroneous code example:

```
fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    let y = &a; // a is borrowed as immutable.
    bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed
            //        as immutable
    print!("{}\n", y);
}
```

To fix this error, ensure that you don't have any other references to the
variable before trying to access it mutably:

```
fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    bar(a);
    let y = &a; // ok!
    print!("{}\n", y);
}
```

For more information on Rust's ownership system, take a look at the
[References & Borrowing][references-and-borrowing] section of the Book.

[references-and-borrowing]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
~
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
rustc --explain E0502 | A variable already borrowed as immutable was borrowed as mutable.

Erroneous code example:

    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        let y = &a;         // a is borrowed as immutable.
        bar(a);             // error: cannot borrow `*a` as mutable because `a` is also borrowed as immutable
        print!("{}\n", y);
    }

To fix this error, ensure that you don't have any other references to the
variable before trying to access it mutably:

    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        bar(a);
        let y = &a; // ok!
        print!("{}\n", y);
    }

For more information on Rust's ownership system, take a look at the
[References & Borrowing][references-and-borrowing] section of the Book.

[references-and-borrowing]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
~
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */



