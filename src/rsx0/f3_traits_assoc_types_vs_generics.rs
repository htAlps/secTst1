// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ f3_traits_assoc_types_vs_generics.rs  Adv Generics § Traits    ι✧21․11․12✦06․56․11․ 🌎η ✧22․09․16․ ✧22․07․04․ ✧22․06․28․ ✧22․06․24․
#![allow(dead_code)]
use std::error::Error;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ unit tests ─ test_system
#[cfg(test)]
mod test_system {

    #[test]
    #[should_panic]
    fn fail_test_system() {
        let ok: bool = false;
        assert!(ok);
    }
}


pub trait Iterator1 {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter1 {
    cntr:   u32,
}

// Only 1 Implementation of Iterator1 for Counter allowed 
impl Iterator1 for Counter1 {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.cntr += 1;
        Some(self.cntr)
    }
}

///λ unit tests ─ test_traits
#[cfg(test)]
mod test_traits {
    use super::*;                    // REALLY HARD TO TEST WITHOUT THIS

    #[test]
    fn test_add() {
        let vv = Vec1{};             // remember to initialize vv
        assert_eq!(vv.sum(), 13);
        assert_eq!(vv.mult(), 17);
        assert_eq!(vv.count(), 19);
    }
}

// But we Can have multiple iomplementations of Iterator using Generics 
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter2 {
    cntr:   u32,
}

// 1st implementation of Iterator2 for concrete type u32 
impl Iterator2<u32> for Counter2 {
    fn next(&mut self) -> Option<u32> { Some(2) }
}

// 2nd implementation of Iterator2 for concrete type u16 
impl Iterator2<u16> for Counter2 {
    fn next(&mut self) -> Option<u16> { Some(4) }
}

// 3rd implementation of Iterator2 for concrete type u8  
impl Iterator2<u8> for Counter2 {
    fn next(&mut self) -> Option<u8> { Some(6) }
}



///λ check_mod is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    let my_location = "f3_traits_assoc_types_vs_generics::check_mod"; 
    print!("{C_LL}🎡𐡋 {my_location} \n", );

    let mut c1 = Counter1{ cntr: 3 };
    print!("c1.next(): {:?}\n", c1.next());
    print!("c1.next(): {:?}\n", c1.next());

    let mut c2 = Counter2{ cntr: 5 };
    print!("<u32>c2.next(): {:?}\n", <Counter2 as Iterator2<u32>>::next(&mut c2));
    print!("<u16>c2.next(): {:?}\n", <Counter2 as Iterator2<u16>>::next(&mut c2));
    print!("<u8> c2.next(): {:?}\n", <Counter2 as Iterator2<u8>>::next(&mut c2));

    Ok(())
    // don't panic!("for: No Reason");
}



//λ The Code Pit
/*

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    use super::*;                    // REALLY HARD TO TEST WITHOUT THIS

*/
// End Of The Code Pit

