//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ a3_tdd.rs  [ιδ21.12.22 τ08:48:42]

#![allow(dead_code)]
// use std::error;
use std::string::FromUtf8Error;


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants Types § Implementations 

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


struct Rectangle {

    ll: u8,
    ww: u8
}


impl Rectangle {

    fn is_square(&self) -> bool {
        self.ll == self.ww
    }
}


    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }


    struct TestSet1 {
        in1:        Vec<u8>,
        want_val:   Vec<u8>,
    }


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

        let age = 27;
        let peter = Person { name, age };

        // Pretty print
        println!("{:#?}", peter);
    }
*/

/*
    var testSet1 =  []struct {
        in1         []int
        in2         []int
        wantVal     string
        wantErr     string
    }{
        {[]int{1, 2, 1, 4, 7, 6, 3},                       []int{1, 1, 1, 1, 1, 1, 1},          "2325874",            ""},
        {[]int{2, 3, 4, 5, 6, 7, 8},                       []int{1, 1, 1, 1, 1, 1, 1},          "3456789",            ""},
        {[]int{2, 3, 4, 5, 6, 7, 8},                       []int{1, 1, 1, 1, 1, 1, 2},          "4456789",            ""},
        {[]int{1, 2, 3, 4, 5, 6, 7},                       []int{1, 2, 3, 4, 5, 6, 7},          "8888888",            ""},

        {[]int{9, 9, 9, 9, 9, 9, 9},                       []int{1, 1, 1, 1, 1, 1, 1},          "11111110",           ""},
        {[]int{9, 9, 9, 9, 9, 9, 9},                       []int{1, 1, 1, 1, 1},                "10011110",           ""},
        {[]int{9, 9, 9, 9, 9, 9, 9},                       []int{9, 9, 9, 9, 9, 9, 9},          "19999998",           ""},
    }
*/

    

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
// #[cfg(test)]
mod parse_test {
    // use chrono::{};
    #[test]
    fn test_incr_each2() {
        
        struct TestSet1{
            
        let test_set1 = []




    #[test]
    fn test_incr_each1() {

        let par1: Vec<u8> = vec![1,2,3,4,5,6,7];
        let exp1: Vec<u8> = vec![2,3,4,5,6,7,8];
        assert_eq!(exp1, super::incr_each(par1));
    }

    #[test]
    #[should_panic]
    fn test_fail() {
        let ok: bool = false;
        assert!(ok);
    }

    #[test]
    fn test_exp_2() {
        assert_eq!(2i32, super::exp_2());

    }

    Ok(())
    // panic!("for: No Reason");
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions

// loop_to_10 keeps a sum of i1 § i2 as it loops until one of the 2 inputs reaches 10. Then, rerutns the sum. 
fn loop_to_10(mut i1: u8, mut i2: u8) -> u8 {
    let mut sum: u8 = 0;

    loop {
        i1 += 1;
        i2 += 1;
        sum += i1+i2;
        if i1 > 9 || i2 > 9 {break}
    }
    sum
}


fn clean(big_int: Vec<u8>) -> Vec<u8> {

    big_int

}

fn add_each(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    
    let mut tmp1: Vec<u8> = Vec::new();
    let mut iter2 = vec2.iter();
    for val1 in vec1 {
        let val2 = iter2.next().unwrap();
        tmp1.push(val1 + val2);
    }
    tmp1

}


fn incr_each(vec1: Vec<u8>) -> Vec<u8> {

    let mut tmp1: Vec<u8> = Vec::new();
    for val in vec1 {
        tmp1.push(val+1);
    }
    tmp1
}


fn add(big_int1: Vec<u8>, big_int2: Vec<u8>) -> Result<String, FromUtf8Error> {

    let mut tmp1 = big_int1;
    print!("big_int1:   {:?}\n", tmp1);   
    tmp1 = big_int2;

    String::from_utf8(tmp1)

}


fn say_hello(to: &str) {
    print!("hello {}\n", to);
}


fn exp_2() -> i32 {
    2
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 STARTING: a3_tdd.rs  \n\n", C_LL);

    let mut v1: Vec<u8> = vec![1, 2, 3, 4, 5];

    print!("v1:   {:?}\n", v1);   

    v1 = incr_each(v1);
    print!("v1:   {:?}\n", v1);   

    let v2: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
    let v3: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];

    let v4 = add_each(v2, v3);
    print!("v4:   {:?}\n", v4);   

    let sum = loop_to_10(2, 3);
    print!("sum:  {:?}\n", sum);   

    let sum = loop_to_10(1, 9);
    print!("sum:  {:?}\n", sum);   
}


/* THE CODE PIT

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Moved  => c0_..

    For Pretty Print (Not soo pretty) use: {:#?}

    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */
