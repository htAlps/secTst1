// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ d2_give_and_take.rs - Give § Take  [ιδ22.02.26 τ08:16:30]  

#![allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // To own or not to own 
    // let's make a few strings [mutable]
    let string0: String = "Strings are Mutable!".to_string();
    println!("string0:          {:?}", string0);

    let string1 = string0.clone();
    println!("string1:          {:?}", string1);

    println!("string0:          {:?}", string0);    // 😠 - Oops Compiler got mad  


    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // Let's try with ints 
    let n0 = 13i32;
    println!("n0:               {}", n0);

    let n1 = n0;
    println!("n1:               {}", n1);

    println!("n0:               {}", n0);           // No Problem with primitive types  


    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // let's try with points 
    let point0 = Point { xx: 1, yy: 2, };
    println!("point0:           {:?}", point0);

    let point1 = point0;                            // .clone();
    println!("point1:           {:?}", point1);

    println!("point0:           {:?}", point0);


    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··════•
    // let's try with vectors 
    let mut v1 = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];     // 89. 144, ...
    print!("v1[4..7]:   {:?}\n",  &v1[4..7]);
    v1.pop();

    let v2 = v1.clone();
    print!("v2[4..7]:   {:?}\n",  &v2[4..7]);
    print!("v1[4..7]:   {:?}\n",  &v1[4..7]);        //  👎ς FAIL! value borrowed after move. More Info: `∞ rustc --explain E0382`

/*
    // what about strs [immutable]
    let str0: &str = "Hello World! strs are immutable.";
    println!("str0:             {:?}", str0);

    let str1 = str0;
    println!("str1:             {:?}", str1);

    println!("str0:             {:?}", str0); 


    // To own or not to own 
    // let's make a few strings [mutable]
    let string0: String = "Strings are Mutable!".to_string();
    println!("string0:          {:?}", string0);

    let string1 = string0.clone();
    println!("string1:          {:?}", string1);

    println!("string0:          {:?}", string0);    // 😠 - Oops Compiler got mad  


    // Let's make some ints 
    let n0 = 13i32;
    println!("n0:               {}", n0);

    let n1 = n0;
    println!("n1:               {}", n1);

    println!("n0:               {}", n0);           // No Problem with primitive types  


    // let's make a few points 
    let point0 = Point { xx: 1, yy: 2, };
    println!("point0:           {:?}", point0);

    let point1 = point0.clone();
    println!("point1:           {:?}", point1);

    println!("point0:           {:?}", point0);


    // what about strs [immutable]
    let str0: &str = "strs are immutable Hello World!";
    println!("str0:             {:?}", str0);

    let str1 = str0;
    println!("str1:             {:?}", str1);

    println!("str0:             {:?}", str0); 



    let point2 = Point { xx: 5, yy: 6, };
    let point3 = point0;
*/    

    Ok(())
    // panic!("for: No Reason");
}

