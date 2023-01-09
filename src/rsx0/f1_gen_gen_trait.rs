// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ f1_gen_gen_trait.rs  generics § traits    ι✧22․06․22✦07․49․56․  🌎η ✧22․07․04․ ✧22․06․28․ ✧22․06․24․
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


///λ unit tests ─ test_traits
#[cfg(test)]
mod test_traits {
    use super::*;                    // REALLY HARD TO TEST WITHOUT THIS

    #[test]
    fn test_canfoldtr_w_vec1() {
        let vv = Vec1{};             // remember to initialize vv
        assert_eq!(vv.sum(), 13);
        assert_eq!(vv.mult(), 17);
        assert_eq!(vv.count(), 19);
    }
    #[test]
    fn test_canfoldtr_w_vec2() {
        let vv =    Vec2 {
                        sum_val: 20,
                        mult_val: 25,
                        };
        assert_eq!(vv.sum(), 20);
        assert_eq!(vv.mult(), 25);
        assert_eq!(vv.count(), 18);
    }

    // here we test CanFoldTr trait for String (which we obviously don't own!)
    #[test]
    fn test_canfoldtr_w_string() {
        let ss = "Hello World".to_string();
        assert_eq!(ss.sum(), 113);
        assert_eq!(ss.mult(), 117);
    }
    #[test]
    fn test_canfoldtr_print_sum() {
        let vv =    Vec2 {
                        sum_val: 20,
                        mult_val: 25,
                        };
        assert_eq!(vv.sum(), 20);
        assert_eq!(vv.mult(), 25);
        assert_eq!(vv.count(), 18);
    }
}

///λ unit tests ─ test_generics
#[cfg(test)]
mod test_generics {
    use super::*;

    #[test]
    fn test_max_int() {
        assert_eq!(max_int(&vec![4, 3, 5, 4, 7, 4, ]), 7);
        assert_eq!(max_int(&vec![5, 9, 7, 4, 4, 3, ]), 9);
        assert_eq!(max_int(&vec![4, 3, 5, 8, 7, 4, ]), 8);
    }
    #[test]
    fn test_max_char() {
        assert_eq!(max_char(&vec!['a', 'd', 'e', 'b', 'f', 'c',]), 'f',);
        assert_eq!(max_char(&vec!['b', 'f', 'c', 'a', 'g', 'e',]), 'g',);
        assert_eq!(max_char(&vec!['k', 'h', 'a', 'd', 'e', 'b',]), 'k',);
    }
    #[test]
    #[should_panic]             // because fn max_val1 is partial implementation without comparison trait
    fn test_max_val1() {
        assert_eq!(max_val1(&vec![4, 3, 5, 4, 7, 4, ]), 7);
        assert_eq!(max_val1(&vec!['a', 'd', 'e', 'b', 'f', 'c',]), 'f',);
    }
    #[test]
    fn test_max_val2() {
        assert_eq!(max_val2(&vec![5, 9, 7, 4, 4, 3, ]), 9);
        assert_eq!(max_val2(&vec!['b', 'f', 'c', 'a', 'g', 'e',]), 'g',);
    }
    #[test]
    fn test_max_val3() {
        assert_eq!(max_val3(&vec![4, 3, 5, 8, 7, 4, ]), 8);
        assert_eq!(max_val3(&vec!['k', 'h', 'a', 'd', 'e', 'b',]), 'k',);
    }
}

///λ unit tests ─ test_prints
#[cfg(test)]
mod test_prints {
    use super::*;

