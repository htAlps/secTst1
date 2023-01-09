//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ c1_collections2strings.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․08․19․ ✧22․04․21․ ✧21․11․07․
#![allow(dead_code)]
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;
// use std::fmt;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests
#[cfg(test)]
mod fail_test {
    // use chrono::{};
    #[test]
    #[should_panic]
    fn test_fail() {
        let ok: bool = false;
        assert!(ok);
    }
}

fn prt_chars(ss: &String) {
    print!("    ss.chars(): ");
    for val in ss.chars() {
        print!("{val} ");
    }
    print!("\n\n");
}

fn prt_graphemes(ss: &String) {
    print!("    ss.graphemes(): ");
    for val in ss.graphemes(true) {
        print!("{val} ");
    }
    print!("\n\n");
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Testing: a81_collections2strings.rs \n\n", C_LL);


    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Testing: Strings \n", C_LL);

    let _string1  = String::new();
    let str_lit1  = "hello world: こんにちは世界";  // string slice
    let string2   = str_lit1.to_string();
    let string3   = String::from("emoji: ⛩📕🎯💢📁🔶🌐🔧");
    print!("    str_lit1:   {}\n", str_lit1);
    print!("    string2:    {}\n", string2);
    print!("    string3:    {}\n", string3);

    print!("🎡𐡋 Testing: string OPS (append, +, ) \n");
    let mut s1  = String::from("foo");
    print!("    s1:                 {}\n", s1);
    s1.push_str("bar");
    print!("    s1.push_str(`bar`): {}\n", s1);


    let mut s1  = String::from("foo");
    let mut s2  = String::from("bar");
    s1.push_str(&s2);                               // 👎ς expected `char`, found struct `String`
    print!("    s1.push_str(&s2):   {}\n", s1);
    print!("    s2:                 {}\n", s2);     // it's ok bec s1 did not borrow s2
    s2.push('!');
    print!("    s2.push(`!`):       {}\n", s2);     // s2 has to be mutable (obviously)

    let s3      = s1 + &s2 + "!";
    print!("    s3 = s1+&s2+`!`:    {}\n", s3);     // s3 now owns s2, a copy of s1 is appended, and also `!`
    // print!(" s1:                 {}\n", s1);     // 👎ς value borrowed here after move

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Testing: Bytes, Scalars, Graphemes, Glyph, Code Points/Units (Runes) \n", C_LL);

    let s4      = String::from("⛩📕🎯💢📁🔶🌐🔧");
    print!("🎡𐡋 Iterating through a string by bytes \n");
    print!("    s4.bytes(): ");
    for val in s4.bytes() {
        print!("{val} ");
    }
    print!("\n");

    print!("🎡𐡋 Iterating through a string by: chars \n");
    prt_chars(&s4);

    print!("🎡𐡋 Iterating through a string by: graphemes \n");
    prt_graphemes(&s4);

    let s5 = String::from("𐫀𐫁𐫂𐫃𐫄𐫅𐫆𐫇𐫈𐫉𐫊𐫋𐫌𐫍𐫎𐫏𐫐 𐫑 𐫒 𐫓 𐫔 𐫕 𐫖 𐫗𐫘 𐫙 𐫚 𐫛 𐫜 𐫝 𐫞 𐫟 𐫠 𐫡𐫢 𐫣 𐫤𐫫𐫬𐫭𐫮𐫯𐫰 𐫱 𐫲 𐫳𐫴𐫵𐫶");

    print!("🎡𐡋 Iterating through Manichean by: chars \n");
    prt_chars(&s5);

    print!("🎡𐡋 Iterating through Manichean by: graphemes \n");
    prt_graphemes(&s5);

    let s6 = String::from(" a i o u  ä í ö ü ");
    print!("🎡𐡋 Iterating through ` a i o u  ä í ö ü ` by: chars \n");
    prt_chars(&s6);

    print!("🎡𐡋 Iterating through ` a i o u  ä í ö ü ` by: graphemes \n");
    prt_graphemes(&s6);

    let s7 = String::from("  न  म  स्  ते ");
    print!("🎡𐡋 Iterating through `  न  म  स्  ते ` by: chars \n");
    prt_chars(&s7);

    print!("🎡𐡋 Iterating through ` न  म  स्  ते  ` by: graphemes \n");
    prt_graphemes(&s7);

    Ok(())
    // panic!("for: No Reason");
}



//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

