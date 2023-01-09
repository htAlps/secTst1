//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ c2_string_methods.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․12․21․✧22․08․19․ ✧22․04․21․ ✧21․11․07․
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


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ functions  

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

///λ Strings Basics: str, String, new string slices, some ops, borrowing, etc  
fn str_basics() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Strings \n", C_LL);

    let _string1  = String::new();
    let str_lit1  = "hello world: こんにちは世界";  // string slice
    let string2   = str_lit1.to_string();
    let string3   = String::from("emoji: ⛩📕🎯💢📁🔶🌐🔧");
    print!("    str_lit1:   {}\n", str_lit1);
    print!("    string2:    {}\n", string2);
    print!("    string3:    {}\n", string3);

    print!("🎡𐡋 Checking: string OPS (append, +, ) \n");
    let mut ss1  = String::from("foo");
    print!("    ss1:                 {}\n", ss1);
    ss1.push_str("bar");
    print!("    ss1.push_str(`bar`): {}\n", ss1);

    let mut ss1  = String::from("foo");
    let mut ss2  = String::from("bar");
    // ss1.push_str('a');                           // 👎ς expected `char`, found struct `String`
    ss1.push_str(&ss2);
    print!("    ss1.push_str(&ss2): {}\n", ss1);    // 👍υ it's ok bec ss1 did not borrow ss2
    print!("    ss2:                {}\n", ss2);
    ss2.push('!');
    print!("    ss2.push(`!`):      {}\n", ss2);    // ss2 has to be mutable (obviously)

    let ss3      = ss1 + &ss2 + "!";                // ss3 now owns ss1, a copy of ss2 is appended, and `!`
    print!("    ss3 = ss1+&ss2+`!`: {}\n", ss3);
    // print!(" ss1:                {}\n", ss1);    // 👎ς value borrowed here after move
    print!("    ss2:                {}\n", ss2);    // 👍υ Ok because ss2 was not borrowed 


}

///λ Unicode Basics: Runes, Bytes, Scalars, Graphemes, Glyph, Code Points/Units 
fn unicode_basics() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Bytes, Scalars, Graphemes, Glyph, Code Points/Units (Runes) \n", C_LL);

    let ss4 = String::from("⛩📕🎯💢📁🔶 Emojis 🌐🔧");
    print!("🎡𐡋 Iterating through a string by bytes \n");
    print!("    ss4.bytes(): ");
    for val in ss4.bytes() {
        print!("{val} ");
    }
    print!("\n");

    print!("🎡𐡋 Iterating through a string by: chars \n");
    prt_chars(&ss4);

    print!("🎡𐡋 Iterating through a string by: graphemes \n");
    prt_graphemes(&ss4);

    let ss5 = String::from("𐫀𐫁𐫂𐫃𐫄𐫅𐫆𐫇𐫈𐫉𐫊𐫋𐫌𐫍𐫎𐫏𐫐 𐫑 𐫒 𐫓 𐫔 𐫕 𐫖 𐫗𐫘 𐫙 𐫚 𐫛 𐫜 𐫝 𐫞 𐫟 𐫠 𐫡𐫢 𐫣 𐫤𐫫𐫬𐫭𐫮𐫯𐫰 𐫱 𐫲 𐫳𐫴𐫵𐫶");

    print!("🎡𐡋 Iterating through Manichean by: chars \n");
    prt_chars(&ss5);

    print!("🎡𐡋 Iterating through Manichean by: graphemes \n");
    prt_graphemes(&ss5);

    let ss6 = String::from(" a i o u  ä í ö ü ");
    print!("🎡𐡋 Iterating through ` a i o u  ä í ö ü ` by: chars \n");
    prt_chars(&ss6);

    print!("🎡𐡋 Iterating through ` a i o u  ä í ö ü ` by: graphemes \n");
    prt_graphemes(&ss6);

    let ss7 = String::from("  न  म  स्  ते ");
    print!("🎡𐡋 Iterating through `  न  म  स्  ते ` by: chars \n");
    prt_chars(&ss7);

    print!("🎡𐡋 Iterating through ` न  म  स्  ते  ` by: graphemes \n");
    prt_graphemes(&ss7);

}