    #[test]
    fn test_strings1() {
        let s1 = "Hello World".to_string();
        let s2 = "Hello World".to_string();
        assert_eq!(s1, s2);
    }
    #[test]
    fn test_print_sum_vec1() {
        let vv: Vec1 = Vec1{};
        let exp = "print_sum_vec1::vv.sum():  13".to_string();
        let got = print_sum_vec1(vv);
        assert_eq!(exp, got);
    }
    #[test]
    fn test_print_sum_vec2() {
        let vv =    Vec2 {
                        sum_val: 20,
                        mult_val: 25,
                    };
        let exp = "print_sum_vec2::vv.sum():  20".to_string();
        let got = print_sum_vec2(vv);
        assert_eq!(exp, got);
    }
    #[test]
    fn test_print_can_fold_trait() {
        let v1 = Vec1{};

        let got_sum = print_sum_tr(&v1);
        let got_mult = print_mult_tr(&v1);

        let exp_sum = "print_sum_tr::vv.sum():  13".to_string();
        let exp_mult = "print_mult_tr::vv.mult():  17".to_string();

        assert_eq!(exp_sum, got_sum);
        assert_eq!(exp_mult, got_mult);
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// TRAITS

/// CanFoldTr allows folding (collapsing) a list of things using methods sum, mult § count.
/// the methods dont have to be + / * but they have have have some properties e.g., closure w.r.t. <T>
pub trait CanFoldTr {           // any type / struct wanting to satisfy this trait must implement ALL methods

    fn sum(&self) -> u32 {      // we can include *default* fn that can be overriden by the implementing types
        10
    }
    fn mult(&self) -> u32;      // a trait method can have no default implementation

    fn count(&self) -> u32 {
        18
    }
}


/// a simple struct with an implementation of CanFoldTr
#[derive(Clone)]
pub struct Vec1 {}           // next we have this struct and want to satisfy CanFoldTr

impl CanFoldTr for Vec1 {    // so we implement the trait by writing the necesary finctions

    fn sum(&self) -> u32 {      // if I leave this function it overrrides 10 with 13
        13                      // if I delete this function it yields 10
    }
    fn mult(&self) -> u32 {     // if I delete this function => Error bec there is no default
        17
    }
    fn count(&self) -> u32 {    // if I leave this function it overrrides 18 with 19
        19                      // if I delete this function it yields 15
    }
}


// a bit more complicated, keep sum and mult in member variables
struct Vec2 {
    sum_val: u32,
    mult_val: u32,
}

impl CanFoldTr for Vec2 {

    fn sum(&self) -> u32 {
        self.sum_val
    }
    fn mult(&self) -> u32 {
        self.mult_val
    }
}


/// now something strange: implement CanFoldTr for standard Strings which we don't own
impl CanFoldTr for String {
    fn sum(&self) -> u32 {
        113
    }
    fn mult(&self) -> u32 {
        117
    }
    fn count(&self) -> u32 {
        119
    }
}


///λ print_sum_vec1 prints out the sum of ONE data type that implements CanFoldTr
fn  print_sum_vec1 (vv: Vec1) -> String {            // returning the printed string for unit testing
    let ss: String = format!("print_sum_vec1::vv.sum():  {}", vv.sum());
    print!("{}\n", ss);
    ss
}

///λ print_sum_vec2 prints out the sum of ONE data type that implements CanFoldTr
fn  print_sum_vec2(vv: Vec2) -> String {
    let ss = format!("print_sum_vec2::vv.sum():  {}", vv.sum());
    print!("{}\n", ss);
    ss
}

/// the problem above is that we have to duplicate fn: sum and mult for all data types that impl CanFoldTr
/// we can fix that by using syntax bellow

///λ print_sum_tr  prints out the sum of ANY data type that implements CanFoldTr
fn  print_sum_tr(vv: &impl CanFoldTr) -> String {
    let ss = format!("print_sum_tr::vv.sum():  {}", vv.sum());
    print!("{}\n", ss);
    ss
}

///λ print_mult_tr  prints out the sum of ANY data type that implements CanFoldTr
fn  print_mult_tr(vv: &impl CanFoldTr) -> String {
    let ss = format!("print_mult_tr::vv.mult():  {}", vv.mult());
    print!("{}\n", ss);
    ss
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// GENERICS


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Coding the dumb way - re-writing algorithms for each type

///λ max_int find the largest integer in an array of integers
fn  max_int(vv: &[i32]) -> i32 {

    let mut max_val = vv[0];                    // max_val is a value
    for val in vv {                             // this iterator yields the pointers to each element
        if *val > max_val { max_val = *val }    // essentially for strings in C:  while *pp++ {yield pp}
    }
    max_val
}


///λ max_char find the largest character in an array of characters
fn  max_char(vv: &[char]) -> char {

    let mut max_val = &vv[0];                   // but here, max_val is a pointer to char so it matches the iterator yield
    for val in vv {                             // this iterator yields the pointers to each element (like above)
        if val > max_val { max_val = val }      // no need to dereference vv, compiler knows
    }
    *max_val
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Coding the smart way - using GENERICS

///λ max_val1<T> is a partial generic fnction that returns the first element of the array
fn  max_val1<T: Copy>(vv: &[T]) -> T {          // the copy trait is needed here to move value out
    let max_val = &vv[0];                       // because it is owned by vv
    *max_val
}


///λ max_val2<T> is a generic fnction that finds the largest value in an array of T
fn  max_val2<T: std::cmp::PartialOrd + Copy>(vv: &[T]) -> T {
    let mut max_val = &vv[0];
    for val in vv {
        if val > max_val { max_val = val }
    }
    *max_val
}


//⭐λ max_val3<T> is a generic function using the where clause for readability
fn  max_val3<T>(vv: &[T]) -> T
where T: std::cmp::PartialOrd + Copy {      // PartialOrd trait is also needed here to compare values

    let mut max_val = &vv[0];
    for val in vv {
        if val > max_val { max_val = val }
    }
    *max_val
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Next, A struct w conditional Header for specific generic types

// SomeStruct is a generic struct w 2 member variables
struct SomeStruct<T> {
    aa: T,
    bb: T,
}

// this is a Header for impl of SomeStruct with fields of generic type T
impl<T> SomeStruct<T> {

    pub fn new(a: T, b: T) -> Self {
        Self {
            aa: a,
            bb: b,
        }
    }
}

// this is a Header for impl of SomeStruct with fields of generic type T that satisfy Display trait
impl<T> SomeStruct<T>
where T: std::fmt::Display {

    pub fn print_ab(&self) {
        println!("aa: {}, bb: {}", self.aa, self.bb);
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Next, A Blanket Implementation ─ Adding methods to this Header automatically adds them to all concrete types

pub trait PrintValue {
    fn print_value(&self);
}

// this is a Header for generic type T that satisfies Display trait
impl<T> PrintValue for T
where T: std::fmt::Display {

    fn print_value(&self) {
        println!("{}", self);
    }
}



///λ check_mod is an integration tester (int-tester) to check functionality in the development vector (dev-vector)
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 ing n3_gen_gen_trait1.rs \n\n", C_LL);

    let _gen1 = SomeStruct::new(10, 20);      // this type supports Display trait
    let _gen2 = SomeStruct::new("10", "20");  // this type supports Display trait
    let _gen3 = SomeStruct::new([10], [20]);  // this type DOES NOT support Display trait
    let _gen4 = 30;
    let _gen5 = "Hello World";

    _gen1.print_ab();
    // _gen1.print_value();         // ⮕> ⚠️.👎ς error[E0599]: `SomeStruct<{integer}>` doesn't implement `std::fmt::Display`
    _gen2.print_ab();
    // _gen2.print_value();         // ⮕> ⚠️.👎ς error[E0599]: the method `print_value` exists for struct `SomeStruct<&str>`, but its trait bounds were not satisfied
    // _gen3.print_ab();            // ⮕> ⚠️.👎ς error[E0599]: the method `print_ab` exists for struct `SomeStruct<[{integer}; 1]>`, but its trait bounds were not satisfied
    // _gen3.print_value();         // ⮕> ⚠️.👎ς error[E0599]: `SomeStruct<[{integer}; 1]>` doesn't implement `std::fmt::Display`
    _gen4.print_value();
    _gen5.print_value();

    // lets play with our struct
    // let v1: Vec1;                // 👎ς FAILS! error: borrow of possibly-uninitialized variable: `v1`
    let v1: Vec1 = Vec1{};          // 👍υ OK! we have to initialize v1
    let tsum = v1.sum();
    print!("tsum:      {}\n", tsum);
    print!("v1.sum():  {}\n", v1.sum());
    print!("v1.mult(): {}\n", v1.mult());

    let v2: Vec2 =   Vec2 {
                            sum_val: 20,
                            mult_val: 25,
                     };
    print!("v2.sum():  {}\n", v2.sum());
    print!("v2.mult(): {}\n", v2.mult());

    let v3: Vec2 =   Vec2 {
                            sum_val: 30,
                            mult_val: 35,
                     };
    let v4: Vec2 =   Vec2 {
                            sum_val: 40,
                            mult_val: 45,
                     };

    print_sum_tr(&v4);

    print!("\n🎡𐡋 Next, printing mult for all variables (1..4) that implement CanFoldTr \n\n");

    print_sum_vec1(v1.clone());
    print_sum_vec2(v3);
    print_mult_tr(&v1);
    print_mult_tr(&v2);
    // print_mult_tr(&v3);  👎ς  error[E0382]: borrow of moved value: `v3`
    print_mult_tr(&v4);

    Ok(())
    // don't panic!("for: No Reason");
}



//λ The Code Pit
/*

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

*/
// End Of The Code Pit

