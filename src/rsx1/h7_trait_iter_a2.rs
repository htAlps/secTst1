//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ p5_trait_iter_a2.rs
#![allow(dead_code)]
use std::error::Error;
// use core::slice::Iter;          // somehow I dont need this anymore 

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests: consumer § adaptor iterators 
#[cfg(test)]
mod test_suite {
    use std::slice::Iter;
    use std::iter::Map;
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }

    #[test]
    fn test_iter_sum1() {      // iterator consumption - detailed version 

        let v1 = vec![ 1, 2, 3, ];
        let v1_iter = v1.iter();
        let got_sum: i32 = v1_iter.sum(); 
        let exp_sum = 6;
        assert_eq!(got_sum, exp_sum);
    }

    #[test]
    fn test_iter_sum2() {               // iterator consumption - short version of above 

        let v1 = vec![ 1, 2, 3, ];
        let exp_sum = 6;
        assert_eq!(v1.iter().sum::<i32>(), exp_sum);
    }

    #[test]
    fn test_iter_map_collect_sum1() {   // iterator map -> collect -> consume - very detailed version 

        let v1:             Vec<i32>    = vec![ 1, 2, 3, ];
        let v1_iter:        Iter<_>     = v1.iter();
        let v1_iter_map:    Map<_, _>   = v1_iter.map(|x: &i32| x+1); 
        let v2:             Vec<_>      = v1_iter_map.collect();
        let v2_iter:        Iter<_>     = v2.iter();
        let got_sum:        i32         = v2_iter.sum();

        let exp_sum = 9;
        assert_eq!(got_sum, exp_sum);
    }

    #[test]
    fn test_iter_map_collect_sum2() {   // iterator map -> collect -> consume - detailed version 

        let v1          = vec![ 1, 2, 3, ];
        let v1_iter     = v1.iter();
        let v1_iter_map = v1_iter.map(|x: &i32| x+1); 
        let v2          = v1_iter_map.collect::<Vec<i32>>();
        let v2_iter     = v2.iter();
        let got_sum     = v2_iter.sum::<i32>();

        let exp_sum = 9;
        assert_eq!(got_sum, exp_sum);
    }

    #[test]
    fn test_iter_map_collect_sum3() {   // iterator map -> collect -> consume - less detailed version 

        let v1 = vec![ 1, 2, 3, ];
        let v1_iter = v1.iter().map(|x: &i32| x+1); 
        let v2: Vec<_> = v1_iter.collect();
        let got_sum: i32 = v2.iter().sum();

        let exp_sum = 9;
        assert_eq!(got_sum, exp_sum);
    }

    #[test]
    fn test_iter_map_collect_sum4() {   // iterator map -> collect -> consume - short version 

        let v1 = vec![ 1, 2, 3, ];
        let got_sum: i32 = v1.iter().map(|x: &i32| x+1).collect::<Vec<_>>().iter().sum::<i32>();

        let exp_sum = 9;
        assert_eq!(got_sum, exp_sum);
    }
/*
*/
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ traits § functions

//λ Iterator trait  
pub trait Iterator {
    type Item;                                      // type Item is an associated type (ch 19)

    fn next(&mut self) -> Option<Self::Item>;       // self must be mutable bec next changes the internal structure of the iterator 

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ check_mod
pub fn check_mod() -> Result<(), Box<dyn Error>> {
    print!("{}🎡𐡋 Checking: p5_trait_iter_a2.rs \n\n", C_LL);

    let v1: Vec<i32> = vec![ 1, 2, 3, 5, 8 ];


    print!("🎡𐡋 Checking: iterator consumption: sum \n");
    print!("    v1.iter().sum():  {}\n", v1.iter().sum::<i32>());

    print!("🎡𐡋 Checking: iterator map: mapping e -> e+1 \n");
    let v1_iter = v1.iter().map(|x: &i32| x+1);     // passive definition 
    let v2: Vec<_> = v1_iter.collect();
    
    for val in v2.iter() {
        print!("    {val}\n");
    }
    
    print!("    v2.iter().sum():  {}\n", v2.iter().sum::<i32>());

    
    Ok(())
    // panic!("for: No Reason");
    
}


//λ The Code Pit
/*

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

*/
// End Of The Code Pit

//λ h7_trait_iter_a2.rs
// ✨λ h7_trait_iter_a2.rs
