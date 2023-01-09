//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ c2_cust_tp_enum_a1.rs  ι✧21․08․23✦12․24․31․
#![allow(dead_code)]
use std::error::Error;
use std::fmt;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]
mod parse_test {
    // use chrono::{};

    #[test]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }


}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ mod ip_versions
mod ip_versions {
    use std::fmt;

    #[derive(Debug)]
    pub enum IpVer1 {           // 2 basic variants V4 and V6
        V4,
        V6,
    }

    #[derive(Debug)]
    pub struct IpAddr1 {
        pub ver:    IpVer1,
        pub addr:   String,
    }

    #[derive(Debug)]
    pub enum IpVer2 {           // Better: Store data in each variant
        V4(String),
        V6(String),
    }

    #[derive(Debug)]
    pub enum IpVer3 {           // Better: Store data in each variant
        V4(u8, u8, u8, u8),
        V6(String),
    }

    pub enum IpVer4 {           // Same as 3 but no #Derive Debug so Implement Display
        V4(u8, u8, u8, u8),
        V6(String),
    }

    //  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
    //λ implementations

    impl fmt::Display for IpAddr1 {
        fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
            write!(ff, "{}", self.addr)
        }
    }

    impl fmt::Display for IpVer4 {
        fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
            match self {
                IpVer4::V4(o1, o2, o3, o4)  => write!(ff, "{}.{}.{}.{}", o1, o2, o3, o4),
                IpVer4::V6(str)             => write!(ff, "{}", str),
            }
        }
    }
}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions

// plus_one increments an Option i32
fn plus_one(xx: Option<i32>) -> Option<i32> {
    match xx {
        Some(ii) => Some(ii + 1),
        None => None,
    }
}

// if_three does something but only if it gets Some(3)
fn if_three(xx: Option<i32>) {
    match xx {
        Some(ii) => print!("Doing Something with {ii}\n"),
        _ => (),    // do nothing (cool notation)
    }
}

// if_le_some_three does the same thing but using if let Some
fn if_let_some_three(xx: Option<i32>) {
    if let Some(3) = xx {
        print!("Doing Something\n");
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ check_mod
pub fn check_mod() -> Result<(), Box<dyn Error>> {
    use ip_versions::{IpVer1, IpVer2, IpVer3, IpVer4, IpAddr1};
    print!("{}🎡𐡋 Checking: c2_cust_tp_enum_a1.rs \n\n", C_LL);

    print!("🎡𐡋 Checking: basic enums \n");

    let _ipv4 = ip_versions::IpVer1::V4;
    let _ipv6 = ip_versions::IpVer1::V6;


    let localhost1 = IpAddr1 {
        ver:    IpVer1::V4,
        addr:   String::from("127.0.0.1"),
    };
    print!("localhost1: {}\n", localhost1);

    let localhost2 = IpVer2::V4(String::from("127.0.0.1"));
    print!("localhost2: {:#?}\n", localhost2);

    let ip_addr2 = IpVer2::V4(String::from("10.17.17.1"));

    print!("ip_addr2:   {:?}\n", ip_addr2);

    let ip_addr3 = IpVer3::V4(10, 17, 19, 2);
    print!("ip_addr3 :?:  {:?}\n", ip_addr3);
    print!("ip_addr3 :#?: {:#?}\n", ip_addr3);

    match ip_addr3 {
        IpVer3::V4(o1, o2, o3, o4)  => print!("ip_addr3: {}.{}.{}.{}\n", o1, o2, o3, o4),
        IpVer3::V6(str)             => print!("ip_addr3: {}\n", str),
        // _ => println!("Something else"),     👎ς Unreachable Pattern
    }

    let ip_addr4 = IpVer4::V4(10, 47, 49, 42);
    print!("ip_addr4: {}\n", ip_addr4);

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: enum Option<T> \n\n", C_LL);

    let some_int: Option<i32>   = Some(37);
    let some_str: Option<&str>  = Some("To be or not to be");
    let none_int: Option<i32>   = None;
    print!("some_int: {:?}, some_str: {:?}, none_int: {:?}\n", some_int, some_str, none_int);

    let xx: Option<i32> = Some(13);
    let yy: i32 = 17;
    //  let zz = xx + yy;           // ⮕> 👎ς Fails bec type mismatch
    let zz = xx.unwrap_or(0) + yy;  // using additive identiry
    print!("xx: {:?}, + yy: {:?} = zz: {:?}\n", xx, yy, zz);

    let none_int: Option<i32>   = None;
    let zz = none_int.unwrap_or(0) + yy;  // using additive identiry
    print!("none_int: {:?}, + yy: {:?} = zz: {:?}\n", none_int, yy, zz);

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: match function \n\n", C_LL);
    let xx: Option<i32> = Some(47);
    print!("xx: {:?}, plus_one(xx): {:?}\n", xx, plus_one(xx));
    let none_int: Option<i32>   = None;
    print!("none_int: {:?}, plus_one(none_int): {:?}\n", none_int, plus_one(none_int));


    let xx: Option<i32> = Some(3);
    print!("xx: {:?}, if_three(xx): \n", xx); if_three(xx);
    let xx: Option<i32> = Some(3);
    print!("xx: {:?}, if_let_some_three(xx): \n", xx); if_let_some_three(xx);

    Ok(())
    // panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit


//λ ab_cust_tp_enum_a1.rs
// ✨λ ab_cust_tp_enum_a1.rs
