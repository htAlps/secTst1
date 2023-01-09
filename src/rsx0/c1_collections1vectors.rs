//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ c1_collections1vectors.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․08․19․ ✧22․04․21․ ✧21․11․07․
#![allow(dead_code)]
use std::error::Error;
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


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Testing: a81_collections1vectors.rs \n\n", C_LL);

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("🎡𐡋 Testing: arrays and vectors \n");

    let a1 = [0, 1, 1];
    let mut v1: Vec<i32> = Vec::new();
    v1.push(2); v1.push(3); v1.push(5); v1.push(8);
    print!("    a1: {:?}\n", a1);
    print!("    v1: {:?}\n", v1);
    {
        print!("    🎡𐡋 Testing: in sub-scopes \n");
        let a1 = [6, 5, 4];
        let mut v1: Vec<i32> = Vec::new();
        v1.push(3); v1.push(2); v1.push(1); v1.push(0);
        print!("        a1: {:?}\n", a1);
        print!("        v1: {:?}\n", v1);
    }
    print!("🎡𐡋 Back to main scope (sub-scope data is dropped)\n");
    print!("    a1: {:?}\n", a1);
    print!("    v1: {:?}\n", v1);

    print!("🎡𐡋 Testing: accessing of individual elements \n");
    print!("    &a1[0]: {}\n", &a1[0]);
    print!("    &v1[1]: {}\n", &v1[1]);

    print!("🎡𐡋 Testing: out-of-bound runtime errors \n");      // Arrays should give a compile time error but it's not
    // print!("&a1[20]: {}\n", &a1[20]);                        // 👎ς thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 20'
    // print!("&v1[20]: {}\n", &v1[20]);                        // 👎ς both arrays and vectors are giving run-time errors

    print!("🎡𐡋 only access elements directly when you expect the program to crash if the index is invalis \n");
    print!("🎡𐡋 otherwise use get() -> enum Option \n");
    print!("    v1.get(2):  {:?}\n", v1.get(2));
    print!("    v1.get(20): {:?}\n", v1.get(20));

    let third_elem = v1.get(3);
    match third_elem {
        Some(ii) => print!("    third_elem matches  Some(ii):  {ii}\n"),
        None     => print!("    third_elem matches  None.\n"),
    }
    match v1.get(20) {
        Some(ii) => print!("    v1.get(20) matches  Some(ii):  {ii}\n"),
        None     => print!("    v1.get(20) matches  None.\n"),
    }

    print!("🎡𐡋 Testing: iterating through a vector \n");       // ❓ Arrays should give a compile time error but it's not
    print!("    v1: ");
    for val in v1 {
        print!("{val} ");
    }
    print!("\n");
    print!("🎡𐡋 Adding 100 to each element \n");
    let mut v1: Vec<i32> = Vec::new();
    v1.push(2); v1.push(3); v1.push(5); v1.push(8);
    print!("    v1: {:?}\n", v1);
    for pp in &mut v1 {
        *pp += 100;
    }
    print!("    v1: {:?}\n\n", v1);

    let mut v2 = vec![101, 102, 103, 104];
    print!("    v2: {:?}\n", v2);
    for pp in &mut v2 {
        *pp += 100;
    }
    print!("    v2: {:?}\n", v2);

    Ok(())
    // panic!("for: No Reason");
}



//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("🎡𐡋 Testing: accessing slices of elements \n");
    print!("&a1[0:1]: {}\n", &a1[0:1]);
    print!("&v1[1:3]: {}\n", &v1[1:3]);
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

*/
// End Of The Code Pit

