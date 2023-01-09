//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ b5_trait_iter3.rs
//  ✨λ p5_trait_iter_a3.rs
#![allow(dead_code)]
use std::error::Error;
use std::slice::Iter;
// use core::slice::Iter;          // somehow I dont need this anymore

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
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

    #[test]
    fn test_consume_iter() {   // Test iterator consume string of utf8 chars
        let s1 = String::from("A⼏α");  // , A💎α , A->C:, Bβ0» , B📚β , C🈳ψ , ═══ ");
        let mut s1_iter = s1.chars();
        assert_eq!(s1_iter.next(), Some('A'));
        assert_eq!(s1_iter.next(), Some('⼏'));
        assert_eq!(s1_iter.next(), Some('α'));
        assert_eq!(s1_iter.next(), None);
    }

/*
*/
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ traits § functions


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Testing: p5_trait_iter_a3.rs \n\n", C_LL);

    print!("🎡𐡋 Testing: iterator consume vec of ints \n");
    let v1: Vec<i32> = vec![ 1, 2, 3, 5, 8 ];
    let v1_iter: Iter<i32> = v1.iter();
    for val in v1_iter {
        print!("    {val}\n");
    }

    print!("🎡𐡋 Testing: iterator consume string of utf8 chars \n");
    let s1 = String::from("A⼏α , A💎α , A->C:, Bβ0» , B📚β , C🈳ψ , ═══ ");
    let s1_iter = s1.chars();
    print!("    ");
    for cc in s1_iter {
        print!("{cc}");
    }
    print!("\n\n");

    print!("🎡𐡋 Testing: same as before but with index \n");
    let s1_iter = s1.chars();
    print!("    ");
    for (ii, cc) in s1_iter.enumerate() {
        print!("{ii}-{cc} ");
    }
    print!("\n");



    Ok(())
    // panic!("for: No Reason");

}


//λ The Code Pit
/*

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

*/
// End Of The Code Pit

