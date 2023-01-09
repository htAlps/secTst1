// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b7-structs-traits-enums.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.02] [δ22.01.01]

#![allow(dead_code, unused_variables)]
// #[derive(Debug, Clone, Copy)]
// use std::io::stdin;

const C_LL:     &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

// C-Style Structs
struct Frame {
    num1:       i32,
    num2:       i32,
    str1:       String,
    opt_num:    Option<i32>
}

impl Frame {
    fn new() -> Self {
        Frame {
            num1: 0,
            num2: 0,
            str1: "dot".to_string(),
            opt_num: None
        }
    }

    fn area(&self) -> i32 {
        self.num1 * self.num2
    }
}


// Tupple Structs
struct Trio (i32, i32, String);

impl Trio {
    fn new() -> Self {
        Trio(0, 0, "dot".to_string())
    }
}



// Units Structs
struct HasArea;






///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    print!("{}📚 STARTING: b7-structs-traits1.rs  \n\n", C_LL);

    print!("📚 Testing C-Structs \n\n");
    let frame1 = Frame {
        num1: 3,
        num2: 5,
        str1: "rectangle".to_string(),
        opt_num: None
    };
    println!("frame1.area():        {}", frame1.area());

    let frame2 = Frame::new();
    println!("frame2.area():        {}", frame2.area());

    let trio = Trio (7, 0, "circle".to_string());
    println!("trio.0: {}    trio.2: {}", trio.0, trio.2);

    let trio = Trio::new();


    println!("trio.0: {}    trio.2: {}", trio.0, trio.2);

    print!("done!\n\n");

    Ok(())
    // panic!("for: No Reason");
}