///λ String_Methods: len, split_whitespace, contains, etc 
fn string_methods_basic() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Basic String Methods  \n", C_LL);

    let ss = String::from("1234567890123");
    print!("String '{}' is {} chars long \n", ss, ss.len());

    // splitting strings 
    let ss = String::from("aaa bb ccc dddd  eee   f   gg");
    print!("    [ ");
    for tok in ss.split_whitespace() {             // split_... returns iterator 
        print!("{tok}, ");
    }
    print!(" ]\n");

    // contains substring ⭐ν Note that Iterator above did not lend ss to the tokens  
    print!("    ss contains dda? {}\n", ss.contains("dda"));
    print!("    ss contains 'b ccc dddd '? {}\n", ss.contains("b ccc dddd "));


    // replace every occurance of pattern 
    print!("    with string: '{ss}' replace 'dddd' with 'xxxx' \n    -> {}\n",
           ss.replace("dddd", "XXXX"));

    let ss = String::from("aaa ddddbb ccc dddd  eee f  g");
    print!("    with string: '{ss}' replace 'dddd' with 'xxxx' \n    -> {}\n",
           ss.replace("dddd", "XXXX"));

    // iterate over lines 
    let ss = String::from("ln1: aa ddd bb ccc  ddd eee f \n\
                            ln2: aa ddd bb ccc  ddd eee f \n\
                            ln3: aa ddd bb ccc  ddd eee f \n");
    print!("\nMultiline String:\n{ss}\n");
    
    print!("🎡𐡋 Iterating over lines \n");
    for line in ss.lines() {
        print!("[  {line}  ]\n");
    }

    print!("\n🎡𐡋 Making a rough JSON Record \n");
    print!("{{ ");
    for line in ss.lines() {
        print!("line: \"{line}\", ");
    }
    print!(" }}\n\n");

    // split by substring: removes substring  
    print!("\n🎡𐡋 Splitting by a Substring \n");
    for ss in ss.split("dd") {
        print!("[  {ss}  ], ");
    }
}

///λ String_Methods_Intermediate: 
fn string_methods_intermed() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Intermediate String Methods  \n", C_LL);

    print!("\n\n🎡𐡋 Collecting an Iteration into a vector of &ss \n");
    let ss = String::from("ln1: aa ddd bb ccc  ddd eee f \n\
                            ln2: aa ddd bb ccc  ddd eee f \n\
                            ln3: aa ddd bb ccc  ddd eee f \n");
    let str_vec: Vec<&str> = ss.split("dd").collect();
    print!("str_vec: {:?}\n\n", str_vec);
    print!("str_vec[2]: {}\n\n",str_vec[2]); 
    print!("str_vec[3]: {}\n\n",str_vec[3]); 

    print!("🎡𐡋 Trimming Strings \n");
    let ss = String::from(" 	a strings with extra chars on both ends  	  \n  \r");
    print!("raw ss: [{}]\n", ss);
    print!("ss.trim(): [{}]\n", ss.trim());

    print!("\n🎡𐡋 Printing the 5th character in a string\n" );
    let ss = String::from("⛩🌐📕💢🎯📁🔶 Emojis 🌐🔧");
    let cc1_option = ss.chars().nth(4);
    let cc2_option = ss.chars().nth(22);

    match cc1_option {
        Some(cc) => print!("ss[4]: {cc}\n"),
        None     => print!("ss[4]: None\n"),
    }
    match cc2_option {
        Some(cc) => print!("ss[22]: {cc}\n"),
        None     => print!("ss[22]: None\n"),
    }
}


///λ String_Methods_Intermediate: 
fn regular_expressions() {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Intermediate String Methods  \n", C_LL);

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";



///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Checking: a82_string_methods.rs \n\n", C_LL);

    str_basics();
    unicode_basics();
    string_methods_basic();
    string_methods_intermed();

    Ok(())
    // panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit



