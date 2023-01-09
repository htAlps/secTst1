//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ p5_trait_iter_a1.rs
#![allow(dead_code)]
use std::error::Error;
use core::slice::Iter;          // use core::option::Iter; ⮕> 👎ς bec Error: expected struct `std::option::Iter`, found struct `std::slice::Iter`

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests: consumer iterators 
#[cfg(test)]
mod test_suite {
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }

    #[test]
    fn test_iter1() {           // inmutable vector, inmutable expected vars 

        let v1 = vec![ 1, 2, 3, ];
        let mut v1_iter = v1.iter();    // iterators are generally mutable 

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_iter2() {           // inmutable vector, mutable expected vars  

        let v1 = vec![ 1, 2, 3, ];
        let mut v1_iter = v1.iter();
        let mut ii = 1;

        assert_eq!(v1_iter.next(), Some(&ii)); ii += 1;
        assert_eq!(v1_iter.next(), Some(&ii)); ii += 1;
        assert_eq!(v1_iter.next(), Some(&ii));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_mutvec_iter() {     // mutable vector, mutable expected vars  

        let mut v1 = vec![ 1, 2, 3, ];
        let mut v1_iter = v1.iter_mut();
        let mut ii = 1;

        assert_eq!(v1_iter.next(), Some(&mut ii)); ii += 1;
        assert_eq!(v1_iter.next(), Some(&mut ii)); ii += 1;
        assert_eq!(v1_iter.next(), Some(&mut ii));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_into_iter1() {      // inmutable vector, mutable expected vars  

        let v1 = vec![ 1, 2, 3, ];
        let mut v1_iter = v1.into_iter();
        let mut ii = 1;

        assert_eq!(v1_iter.next(), Some(ii)); ii += 1;
        assert_eq!(v1_iter.next(), Some(ii)); ii += 1;
        assert_eq!(v1_iter.next(), Some(ii));
        assert_eq!(v1_iter.next(), None);
    }
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
    print!("{}🎡𐡋 Checking: p5_trait_iter_a1.rs \n\n", C_LL);

    let v1: Vec<i32> = vec![ 1, 2, 3, 5, 8 ];

    let v1_iter: Iter<i32> = v1.iter();

    for val in v1_iter {
        print!("    {val}\n");
    }


    Ok(())
    // panic!("for: No Reason");
    
}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•


//λ The Code Pit
/*

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

*/
// End Of The Code Pit

//λ h6_trait_iter_a1.rs
// ✨λ h6_trait_iter_a1.rs
