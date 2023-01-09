// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ d3_borrow_or_clone.rs - Borrow or Clone - [ιδ22.03.16 τ20:15:43]   


#![allow(dead_code)]
pub fn get_2() -> i32 {
    2
}


pub fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}

#[derive(Debug, Clone)]     // , Copy)]
pub struct Point{

    pub xx: i32,
    pub yy: i32
}


impl Point {

    pub fn has_slope_1(&self) -> bool {
        self.xx == self.yy
    }

    pub fn extend(&self, p2: Point) -> Point {
        let res = Point { 
            xx: self.xx + p2.xx,
            yy: self.yy + p2.yy,
        };
        return res;
    }
}

pub fn give_alice(a_point: Point) {
    println!("Alice::Owns a_point:      {:?}", a_point);
}

pub fn give_bob(a_point: Point) {
    println!("Bob::Owns a_point:        {:?}", a_point);
}

pub fn lend_alice(a_point: Point) -> Point {
    println!("Alice::Borrowing a_point: {:?}", a_point);
    a_point
}


pub fn lend_bob(a_point: Point) -> Point {
    println!("Bon::Borrowing a_point:   {:?}", a_point);
    a_point
}


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // 3. let's try lending make a few points       👍υ OK! 
    let mut point0 = Point { xx: 1, yy: 2, };
    point0 = lend_alice(point0);
    point0 = lend_bob(point0);
    give_alice(point0);
    

/*
    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // 1. let's make a few points                   👎ς FAILS!  
    let mut point0 = Point { xx: 1, yy: 2, };
    give_alice(point0);
    give_bob(point0);
    

    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // 2. let's try it this way                     👍υ OK! 
    let point0 = Point { xx: 1, yy: 2, };
    let point1 = point0;                    // .clone();
    give_alice(point0);
    give_bob(point1);
    

    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // 3. let's try lending points                  👍υ OK! 
    let mut point0 = Point { xx: 1, yy: 2, };
    point0 = lend_alice(point0);
    point0 = lend_bob(point0);
    give_alice(point0);
    

    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // 4. let's try again deriving copy trait       👍υ OK! 
    let mut point0 = Point { xx: 1, yy: 2, };
    give_alice(point0);
    give_bob(point0);
    

    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // what about strs [immutable]
    let str0: &str = "strs are immutable Hello World!";
    println!("str0:             {:?}", str0);

    let str1 = str0;
    println!("str1:             {:?}", str1);

    println!("str0:             {:?}", str0); 

    let point2 = Point { xx: 5, yy: 6, };
    let point3 = point0;

    
    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // To borrow or clone 
    // let's try with strings [mutable]
    let string0: String = "Strings are Mutable!".to_string();
    println!("string0:          {:?}", string0);

    let string1 = string0.clone();
    println!("string1:          {:?}", string1);

    println!("string0:          {:?}", string0);            // 😠 - Oops Compiler got mad  

    
*/    

    Ok(())
    // panic!("for: No Reason");
}

