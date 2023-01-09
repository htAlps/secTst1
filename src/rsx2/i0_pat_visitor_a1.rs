//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ i0_pat_visitor_a1.rs Visitor Design Pattern ι✧21․07․23✦12․43․42․   🌎η ✧22․07․28․ ✧22․07․23․    
#![allow(dead_code)]
use std::error::Error;
use alps::*; 

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ mod unit test

//λ test_system
#[cfg(test)]
mod test_system {

    #[test]
    #[should_panic]
    fn fail_test_system() {
        let ok: bool = false;
        assert!(ok);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ mod util  
mod alps {

    //λ type_of returns a description (string) of the type of an object 
    pub fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }


    pub fn pr_type_of<T>(label: &str, _: &T) {
        print!("type of {label} is: <{}>\n", std::any::type_name::<T>())
    }
}


//λ unit tests ─ test_traits
// #[cfg(test)]

// cargo watch -q -c -x run        ‡ -q: quite, -c: clear, -x: ???   
// cargo watch -q -c -x run 


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ mod ip
mod ip {
    use std::fmt;

    #[derive(Debug)]
    pub enum Version {                // 2 basic variants V4 and V6
        V4,
        V6,
    }

    #[derive(Debug)]
    pub struct IpAddrSimpleString {
        pub ver:    IpVer,
        pub addr:   String,
    }

    //  No #Derive Debug so we Implement Display 
    pub enum Form {                   // Using IpForm with all forms of IP Addr   
        IpV4Str(String),
        IpV4Oct(u8, u8, u8, u8),
        IpV6Str(String),
        IpV6Oct(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8),   // 16 octals 
    }

    pub struct IpV4String {
        pub form: IpForm,
        pub ip_addr: String,
    }
    
    pub struct IpV4Octal {
        pub form: IpForm::IpV4Oct,
        pub ip_addr: String,
    }
    
    pub struct IpV6String {
        pub form: IpForm,
        pub ip_addr: String,
    }
    
    pub struct IpV6Octal {
        pub form: IpForm,
        pub ip_addr: String,
    }
    
    #[derive(Debug)]
    pub struct IpByForm {
        pub form:       IpForm,
        pub addr_str:   String,
        pub addr_oct:   (u8, u8, u8, u8),
    }

    #[derive(Debug)]
    pub struct IpAddr {
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

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ mod ast 
mod ast {

    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}



///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    use ip::{IpVer1, IpVer2, IpVer3, IpVer4, IpAddr1};
    print!("{}🎡𐡋 Testing: i0_pat_visitor_a1.rs \n\n", C_LL);

    print!("🎡𐡋 Testing: basic enums \n");

    let _ipv4 = ip::Version::V4;
    let _ipv6 = ip::Version::V6;


    let localhost1 = IpAddrSimpleString {
        ver:    Version::V4,
        addr:   String::from("127.0.0.1"),
    };
    print!("localhost1-IpV4 (:?):    {:?}\n", localhost1);

    let localhost2 = IpAddrSimpleString {
        ver:    Version::V6,
        addr:   String::from("::1/128"),
    }
    print!("localhost2-IpV6 (:?):    {:?}\n", localhost2);                  // pretty pr is not pretty 

    
    let ip1 = IpVer2::V4(String::from("10.17.17.1"));

    print!("ip_addr2:   {:?}\n", ip_addr2);

    let ip_addr3 = IpVer3::V4(10, 17, 19, 2);
    print!("ip_addr3 :?:  {:?}\n", ip_addr3);

    match ip_addr3 {
        IpVer3::V4(o1, o2, o3, o4)  => print!("ip_addr3: {}.{}.{}.{}\n", o1, o2, o3, o4),
        IpVer3::V6(str)             => print!("ip_addr3: {}\n", str),
        // _ => println!("Something else"),                             👎ς Unreachable Pattern  
    }

    let ip_addr4 = IpVer4::V4(10, 47, 49, 42);
    print!("ip_addr4: {}\n", ip_addr4);


    let v1: Vec<i32> = vec![ 1, 2, 3, 5, 8 ];


    print!("🎡𐡋 Testing: iterator consumption: sum \n");
    print!("    v1.iter().sum():  {}\n", v1.iter().sum::<i32>());

    print!("🎡𐡋 Testing: iterator map: mapping e -> e+1 \n");
    let v1_iter = v1.iter().map(|x: &i32| x+1);                         // passive definition 
    let v2: Vec<_> = v1_iter.collect();
    pr_type_of("v2", &v2);
    
    print!("    ");
    for val in v2.iter() {
        print!("{val}, ");
    }
    
    print!("\n    v2.iter().sum():  {}\n", v2.iter().sum::<i32>());

    
    Ok(())
    // panic!("for: No Reason");
    

    Ok(())
    // panic!("for: No Reason");
}



//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

