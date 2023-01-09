// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ d0_python_and_rust0.rs - Give § Take  [ιδ22.02.26 τ08:16:30]  


#![allow(dead_code)]
pub fn get_2() -> i32 {
    2
}


pub fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}

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


    // To own or not to own 
    // Let's make some ints 
    let n0 = 13i32;
    println!("n0:               {}", n0);

    let n1 = n0;
    println!("n1:               {}", n1);

    println!("n0:               {}", n0);           // No Problem Here 


    // let's make a few points 
    let point0 = Point { xx: 1, yy: 2, };
    println!("point0:           {:?}", point0);

    let point1 = point0.clone();
    println!("point1:           {:?}", point1);

    println!("point0:           {:?}", point0);


    // let's make a few strings [mutable]
    let string0: String = "Strings are Mutable!".to_string();
    println!("string0:          {:?}", string0);

    let string1 = string0.clone();
    println!("string1:          {:?}", string1);

    println!("string0:          {:?}", string0);    // 😠 - Oops Compiler got mad  


    // what about strs [immutable]
    let str0: &str = "strs are immutable Hello World!";
    println!("str0:             {:?}", str0);

    let str1 = str0;
    println!("str1:             {:?}", str1);

    println!("str0:             {:?}", str0); 

/*
    let point2 = Point { xx: 5, yy: 6, };
    let point3 = point0;
*/    

    Ok(())
    // panic!("for: No Reason");
}

