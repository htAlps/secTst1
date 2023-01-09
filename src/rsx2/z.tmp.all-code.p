η5»ξ a0_aws1.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ a0_aws1.rs  ι․22․05․14✦20․14․47․    

#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};   // no need for self bec no io::BufReader, etc  
use std::path::Path;
use std::collections::{HashMap, HashSet};


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests

#[cfg(test)]
mod load_data_test {
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions: read_lines  print_lines

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {

    let ff = File::open(filename)?;
    Ok(BufReader::new(ff).lines())

}


fn print_lines(results: Result<Lines<BufReader<File>>>) {

    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                print!("    {}\n", val);
            }
        }
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 Starting: g1_hash_maps_sets.rs \n\n", C_LL);
    
    print!("📚 1. Testing HashMaps with &strs \n\n");

    let mut map0: HashMap<&str, &str> = HashMap::new();
    map0.insert("k_a", "aaaa");
    map0.insert("k_b", "bbbb");
    map0.insert("k_c", "cccc");
    map0.insert("k_d", "dddd");
    map0.insert("k_e", "eeee");
    print!("map0: \n    {:?}\n", map0);
    map0.insert("k_b", "xxxx");
    print!("map0 after insert('k_b', 'xxxx') \n    {:?}\n", map0);
    let val0 = map0.get("ccc");
    print!("getting - val0 = map0.get('ccc'): \n    {:?}\n", val0);

    print!("{}📚 2. Testing HashMaps with Arrays \n\n", C_LL);

    let mut map1: HashMap<&str, [&str; 4]> = HashMap::new();
    map1.insert("k1", ["arr1", "aa11", "bb11", "cc11"]);
    map1.insert("k2", ["arr2", "aa22", "bb22", "cc22"]);
    map1.insert("k3", ["arr3", "aa33", "bb33", "cc33"]);
    let val1 = map1.get("k2");
    print!("map1:         \n    {:?}\n", map1);
    print!("getting - val1 = map1.get('k2'): \n    {:?}\n", val1);
    let arr4 = ["arr4", "aaa44", "bbb44", "ccc44"];
    print!("inserting - arr4 into map1:     \n    {:?}\n", arr4);
    map1.insert("aa4", arr4);
    print!("map1:         \n    {:?}\n", map1);

    print!("{}📚 3. Testing HashMaps with Vectors \n\n", C_LL);

    let mut map2: HashMap<&str, Vec<&str>> = HashMap::new();
    map2.insert("k1", vec!["vec11", "aa11", "bb11", "cc11"]);
    map2.insert("k2", vec!["vec22", "aa22", "bb22", "cc22"]);
    map2.insert("k3", vec!["vec33", "aa33", "bb33", "cc33"]);
    let val2 = map2.get("k3");
    print!("map2: \n    {:?}\n", map2);
    print!("getting - val2 = map2.get('k3'): \n    {:?}\n", val2);

    print!("{}📚 4. Testing HashSets Union Difference Intersection § Symmetric Difference \n\n", C_LL);

    let hs_a:   HashSet<String> = ["aa11", "aa22", "aa33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_b:   HashSet<String> = ["bb11", "bb22", "bb33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_c:   HashSet<String> = ["cc11", "cc22", "cc33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_d:   HashSet<String> = ["dd11", "dd22", "dd33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_big: HashSet<String> = ["aa11", "aa22", "aa33", "bb11", "bb22", "bb33", "cc11", "cc22",
                                   "cc33", "dd11", "dd22", "dd33", "ee11", "ee22", "ee33"].iter().map(|&ss| ss.to_owned()).collect();

    print!("HashSets hs_a:    {:?}\nHashSets hs_b:    {:?}\nHashSets hs_big:  {:?}\n", hs_a, hs_b, hs_big);
    print!("hs_a.union(hs_b): \n    {:?}\n", hs_a.union(&hs_b));

    print!("hs_big.difference(&hs_a): \n    {:?}\n", hs_big.difference(&hs_a));

    print!("Iterating through hs_a.union(&hs_b): \n    ");

    for xx in hs_a.union(&hs_b) {
        print!("{}, ", xx);
    }
    print!("\n\n");

    
    print!("{}📚 6. Getting a vaue from map3, changing it, and updating map with new value \n\n", C_LL);

    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    map3.insert("k_a", hs_a);
    map3.insert("k_b", hs_b);
    map3.insert("k_c", hs_c);
    print!("map3: \n    {:?}\n", map3);

    let hs_e: HashSet<String> = ["ee77", "ee88", "ee99"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_x: HashSet<String> = (hs_e.union(&hs_d)).map(|ss| ss.to_owned()).collect();    
    map3.insert("k_e", hs_e);
    map3.insert("k_x", hs_x);
    print!("map3: \n    {:?}\n", map3);


    print!("Iterating through map3: \n");
    match map3.get("k_c") {
        Some(val) => {
            print!("val: \n    {:?},    ", val);

            let hs_y: HashSet<String> = (val.union(&hs_d)).map(|ss| ss.to_owned()).collect();
            print!("val U hs_d:   {:?}\n", hs_y);
            map3.insert("k_y", hs_y);
        }
        None => {
            print!("key: 'k_c' Not Found\n");
        }
    }
    print!("map3: \n    {:#?}\n", map3);
}



/*λ The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */




η5»ξ b0_hello1.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b0-hello0.rs and some comments  [ιδ21.12.22 τ08:48:42]
/*
   Block comments
*/

/// Library docs generating documentation (for later)

use ferris_says::say;                            //  ferris_says crate exports 'say function'
use std::io::{stdout, BufWriter};
// use std::{i8, i16, i32, i64, u16, u32, u64, isize, usize, f32, f64};     // on by default
// use std::io::stdin;                                                      // unused import

fn main() {
    // set a const line (C_LL); use snake-case; add prefex '_' to unused vars to avoid warnings
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: 010 hello0:rs \n\n", C_LL);

    print!("📚 Testing Bools Chars § Runes \n");

    let rr: char = '🈴';                // rune? yes
    let mut cc: char = 'x';             // mutable vars should be read
    let mut ok: bool = true;
    print!("(rr, ok, cc) = ({}, {}, {}) \n", rr, ok, cc);


    print!("{}📚 Getting Rid of Mutable Warnings \n", C_LL);
    ok = false; cc = 'y';
    print!("(ok, cc) = ({}, {})\n", ok, cc);


    print!("{}📚 Testing Multiple value assignment \n", C_LL);
    let (mut xx, mut yy, mut zz) = (1, 2, 3);
    print!("(xx, yy, zz) = ({}, {}, {}) \n", xx, yy, zz);
    let tmp = xx; xx = yy; yy = zz; zz = tmp;
    print!("(xx, yy, zz) = ({}, {}, {}) \n", xx, yy, zz);

    let ii64 = 64i64;
    print!("ii64 = {} \n", ii64);


    print!("{}📚 Testing Numbers \n", C_LL);
    let nn = 10;                        // inmutable
    let mut n1: i32 = 30;               // mutable
    let n2 = n1; n1 = 32;
    print!("(n1, n2) = ({}, {}) \n", n1, n2);


    print!("nn = {}, n1 = {} \n", nn, n1);                      // print ints
    print!("i8::MIN = {}, i8::MAX = {} \n", i8::MIN, i8::MAX);  // print min max for a type

    print!("{}📚 Numbers - Precission \n", C_LL);
    print!("2 dec places: {:.2} \n", 1.2345);
    print!("2 dec places: {:>4} \n", 1.23456789);

    print!("{}📚 Numbers - Spacing \n", C_LL);
    print!("2 dec places: {:.ws$} \n", 1.2345, ws = 4);
    print!("3 dec places: {:.ws$} \n", 1.2345, ws = 3);


    print!("{}📚 Binary, Octal, Hex \n", C_LL);
    print!("Bin = {0:b}, Octal = {0:o}, Hex = {0:x} \n", 255);


    print!("{}📚 Testing Positional Arguments \n", C_LL);
    let (bt, bf): (bool, bool) = (true, false);
    print!("b1 = {0}, b2 = {1}, b3 = {1}, b4 = {0} \n", bt, bf);
    print!("b1 = {0}, b2 = {1}, b3 = {1}, b4 = {0} \n", true, false);

    print!("{}📚 Argument - Spacing with ints \n", C_LL);                // pre = prefix, dp = decimal places

    print!("ten:>[8]         -> {ten:>pre$} \n",  ten = 10, pre = 8);
    print!("ten:>0[8]        -> {ten:>0pre$} \n", ten = 10, pre = 8);
    print!("ten:.0[8]        -> {ten:0.dp$} \n",   ten = 10, dp = 3);
    print!("ten:>[8].[3]     -> {ten:>pre$.dp$} \n",  ten = 10, pre = 8, dp = 3);

    print!("{}📚 Argument - Spacing with floats (flt) \n", C_LL);

    //        1234567890
    let flt = 10.1234567f64;                                            // Suffix annotation

    print!("{}📚 arg spacing - pre \n", C_LL);
    print!("flt:>[10]        -> {0:>pre$} \n",      flt, pre = 10);
    print!("flt:>[13]        -> {0:>pre$} \n",      flt, pre = 13);
    print!("flt:>[16]        -> {0:>pre$} \n",      flt, pre = 16);
    print!("flt:>[19]        -> {0:>pre$} \n",      flt, pre = 19);
    print!("flt:>[22]        -> {0:>pre$} \n",      flt, pre = 22);

    print!("{}📚 arg spacing - pre - pad with zeroes \n", C_LL);
    print!("flt:>0[10]       -> {0:0pre$} \n",      flt, pre = 10);
    print!("flt:>0[13]       -> {0:0pre$} \n",      flt, pre = 13);
    print!("flt:>0[16]       -> {0:0pre$} \n",      flt, pre = 16);
    print!("flt:>0[19]       -> {0:0pre$} \n",      flt, pre = 19);
    print!("flt:>0[22]       -> {0:0pre$} \n",      flt, pre = 22);

    print!("{}📚 arg spacing - pre pad with blanks \n", C_LL);
    print!("flt:>-[10]       -> {0:>-pre$} \n",     flt, pre = 10);
    print!("flt:>-[13]       -> {0:>-pre$} \n",     flt, pre = 13);
    print!("flt:>-[16]       -> {0:>-pre$} \n",     flt, pre = 16);
    print!("flt:>-[19]       -> {0:>-pre$} \n",     flt, pre = 19);
    print!("flt:>-[22]       -> {0:>-pre$} \n",     flt, pre = 22);

    print!("{}📚 arg spacing - pre (same) shortest \n", C_LL);
    print!("flt:[10]         -> {0:pre$} \n",       flt, pre = 10);
    print!("flt:[13]         -> {0:pre$} \n",       flt, pre = 13);
    print!("flt:[16]         -> {0:pre$} \n",       flt, pre = 16);
    print!("flt:[19]         -> {0:pre$} \n",       flt, pre = 19);
    print!("flt:[22]         -> {0:pre$} \n",       flt, pre = 22);

    print!("{}📚 arg spacing - pre § dec-places (dp) \n", C_LL);
    print!("flt:[10][0]      -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 0);
    print!("flt:[10][2]      -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 2);
    print!("flt:[10][4]      -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 4);
    print!("flt:[10][6]      -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 6);
    print!("flt:[10][7]      -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 7);
    print!("flt:[10][9]      -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 9);
    print!("flt:[10][11]     -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 11);
    print!("flt:[10][13]     -> {0:pre$.dp$} \n",   flt, pre = 10, dp = 13);

    print!("{}📚 arg spacing - pre § dec-places (dp) same but numbers only! \n", C_LL);
    print!("flt:[10][0]      -> {0:10.0} \n",       flt);
    print!("flt:[10][2]      -> {0:10.2} \n",       flt);
    print!("flt:[10][4]      -> {0:10.4} \n",       flt);
    print!("flt:[10][6]      -> {0:10.6} \n",       flt);
    print!("flt:[10][7]      -> {0:10.7} \n",       flt);
    print!("flt:[10][9]      -> {0:10.9} \n",       flt);
    print!("flt:[10][11]     -> {0:10.10} \n",      flt);
    print!("flt:[10][13]     -> {0:10.13} \n",      flt);


    print!("{}📚 Testing Crate: ferris_says \n", C_LL);
    let stdout = stdout();
    let msg = String::from("Hello Rustaceans!");
    let width = msg.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(msg.as_bytes(), width, &mut writer).unwrap();
    /*
    */
}


η5»ξ b1_basic_math0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b1-basic-math.rs [ιδ21.12.22 τ08:48:42]

#![allow(mixed_script_confusables)]  // note the '!', otherwise we get   => allow(mixed_script_confusables) is ignored unless specified at crate level
// #![allow(dead_code)]

fn main() {
    // set a const line (C_LL); use snake-case; add prefex '_' to unused vars to avoid warnings
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b1-basic-math.rs  \n\n", C_LL);

    print!("📚 Testing Basic Arithmetic \n");

    let i5 = 5i64; let i4 = 4i64; let i_4 = -4i64;
    print!("5 + 4   = {} \n", i5 + i4);
    print!("5 - 4   = {} \n", i5 - i4);
    print!("5 * 4   = {} \n", i5 * i4);
    print!("5 / 4   = {} \n", i5 / i4);
    print!("5 % 4   = {} \n", i5 % i4);
    print!("4 % 5   = {} \n", i4 % i5);

    print!("{}📚 Testing math functions \n", C_LL);

    let π = 3.1415f64; let f4 = 4f64; let f5 = 5f64; let f90 = 90f64;

    print!("abs(-4)         = {} \n", i_4.abs());
    print!("max 4, 5        = {} \n", i4.max(i5));
    print!("max 5, 4        = {} \n", i5.max(i4));
    print!("min 5, 4        = {} \n", i5.min(i4));

    print!("sqrt(4.0)       = {} \n", f4.sqrt());
    print!("sqrt(5.0)       = {} \n", f5.sqrt());
    print!("cbrt(4.0)       = {} \n", f4.cbrt());           // cube root

    print!("4 ^ 5           = {} \n", i4.pow(5u32));        // exponent has to be u32
    print!("4.0 ^ 5.0       = {} \n", f4.powf(5f64));       // exponent has to be u32
    print!("round(π)        = {} \n", π.round());
    print!("floor(π)        = {} \n", π.floor());
    print!("ceiling(π)      = {} \n", π.ceil());

    print!("e ^ 4.0         = {} \n", f4.exp());
    print!("e ^ 0.0         = {} \n", 0f64.exp());
    print!("e ^ 1.0         = {} \n", 1f64.exp());
    print!("ln(4.0)         = {} \n", f4.ln());
    print!("log(4.0)        = {} \n", f4.log10());
    print!("90.0 to_radians = {} \n", f90.to_radians());
    print!("π to_degrees    = {} \n", π.to_degrees());
    print!("sin(π / 2)      = {} \n", (π / 2f64).sin());

    print!("{}📚 Testing conditional operators \n", C_LL);  // !=, ==, >, <, >=, <=
    let ok = if i4 == 4 {true} else {false};
    print!("ok = {} ", ok);
    print!("\ndone!\n");


    print!("{}📚 Testing logical operators \n", C_LL);      // &&, ||, !
    if i4 == 4 {
        print!("i4 = 4 ");
    } else if (i4 == 5) || (i4 == 6) {
        print!("i4 = 5 ");
    } else {
        print!("i4 != 4, 5, or 6 ");
    }
    print!("\ndone!\n");


    print!("{}📚 Testing loops - print even numbers - break if => 10 \n", C_LL);
    let mut nn = 0;
    loop {
        if nn > 9 { break }
        if nn % 2 == 0 {                        // print if even
            print!("{}, ", nn);
            nn += 1;                            // no var++ in rust
            continue;
        }
        nn += 1;
        continue;
    }
    print!("\ndone!\n");


    print!("{}📚 Testing while loops - while < 10:  \n", C_LL);
    nn = 0;
    while nn < 10 {
        if nn % 2 == 0 { print!("{}, ", nn); }
        nn += 1;
    }
    print!("\ndone!\n");


    print!("{}📚 Testing range loops - range: 0..10: \n", C_LL);
    for ii in 0..10 {
        if ii % 2 == 0 { print!("{}, ", ii); }
    }
    print!("\ndone!\n");

}


η5»ξ b2_basic_strings0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b2-basic-strings.rs [ιδ21.12.22 τ08:48:42]


fn main() {
    // set a const line (C_LL); use snake-case; add prefex '_' to unused vars to avoid warnings
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b2-basic-strings.rs \n\n", C_LL);

    let ss0 = "One ring to rule them all";
    //         0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
    //         0        10        20        30        40        50        60        70        80        90       100       110       120
    //        0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
    let ss1 = "A💎 B📚 C🈳 D📁 E📕 F💢 G🔶 H🌐 I👽 J🔧 K⛩ L✨ M📡 N⭐ O⭕️ P🐬 Q❓ R🏃 S🔒 T🌀 U⏩ V⛅ W👎 X❗ Y👍 Z🈴 q🎡 x💫 ";
    let ss2 = "A💎 B📚 C🈳 D📁 \nE📕 F💢 G🔶 H🌐 \nI👽 J🔧 \nK⛩ L✨ M📡 N⭐ O⭕️ \nP🐬 Q❓ R🏃 S🔒 T🌀 U⏩ V⛅ \nW👎 X❗ Y👍 Z🈴 q🎡 x💫 ";
    let ss3 = "⭕️";
    let ss4 = "🔱";
    let ss5 = "📛";
    let ss6 = "🔰";
    let ss7 = "✅";
    let ss8 = "☑️✔️";
    let ss9 = "✔️✖️";
    let ssa = "✖️ ";
    let ssb = "❌";
    let ssc = "❎";                               // ➕➖➗➰➿〽️✳️✴️❇️❔❕Ⓜ️";

    print!("📚 Testing strings - length \n");
    print!("ss0: {}, ss0.len: {} \n", ss0, ss0.len());

    print!("{}📚 strings - split: \n\n", C_LL);
    let (str1, str2) = ss0.split_at(8);                         // [0..7] -> str1,  [8..24] -> str2
    print!("str1 [{}], str2 [{}] \n", str1, str2);

    print!("{}📚 Testing Runes (Chars) \n\n", C_LL);
    print!("ss1: [{}] \n", ss1);
    print!("ss1 has: 84 UTF-8 code points: 112 spaces, 28 letters, 28 2-char runes, 28 spaces \n");
    print!("ss1.len: {},  ss1.count: {} \n", ss1.len(), ss1.chars().count());

    print!("{}: ss3.count: {}, ss3.len: {}  \n", ss3, ss3.chars().count(), ss3.len());
    print!("{}: ss4.count: {}, ss4.len: {}  \n", ss4, ss4.chars().count(), ss4.len());
    print!("{}: ss5.count: {}, ss5.len: {}  \n", ss5, ss5.chars().count(), ss5.len());
    print!("{}: ss6.count: {}, ss6.len: {}  \n", ss6, ss6.chars().count(), ss6.len());
    print!("{}: ss7.count: {}, ss7.len: {}  \n", ss7, ss7.chars().count(), ss7.len());
    print!("{}: ss8.count: {}, ss8.len: {}  \n", ss8, ss8.chars().count(), ss8.len());
    print!("{}: ss9.count: {}, ss9.len: {}  \n", ss9, ss9.chars().count(), ss9.len());
    print!("{}: ssa.count: {}, ssa.len: {}  \n", ssa, ssa.chars().count(), ssa.len());
    print!("{}: ssb.count: {}, ssb.len: {}  \n", ssb, ssb.chars().count(), ssb.len());
    print!("{}: ssc.count: {}, ssc.len: {}  \n", ssc, ssc.chars().count(), ssc.len());


    print!("{}📚 runes, one-by-one \n\n", C_LL);

    let mut runes_iterator = ss1.chars();
    let mut next_rune = runes_iterator.next();
    loop {
        match next_rune {
            Some(val)    => print!("[{}], ", val),
            None         => break,
        }
        next_rune = runes_iterator.next();
    }
    print!("\ndone!\n");

    print!("{}📚 words, one-by-one \n\n", C_LL);

    let mut words_iterator = ss1.split_whitespace();
    let mut next_word = words_iterator.next();
    loop {
        match next_word {
            Some(val)    => print!("[{}], ", val),
            None         => break,
        }
        next_word = words_iterator.next();
    }
    print!("\ndone!\n");

    print!("{}📚 lines, one-by-one \n\n", C_LL);

    let mut lines_iterator = ss2.lines();
    let mut next_line = lines_iterator.next();
    loop {
        match next_line {
            Some(val)    => print!("[{}]\n", val),
            None         => break,
        }
        next_line = lines_iterator.next();
    }
    print!("done!\n");

    print!("{}📚 find patterns \n\n", C_LL);
    print!("is pattern \"S🔒 T🌀\" in ss1? {}\n", ss1.contains("S🔒 T🌀"));
    print!("done!\n");

}


η5»ξ b3_guess_game0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b3-guess-game.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.01]   

use std::io::stdin;

fn main() {
    // set a const str_buf (C_LL); use snake-case; add prefex '_' to unused vars to avoid warnings
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b3-guess-game.rs \n\n", C_LL);

    print!("📚 a labeled loop \n");
    
    'outer: loop {
        let answ: i32 = 10;
        print!("pick a number \n");

        loop {
            let mut str_buf = String::new();
            let reader = stdin().read_line(&mut str_buf);

            // Type Option<i32>:                is a monadic type that can be: None, |_| = an error, or a function
            // reader.ok()                      means that reader is at the end of the input line
            // map_or(None, |_| func())         maps to: a default, an error, or a function  
            // str_buf.trim().parse().ok()      is a function that trims str_buf -> converts string to i32 -> checks-ok
            let guess: Option<i32> = reader.ok().map_or(None, |_| str_buf.trim().parse().ok());

            match guess {
                None     => print!("Do pick a number \n"),
                Some(nn) if nn == answ => { 
                    print!("\nYou Win!!\n");
                    break 'outer;
                } 
                Some(nn) if nn < answ => {
                    print!("Guess Higher! \n");
                }
                Some(nn) if nn > answ => {
                    print!("Guess Lower! \n");
                }
                Some(_) => print!("Error \n"),
            }
        }
    }
    print!("done!\n");

}


η5»ξ b4_arrays_vectors_tuples0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b4-arrays-vectors-tuples.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.01]

// use std::io::stdin;

fn main() {
    // set a const str_buf (C_LL); use snake-case; add prefex '_' to unused vars to avoid warnings
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b4-arrays.rs \n\n", C_LL);

    print!("📚 Testing array (inmutable) \n\n");

    let aa = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    print!("aa[*]:      {:?}\n",  &aa);
    print!("aa.len():   {}\n",    aa.len());
    print!("aa[2]:      {}\n",    &aa[2]);
    print!("aa[4..7]:   {:?}\n",  &aa[4..7]);
    print!("done!\n");

    print!("{}📚 Testing vectors \n\n", C_LL);
    let mut vv = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];     // 89. 144, ...
    print!("vv[4..7]:   {:?}\n",  &vv[4..7]);

    print!("\nranging on vector vv: ");
    for elem in &vv {
        print!("{}, ", elem);
    }
    print!("\ndone!\n");

    print!("{}📚 vector pop § push \n\n", C_LL);
    print!("vv.pop():   {:?}\n", vv.pop());
    print!("vv.pop():   {:?}\n", vv.pop());
    
    let elem = vv.pop();
    print!("elem:       {:?}\n", elem);
    print!("done!\n\n");

    print!("📚 vector push 100, 101 \n");
    vv.push(100);
    vv.push(101);
    print!("vv[*]:      {:?}\n",  &vv);
    print!("done!\n\n");

    print!("{}📚 Testing tuples \n\n", C_LL);
    let tt: (&str, i32) = ("aaa", 111);
    let tt2             = ("bbb", 222);
    let tt3             = ("ccc", 333, "normal");
    print!("tt.0: {}, tt.1: {} \n", tt.0, tt.1);
    print!("tt2-pretty:\n{:#?}\n", tt2);
    print!("tt3-tuple:  {:?} \n", tt3);
    print!("done!\n\n");
}


η5»ξ b5_functions_closures0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b5-functions-closures.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.01]

// use std::io::stdin;

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}

fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


fn main() {
    // set a const str_buf (C_LL); use snake-case; add prefex '_' to unused vars to avoid warnings
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b5-functions-closures.rs \n\n", C_LL);

    print!("📚 fn say_hello \n");

    say_hello("Samael");
    print!("done!\n\n");

    print!("📚 fn add \n");
    print!("add(3, 4) = {} \n", add(3, 4));
    print!("done!\n\n");

    print!("📚 bind fn to a var - let sum = add \n");
    let sum = add;
    print!("sum(5, 7) = {} \n", sum(5, 7));
    print!("done!\n\n");

    print!("{}📚 Testing closures \n\n", C_LL);

    let closure_add = |ii: i32, jj: i32| ii + jj;
    print!("closure_add(5, 7) = {}\n", closure_add(5, 7));

    let closure_incr = |ii: i32| ii + 1;
    print!("closure_incr(5) = {}\n", closure_incr(5));

    let closure_counter1 = |seed: i32| {
        let mut index: i32 = seed;
        let counter = move || {
            index += 1;
            index
        };
        counter
    };

    let mut counter1 = closure_counter1(12);
    print!("counter1 - tic: {}, tic: {}, tic: {}\n", counter1(), counter1(), counter1()); 
    print!("done!\n\n");
}

η5»ξ b6_owners_pointers_refs0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b6-owners-pointers-refs.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.02] [δ22.01.01]

// use std::io::stdin;

fn sum_vector(vv: &Vec<i32>) -> i32 {

    // folds is an iterator adapter that takes an initial value and applies a closure to all the values of the vector
    let sum = vv.iter().fold(0, |mut sum, &xx| { sum += xx; sum });     // it seems xx ranges through the vector elements
    return sum;
}

fn main() {
    // set a const str_buf (C_LL); use snake-case; add prefex '_' to unused vars to avoid warnings
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b6-owners-pointers-refs.rs \n\n", C_LL);

    // One binding for each resource (not a primitive val or var)
    print!("{}📚 Testing loosing a binding \n\n", C_LL);

    let mut v1 = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];     // 89. 144, ...
    print!("v1[4..7]:   {:?}\n",  &v1[4..7]);
    v1.pop();

    let v2 = v1;
    print!("v2[4..7]:   {:?}\n",  &v2[4..7]);
    // print!("v1[4..7]:   {:?}\n",  &v1[4..7]);        //  👎ς FAIL! value borrowed after move. More Info: `∞ rustc --explain E0382`
    print!("done!\n\n");

    print!("{}📚 Testing folding adapter \n\n", C_LL);
    print!("v2:             {:?}\n",  &v2);
    print!("sum_vector(v2): {}\n", sum_vector(&v2));
    print!("done!\n\n");

}


η5»ξ b7_structs_traits0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b7-structs-traits-enums.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.02] [δ22.01.01]

// use std::io::stdin;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// structs - a circle struct 
struct Circle {
    xx: f64,
    yy: f64,
    rr: f64
}

// a rectangle struct 
struct Rect {
    ll: f64,
    ww: f64
}

// the bad way: a fn to get the radius 
fn get_radius(cc: &Circle) -> f64 {
    cc.rr
}

impl Circle {
    pub fn get_xx(&self) -> f64 { self.xx }     // ugly way 
    pub fn get_yy(&self) -> f64 { self.yy }     // ugly way 
    pub fn get_rr(&self) -> f64 { self.rr }     // ugly way 
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// traits
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 { 3.14159 * self.rr * self.rr }
}

impl HasArea for Rect {
    fn area(&self) -> f64 { self.ll * self.ww }
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// enumerations 
enum Hero {
    Fast,
    Strong(i8),
    Info {name: String, secret: String}
}

fn get_info(hh: Hero) {
    match hh {
        Hero::Fast                   => print!("fast hero\n"), 
        Hero::Strong(ii)             => print!("strong hero, {} rating \n", ii), 
        Hero::Info {name, secret}    => print!("{}'s is {}\n", name, secret),
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// main
pub fn mod_main() {
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b7-structs-traits.rs  \n\n", C_LL);

    print!("📚 Testing structs - the bad way\n\n");
    let c1 = Circle { xx: 3., yy: 4., rr: 5. };

    print!("get_radius(c1): {}\n", get_radius(&c1));
    print!("done!\n\n");

    print!("{}📚 Testing structs - the good way\n\n", C_LL);
    print!("c1.get_xx():     {}\n", c1.get_xx());
    print!("c1.get_yy():     {}\n", c1.get_yy());
    print!("c1.get_rr():     {}\n", c1.get_rr());
    print!("done!\n\n");

    print!("{}📚 Testing structs - the ugly wayY!!\n\n", C_LL);
    print!("c1.xx:          {}\n", c1.xx);
    print!("c1.yy:          {}\n", c1.yy);
    print!("c1.rr:          {}\n", c1.rr);
    print!("done!\n\n");

    print!("{}📚 Testing traits (interfaces) \n\n", C_LL);
    let r1 = Rect { ll: 5., ww: 7. };
    print!("c1.area():      {}\n", c1.area());
    print!("r1.area():      {}\n", r1.area());
    print!("done!\n\n");

    print!("{}📚 Testing enumerations \n\n", C_LL);
    let hulk    = Hero::Strong(100);
    let flash   = Hero::Fast;
    let batman  = Hero::Info {name: "Bruce Wayne".to_owned(), secret: "111".to_owned()};

    get_info(hulk);
    get_info(flash);
    get_info(batman);
    print!("done!\n\n");

}


η5»ξ b7_structs_traits1.rs
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





// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// main
pub fn mod_main() {
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
}

η5»ξ c0_time_date_zone0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ c0_time_date_zone0.rs  [ιδ21.12.22 τ08:48:42] 

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;


fn say_hello(to: &str) {
    print!("hello {}\n", to);
}

fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// read, parse § write files 

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn read_lines_from_file() {

    if let Ok(ff) = open_file("./data0.txt") {

        let lines = ff.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main
pub fn mod_main() {

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

    // method 1
    assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

    // method 2
    assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

    // method 3
    assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
    assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

    // oops, the year is missing!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
    // oops, the format string does not include the year at all!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
    // oops, the weekday is incorrect!
    assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());

}

/* THE CODE PIT 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.07] 
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let tt = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", tt);

    let tt = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", tt);

    let tt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", tt);

    Ok(())
}



•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.05] 
    let tt = DateTime::parse_from_str("Fri Nov 28 12:00:09 2021", "%a %b %e %T %Y");
    print!("Fri Nov 28 12:00:09:                {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y");
    print!("Fri Nov 28 12:00:09:                {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08", "%b %d, %Y %H.%M%Z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

    // method 1
    assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
    assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

    // method 2
    assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
               Ok(fixed_dt.clone()));
    assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

    // method 3
    assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
    assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

    // oops, the year is missing!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
    // oops, the format string does not include the year at all!
    assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
    // oops, the weekday is incorrect!
    assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.05] 
Jan-10-1990-00.00-P-0003000
Jan-11-1991-00.01-P-0003011
Jan-12-1992-00.02-P-0003022
Jan-13-1993-00.03-P-0003033
Jan-14-1994-00.04-P-0003044
Jan-15-1995-00.05-P-0003055
Jan-16-1996-00.06-P-0003066
Jan-16-1996-00.06-P-0003066
Jan-17-1997-00.07-P-0003077
Jan-18-1998-00.08-P-0003088
Jan-19-1999-00.09-P-0003099
Jan-20-2000-00.10-P-0003000
Jan-21-2211-00.11-P-0003011

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
[δ22.03.07] 

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ c0_time_date_zone0.rs  [ιδ21.12.22 τ08:48:42] 

#![allow(dead_code)]
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Local};
use chrono_tz::US::Pacific;


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main
pub fn mod_main() {
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: c0-time-date-zone0.rs  \n\n", C_LL);

    print!("📚 Testing time, date and zones \n\n");
    println!("{:?}", chrono::offset::Local::now());
    println!("{:?}", chrono::offset::Utc::now());

    print!("📚 Testing time \n\n");
    let tnow = Local::now();
    let tutc = Utc::now();

    print!("local time: {}\n", tnow);
    print!("UTC time:   {}\n", tutc);

    print!("local time: {:?}\n", tnow);
    print!("UTC time:   {:?}\n", tutc);

    print!("📚 Testing Converting from one time-zone to another \n\n");

    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);

    print!("\ndone!\n");
}




use dateparser::DateTimeUtc;

use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Local};
use chrono_tz::US::Pacific;
use std::error::Error;

use chrono_tz::America::Chicago;

fn main() {
    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: c1-time-date-zone0.rs  \n\n", C_LL);

    print!("📚 Testing time, date and zones \n\n");
    println!("{:?}", chrono::offset::Local::now());
    println!("{:?}", chrono::offset::Utc::now());

    print!("📚 Testing time \n\n");
    let tnow = Local::now();
    let tutc = Utc::now();

    print!("local time: {}\n", tnow);
    print!("UTC time:   {}\n", tutc);

    print!("local time: {:?}\n", tnow);
    print!("UTC time:   {:?}\n", tutc);

    print!("📚 Testing Converting from one time-zone to another \n\n");
    "May 25, 2021",
    "oct 7, 1970",
    "oct 7, 70",
    "oct. 7, 1970",
    "oct. 7, 70",
    "October 7, 1970",
    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);

    print!("\ndone!\n");
}
fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
*/



/*
fn main() {
    let parsed = "Wed, 02 Jun 2021 06:31:39 GMT".parse::<DateTimeUtc>()?.0;
    println!("{:#?}", parsed.with_timezone(&Pacific));
    Ok(());


    read_lines_from_file();
    if let Ok(usrn) = read_username_from_file() {
        println!("{}", usrn);
    }

    print!("📚 Testing Converting from one time-zone to another \n\n");
        "May 25, 2021",
        "oct 7, 1970",
        "oct 7, 70",
        "oct. 7, 1970",
        "oct. 7, 70",
        "October 7, 1970",
    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);
}

US::
    Alaska	
    Aleutian	
    Arizona	
    Central	
    EastIndiana	
    Eastern	
    Hawaii	
    IndianaStarke	
    Michigan	
    Mountain	
    Pacific	
    Samoa

America::
    Adak	
    Anchorage	
    Anguilla	
    Antigua	
    Araguaina	
    Aruba	
    Asuncion	
    Atikokan	
    Atka	
    Bahia	
    Bahia_Banderas	
    Barbados	
    Belem	
    Belize	
    BlancSablon	
    Boa_Vista	
    Bogota	
    Boise	
    Buenos_Aires	
    Cambridge_Bay	
    Campo_Grande	
    Cancun	
    Caracas	
    Catamarca	
    Cayenne	
    Cayman	
    Chicago	
    Chihuahua	
    Coral_Harbour	
    Cordoba	
    Costa_Rica	
    Creston	
    Cuiaba	
    Curacao	
    Danmarkshavn	
    Dawson	
    Dawson_Creek	
    Denver	
    Detroit	
    Dominica	
    Edmonton	
    Eirunepe	
    El_Salvador	
    Ensenada	
    Fort_Nelson	
    Fort_Wayne	
    Fortaleza	
    Glace_Bay	
    Godthab	
    Goose_Bay	
    Grand_Turk	
    Grenada	
    Guadeloupe	
    Guatemala	
    Guayaquil	
    Guyana	
    Halifax	
    Havana	
    Hermosillo	
    Indianapolis	
    Inuvik	
    Iqaluit	
    Jamaica	
    Jujuy	
    Juneau	
    Knox_IN	
    Kralendijk	
    La_Paz	
    Lima	
    Los_Angeles	
    Louisville	
    Lower_Princes	
    Maceio	
    Managua	
    Manaus	
    Marigot	
    Martinique	
    Matamoros	
    Mazatlan	
    Mendoza	
    Menominee	
    Merida	
    Metlakatla	
    Mexico_City	
    Miquelon	
    Moncton	
    Monterrey	
    Montevideo	
    Montreal	
    Montserrat	
    Nassau	
    New_York	
    Nipigon	
    Nome	
    Noronha	
    Nuuk	
    Ojinaga	
    Panama	
    Pangnirtung	
    Paramaribo	
    Phoenix	
    Port_of_Spain	
    PortauPrince	
    Porto_Acre	
    Porto_Velho	
    Puerto_Rico	
    Punta_Arenas	
    Rainy_River	
    Rankin_Inlet	
    Recife	
    Regina	
    Resolute	
    Rio_Branco	
    Rosario	
    Santa_Isabel	
    Santarem	
    Santiago	
    Santo_Domingo	
    Sao_Paulo	
    Scoresbysund	
    Shiprock	
    Sitka	
    St_Barthelemy	
    St_Johns	
    St_Kitts	
    St_Lucia	
    St_Thomas	
    St_Vincent	
    Swift_Current	
    Tegucigalpa	
    Thule	
    Thunder_Bay	
    Tijuana	
    Toronto	
    Tortola	
    Vancouver	
    Virgin	
    Whitehorse	
    Winnipeg	
    Yakutat	
    Yellowknife	

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */

η5»ξ c1_time_date_zone0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ c1_time_date_zone0.rs  [ιδ21.12.22 τ08:48:42]

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;
use chrono_tz::US::Pacific;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types 

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Some simple functions 

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}


fn exp_2() -> i32 {
    2
}


fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


struct Rectangle {

    ll: u8,
    ww: u8
}


impl Rectangle {

    fn is_square(&self) -> bool {
        self.ll == self.ww
    }
}



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// read, parse § write files 

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn read_lines_from_file() {

    if let Ok(ff) = open_file("./data0.txt") {

        let lines = ff.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ unit tests 
#[cfg(test)]                                        // only compiles module if: ∞ cargo test 
mod parse_test {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    // use chrono::{DateTime, FixedOffset, TimeZone, NaiveDateTime, Utc, Local};
    // use chrono_tz::US::Pacific;
    // use chrono_tz::America::Chicago;
    
    #[test]
    #[should_panic]
    fn test_basic1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }


    #[test]
    fn test_basic2() {

        assert_eq!(super::exp_2(), super::add(1, 1));
        assert_ne!(super::exp_2(), super::add(1, 2));
    }


    #[test]
    fn test_struct1() {

        let rr = super::Rectangle {
            ll: 20,
            ww: 20
        };

        assert!(rr.is_square());
    }


    #[test]
    fn test_parse_method() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));
    }


    #[test]
    #[ignore]
    // test DateTime::parse_from_{ str, rfc2822, rfc3339}
    fn test_parse_from() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

        assert_ne!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+08:00"), Ok(fixed_dt.clone()));
    }

    
    #[test]
    // test chrono::Utc.datetime_from_strm
    fn test_datetime_from_str() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);

        assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
        assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

        // oops, the year is missing!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
        // oops, the format string does not include the year at all!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
        // oops, the weekday is incorrect!
        assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main
pub fn mod_main() {

    print!("{}📚 STARTING: c1-time-date-zone0.rs  \n\n", C_LL);

    // read_lines_from_file();
    
    print!("📚 Testing time, date and zones \n\n");
    println!("{:?}", chrono::offset::Local::now());
    println!("{:?}", chrono::offset::Utc::now());

    print!("📚 Testing time \n\n");
    let tnow = Local::now();
    let tutc = Utc::now();

    print!("local time: {}\n", tnow);
    print!("UTC time:   {}\n", tutc);

    print!("local time: {:?}\n", tnow);
    print!("UTC time:   {:?}\n", tutc);

    print!("📚 Testing Converting from one time-zone to another \n\n");

    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);


    print!("{}📚 Testing parsing from rfc2822 \n\n", C_LL);

    let tt = DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_rfc2822("28 Nov 2014 21:00:09 +0900");
    print!("28 Nov 2014 21:00:09 +0900:         {}\n", tt.unwrap());
        

    print!("{}📚 Testing DateTime parsing-from-string \n\n", C_LL);

    let tt = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    print!("2014-11-28 21:00:09 +09:00:         {}\n", tt.expect("oops!"));
        
    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0900", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.expect("oops!"));
        
    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0000", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 :         {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 +0000", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());
        
    let tt = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z");
    print!("5.8.1994 8:00 am +0000:             {}\n", tt.unwrap());

    // Note that the Z-Offset of: -0800, refers to the local time and the time diference to UTC
    // e.g., 1200 -08:00, means that it's noon in the local zone and 4:00 am in UTC. 
    print!("{}📚 Testing parsing-from-string in CST Time Zone (UTC - 6) \n\n", C_LL);

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 -0600", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());
    
    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 -0600", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());
    

    print!("{}📚 Testing NaiveDateTime parsing-from-string \n\n", C_LL);
    
    let tt = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S");
    print!("23:56:04:                           {}\n", tt.unwrap());

    let tt = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d");
    print!("2015-09-05:                         {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
    print!("2015-09-05 23:56:04:                {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("23:56:04 2015-09-05", "%H:%M:%S %Y-%m-%d");
    print!("23:56:04 2015-09-05:                {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("23 56 04 2015 Aug 05", "%H %M %S %Y %b %d");
    print!("23 56 04 2015 Aug 05:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("23 56 04 Aug 05 2015", "%H %M %S %b %d %Y");
    print!("23 56 04 Aug 05 2015:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("Aug 05 23 56 04 2015", "%b %d %H %M %S %Y");
    print!("Aug 05 23 56 04 2015:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("Aug 05 23:56:04 2015", "%b %d %H:%M:%S %Y");
    print!("Aug 05 23:56:04 2015:               {}\n", tt.unwrap());
    
    let tt = NaiveDateTime::parse_from_str("Nov 27 12:00:09 2021", "%b %d %H:%M:%S %Y");
    print!("Nov 27 12:00:09 2021:               {}\n", tt.unwrap());
        
        
    let tt = NaiveDateTime::parse_from_str("Fri Nov 28 12:00:09 2014", "%a %b %d %T %Y");
    print!("Fri Nov 28 12:00:09 2014:           {}\n", tt.unwrap());
        
    let tt = NaiveDateTime::parse_from_str("Sat Nov 29 12:00:09 2014", "%a %b %d %T %Y");
    print!("Sat Nov 29 12:00:09 2014:           {}\n", tt.unwrap());
        
    let tt = NaiveDateTime::parse_from_str("Dec 18, 1998 19.08;", "%b %d, %Y %H.%M;");
    print!("Dec 18, 1998 19.08:                 {}\n", tt.unwrap());


    print!("\ndone!\n");
    

}

/* THE CODE PIT 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Moved  => c0_..


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */
η5»ξ c2_time_date_zone0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ c2-time-date-zone0.rs  [ιδ21.12.22 τ08:48:42] 

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types 

const C_LL:     &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
const C_MSG1:   &str = "1. Not all who wonder are lost.  ";
const C_MSG2:   &str = "2. Not all who wonder are lost.  ";
const C_MSG3:   &str = "3. Not all who wonder are lost.  \n";
const C_ERR1:   &str = "::👎ς FAILED!::";

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// read, parse § write files 

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data_in1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn read_lines_from_file() {

    if let Ok(get_buf) = open_file0("./data_in0.txt") {

        let lines = get_buf.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


fn parse_lines_from_file() {

    if let Ok(get_buf) = open_file0("./data_in1.txt") {

        let lines = get_buf.lines();

        for line in lines {
            if let Ok(ip) = line {
                let tt = NaiveDateTime::parse_from_str(ip.as_str(), "%b %d %Y %H.%M");
                println!("{}:       {}", ip, tt.unwrap());
            }
        }
    }
}


// λ open_file0 opens a file in read-only mode 
fn open_file0<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let get_buf = File::open(filename)?;
    Ok(io::BufReader::new(get_buf))
}



// λ open_file1 opens a file in read-only mode 
fn open_file1(filename: &str) -> io::BufReader<File> {

    let path = Path::new(filename);
    let what = path.display();

    let get_buf = match File::open(&path) {
        Err(why) => panic!("::open_file1::error_on::{}::because{}::", what, why),
        Ok(get_buf) => get_buf,
    };
    io::BufReader::new(get_buf)
}



// λ create_file1 opens a file in read-only mode 
fn create_file1(filename: &str) -> File {

    let path = Path::new(&filename);
    let what = path.display();

    // Create a file in write-only mode
    let put_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", what, why),
        Ok(put_file) => put_file,
    };
    put_file
}




// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main
pub fn mod_main() {


    print!("{}📚 STARTING: c2-time-date-zone0.rs ISO 8601 § RFC 3339  \n\n", C_LL);
    // test DateTime::parse_from_{ str, rfc2822, rfc3339 }

    //create an output file 
    let mut put_file = create_file1("./data_out1.txt");

    // read a file with date-time stamps is a regular format and write them in rf 
    if let Ok(get_buf) = open_file0("./data_in1.txt") {

        let lines = get_buf.lines();

        for line in lines {
            if let Ok(ip) = line {
                let tt = NaiveDateTime::parse_from_str(ip.as_str(), "%b %d %Y %H.%M");
                let line: String = ip.to_owned() + ", " + &tt.unwrap().to_string() + ",\n"; 
                put_file.write_all(line.as_bytes()).expect(C_ERR1);
                // println!("{}:       {}", ip, tt.unwrap());
            }
        }
    }

    print!("\ndone!\n");
}

/* THE CODE PIT 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

    print!("📚 Writing to a file using `match` \n\n");

    let line: String = C_MSG1.to_owned() + C_MSG2 + C_MSG3; 

    put_file.write_all(line.as_bytes()).expect(C_ERR1);
    
    put_file.write_all(C_MSG1.as_bytes()).expect(C_ERR1);
    put_file.write_all(C_MSG1.as_bytes()).expect(C_ERR1);




•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    //create a file and and write 3 entries using `expect`
    let mut put_file = create_file1("./data_out.txt");

    put_file.write_all(C_MSG1.as_bytes()).expect(C_ERR1);
    put_file.write_all(C_MSG1.as_bytes()).expect(C_ERR1);
    put_file.write_all(C_MSG1.as_bytes()).expect(C_ERR1);


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

    //create a file and and write 2 entries using `match`
    let mut put_file = create_file1("./data_out.txt");

    match put_file.write_all(C_MSG1.as_bytes()) {
        Err(why) => panic!("::👎ς FAILED!::because::{}::", why),
        Ok(_)  => println!("::👍υ OK!::"),
    }
    
    match put_file.write_all(C_MSG1.as_bytes()) {
        Err(why) => panic!("::👎ς FAILED!::because::{}::", why),
        Ok(_)  => println!("::👍υ OK!::"),
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    // Very Manually Write to a file twice 
    print!("📚 Manually Writing to a file twice  \n\n");
    let path = Path::new("data_out.txt");
    let what = path.display();

    // Manually create a file in write-only mode, it returns `File`
    let mut put_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", what, why),
        Ok(put_file) => put_file,
    };

    // Manually write the some junk to put_file, it returns `io::Result<()>`
    match put_file.write_all(C_MSG1.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", what, why),
        Ok(_) => println!("successfully wrote to {}", what),
    }
    
    // Manually write the some junk to put_file, it returns `io::Result<()>`
    match put_file.write_all(C_MSG1.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", what, why),
        Ok(_) => println!("successfully wrote to {}", what),
    }
    
    let tt = NaiveDateTime::parse_from_str("Dec 18, 1998 19.08;", "%b %d, %Y %H.%M;");
    print!("Dec 18, 1998 19.08:                 {}\n", tt.unwrap());


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", what, why),
        Ok(file) => file,
    };


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•



// λ create_file opens a file in read-only mode 
fn create_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {

    let path = Path::new(filename);
    let what = path.display();
    // Create a file in write-only mode, it returns `io::Result<File>`
    let mut Ok(put_buf) = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", what, why),
        Ok(put_buf) => put_buf,
    };
}



•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// λ open_file opens a file in read-only mode 
fn open_file2<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {

    // Create a file in write-only mode, it returns `io::Result<File>`
    if let mut put_buf = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", what, why),
        Ok(put_buf) => put_buf,
    };

where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}


    // Open a file in write-only mode, it returns `io::Result<File>`
    let mut put_buf = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", what, why),
        Ok(put_buf) => put_buf,
    };

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */

η5»ξ e001_sum_multiples.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//  ✨λ e001_sum_multiples.rs Project Euler  ι✧22․06․13✦15․01․43․ 🌎η ✧22․06․15․    

#![allow(dead_code)]

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//λ unit tests
#[cfg(test)]
mod unit_tests{
    // use chrono::{};

    #[test]
    #[should_panic]
    fn fail_test() {

        let ok: bool = false;
        assert!(ok);
    }

    #[test]
    fn basic_tests1() {

        assert_eq!(super::add_multiples1(6), 8);
        assert_eq!(super::add_multiples1(7), 14);
        assert_eq!(super::add_multiples1(10), 23);
        assert_eq!(super::add_multiples1(11), 33);
        assert_eq!(super::add_multiples1(12), 33);

        assert_eq!(super::add_multiples1(31), 225);
        assert_eq!(super::add_multiples1(1000), 233168);
    }

    #[test]
    fn basic_tests2() {

        assert_eq!(super::add_multiples2(6), 8);
        assert_eq!(super::add_multiples2(7), 14);
        assert_eq!(super::add_multiples2(10), 23);
        assert_eq!(super::add_multiples2(11), 33);
        assert_eq!(super::add_multiples2(12), 33);

        assert_eq!(super::add_multiples2(31), 225);
        assert_eq!(super::add_multiples2(1000), 233168);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//λ functions

//  second attempt - a bit cleaner
pub fn add_multiples1(max: i64) -> i64 {

    let mut mult5 = 0;          //  multiple_of_5
    let mut mult3  = 0;         //  multiple_of_3
    let mut sum  = 0;

    loop {

        if mult3.min(mult5) >= max { break; }
        sum = sum + mult3.min(mult5);
        if mult3 == mult5 {
            mult3 += 3;
            mult5 += 5;
        } else if mult3 < mult5 {
            mult3 += 3;
        } else {
            mult5 += 5;
        }
    }
    sum
}

//  brute force attempt 
pub fn add_multiples2(max: i64) -> i64 {

    let mut mult5 = 0;      //  multiple_of_5
    let mut mult3 = 0;      //  multiple_of_3
    let mut sum = 0;

    'outer: loop {

        if mult3 == mult5 {
            if mult3 < max {
                sum = sum + mult3;
                mult3 = mult3 + 3;
                mult5 = mult5 + 5;
            } else {
                break 'outer;
            }
        } else if mult3 < mult5 {
            if mult3 < max {
                sum = sum + mult3;
                mult3 = mult3 + 3;
            } else {
                break 'outer;
            }
        } else {
            if mult5 < max {
                sum = sum + mult5;
                mult5 = mult5 + 5;
            } else {
                break 'outer;
            }
        }
    }
    sum
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//λ mod_main
pub fn mod_main() {

    print!("{}📚 STARTING: t1_tdd.rs - Test Driven Development  \n\n", C_LL);
    let max1000 = add_multiples1(1000);
    print!("add_multiples1(1000) = {} \n\n", max1000);
    print!("\ndone!\n");

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════•
//λ mod_main main program in separate file: main.rs 
mod e001_sum_multiples;                 use crate::e001_sum_multiples::{mod_main};

fn main() {
    mod_main()
}


/* The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════•
12:  3  6  9  5  10 = 33
31:  3  6  9  12  15  18  21  24  27  30    5  10      20  25 = 225

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════•

End of The Code Pit */

η5»ξ f0_python_and_rust0.rs
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

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main

pub fn mod_main() {

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
}

η5»ξ f1_python_and_rust0.rs
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ d1_python_and_rust0.rs - Borrow or Clone - [ιδ22.03.16 τ20:15:43]   


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

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main

pub fn mod_main() {

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
}


η5»ξ f2_give_and_take.rs
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


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main

pub fn mod_main() {

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
}

η5»ξ f3_borrow_or_clone.rs
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


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// λ mod_main

pub fn mod_main() {


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
}


η5»ξ g0_load_data.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ g0_load_data.rs

#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};   // no need for self bec no io::BufReader, etc  
use std::path::Path;
use std::collections::{HashMap, HashSet};


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests

#[cfg(test)]
mod load_data_test {
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions: read_lines  print_lines

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {

    let ff = File::open(filename)?;
    Ok(BufReader::new(ff).lines())

}


fn print_lines(results: Result<Lines<BufReader<File>>>) {

    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                println!("    {}", val);
            }
        }
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 Starting: g0_load_data.rs \n\n", C_LL);
    
    let results = read_lines("./d3_one_to_many_in.csv");
    print_lines(results);

}



/*λ The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */


η5»ξ g1_hash_maps_sets.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ g1_hash_maps_sets.rs  

#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};   // no need for self bec no io::BufReader, etc  
use std::path::Path;
use std::collections::{HashMap, HashSet};


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests

#[cfg(test)]
mod load_data_test {
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions: read_lines  print_lines

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {

    let ff = File::open(filename)?;
    Ok(BufReader::new(ff).lines())

}


fn print_lines(results: Result<Lines<BufReader<File>>>) {

    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                print!("    {}\n", val);
            }
        }
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 Starting: g1_hash_maps_sets.rs \n\n", C_LL);
    
    print!("📚 1. Testing HashMaps with &strs \n\n");

    let mut map0: HashMap<&str, &str> = HashMap::new();
    map0.insert("k_a", "aaaa");
    map0.insert("k_b", "bbbb");
    map0.insert("k_c", "cccc");
    map0.insert("k_d", "dddd");
    map0.insert("k_e", "eeee");
    print!("map0: \n    {:?}\n", map0);
    map0.insert("k_b", "xxxx");
    print!("map0 after insert('k_b', 'xxxx') \n    {:?}\n", map0);
    let val0 = map0.get("ccc");
    print!("getting - val0 = map0.get('ccc'): \n    {:?}\n", val0);

    print!("{}📚 2. Testing HashMaps with Arrays \n\n", C_LL);

    let mut map1: HashMap<&str, [&str; 4]> = HashMap::new();
    map1.insert("k1", ["arr1", "aa11", "bb11", "cc11"]);
    map1.insert("k2", ["arr2", "aa22", "bb22", "cc22"]);
    map1.insert("k3", ["arr3", "aa33", "bb33", "cc33"]);
    let val1 = map1.get("k2");
    print!("map1:         \n    {:?}\n", map1);
    print!("getting - val1 = map1.get('k2'): \n    {:?}\n", val1);
    let arr4 = ["arr4", "aaa44", "bbb44", "ccc44"];
    print!("inserting - arr4 into map1:     \n    {:?}\n", arr4);
    map1.insert("aa4", arr4);
    print!("map1:         \n    {:?}\n", map1);

    print!("{}📚 3. Testing HashMaps with Vectors \n\n", C_LL);

    let mut map2: HashMap<&str, Vec<&str>> = HashMap::new();
    map2.insert("k1", vec!["vec11", "aa11", "bb11", "cc11"]);
    map2.insert("k2", vec!["vec22", "aa22", "bb22", "cc22"]);
    map2.insert("k3", vec!["vec33", "aa33", "bb33", "cc33"]);
    let val2 = map2.get("k3");
    print!("map2: \n    {:?}\n", map2);
    print!("getting - val2 = map2.get('k3'): \n    {:?}\n", val2);

    print!("{}📚 4. Testing HashSets Union Difference Intersection § Symmetric Difference \n\n", C_LL);

    let hs_a:   HashSet<String> = ["aa11", "aa22", "aa33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_b:   HashSet<String> = ["bb11", "bb22", "bb33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_c:   HashSet<String> = ["cc11", "cc22", "cc33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_d:   HashSet<String> = ["dd11", "dd22", "dd33"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_big: HashSet<String> = ["aa11", "aa22", "aa33", "bb11", "bb22", "bb33", "cc11", "cc22",
                                   "cc33", "dd11", "dd22", "dd33", "ee11", "ee22", "ee33"].iter().map(|&ss| ss.to_owned()).collect();

    print!("HashSets hs_a:    {:?}\nHashSets hs_b:    {:?}\nHashSets hs_big:  {:?}\n", hs_a, hs_b, hs_big);
    print!("hs_a.union(hs_b): \n    {:?}\n", hs_a.union(&hs_b));

    print!("hs_big.difference(&hs_a): \n    {:?}\n", hs_big.difference(&hs_a));

    print!("Iterating through hs_a.union(&hs_b): \n    ");

    for xx in hs_a.union(&hs_b) {
        print!("{}, ", xx);
    }
    print!("\n\n");

    
    print!("{}📚 6. Getting a vaue from map3, changing it, and updating map with new value \n\n", C_LL);

    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    map3.insert("k_a", hs_a);
    map3.insert("k_b", hs_b);
    map3.insert("k_c", hs_c);
    print!("map3: \n    {:?}\n", map3);

    let hs_e: HashSet<String> = ["ee77", "ee88", "ee99"].iter().map(|&ss| ss.to_owned()).collect();
    let hs_x: HashSet<String> = (hs_e.union(&hs_d)).map(|ss| ss.to_owned()).collect();    
    map3.insert("k_e", hs_e);
    map3.insert("k_x", hs_x);
    print!("map3: \n    {:?}\n", map3);


    print!("Iterating through map3: \n");
    match map3.get("k_c") {
        Some(val) => {
            print!("val: \n    {:?},    ", val);

            let hs_y: HashSet<String> = (val.union(&hs_d)).map(|ss| ss.to_owned()).collect();
            print!("val U hs_d:   {:?}\n", hs_y);
            map3.insert("k_y", hs_y);
        }
        None => {
            print!("key: 'k_c' Not Found\n");
        }
    }
    print!("map3: \n    {:#?}\n", map3);
}



/*λ The Code Pit

        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        print!("A: {:?}\n", a);
        print!("B: {:?}\n", b);

        // Print [1, 2, 3, 4, 5] in arbitrary order
        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());

        // This should print [1]
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());

        // Print [2, 3, 4] in arbitrary order.
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());

        // Print [1, 5]
        print!("Symmetric Difference: {:?}\n",
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());

        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    map2.insert("bb1", HashSet::from(vec!(["top", "aaa11", "bbb11", "ccc11"])));
    map2.insert("bb2", HashSet::from(vec!(["top", "aaa22", "bbb22", "ccc22"])));
    map2.insert("bb3", HashSet::from(vec!(["top", "aaa33", "bbb33", "ccc33"])));

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
let s: String = "abcdefg".to_owned();
let s_slice: &str = &s[..];  // take a full slice of the string

    let mut aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
            //print!("aa2 U aa1:    {:?}\n", aa2.union(val).into_iter().collect::<HashSet>());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    let mut map3 = HashMap::new();
    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    print!("map3 (insert some):      {:?}\n", map3);

    let mut aa3 = HashSet::from(["aaa77", "aaa88", "aaa99"]);
    let aa4 = aa3.union(&aa1).collect().to_string();    
    print!("aa3 U aa1                {:?}\n", aa4);
    map3.insert("aa1", aa4);
    print!("map3 (insert some):      {:?}\n", map3);

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
use std::io::Read;

    let aa4: HashSet<_> = aa3.union(&aa1).to_string().collect();    
    let diff: HashSet<_> = a.difference(&b).collect();
How to type cast from std::collections::hash_set::Union  to HashSet 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Doesnt Work 
fn read_lines<P>(filename: &dyn AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>> {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
where P: AsRef<Result<Lines<BufReader<File>>>>, {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                print!("    {}\n", val);
            }
        }
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
#[derive(debug)]
struct One2many {
    one: 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Try all of these in the following example:

    use std::collections::HashSet;

    fn main() {
        let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
        let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

        assert!(a.insert(4));
        assert!(a.contains(&4));

        // `HashSet::insert()` returns false if there was a value already present.
        assert!(b.insert(4), "Value 4 is already in set B!");
        // FIXME ^ Comment out this line

        b.insert(5);

        // If a collection's element type implements `Debug`,
        // then the collection implements `Debug`.
        // It usually prints its elements in the format `[elem1, elem2, ...]`
        print!("A: {:?}\n", a);
        print!("B: {:?}\n", b);

        // Print [1, 2, 3, 4, 5] in arbitrary order
        print!("Union: {:?}\n", a.union(&b).collect::<Vec<&i32>>());

        // This should print [1]
        print!("Difference: {:?}\n", a.difference(&b).collect::<Vec<&i32>>());

        // Print [2, 3, 4] in arbitrary order.
        print!("Intersection: {:?}\n", a.intersection(&b).collect::<Vec<&i32>>());

        // Print [1, 5]
        print!("Symmetric Difference: {:?}\n",
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1 = HashSet::from(["aaa11", "aaa22", "aaa33"]);
    let mut bb1 = HashSet::from(["bbb11", "bbb22", "bbb33"]);
    let mut cc1 = HashSet::from(["ccc11", "ccc22", "ccc33"]);

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1: HashSet<String> = vec!["aaa11", "aaa22", "aaa33"].into_iter().collect();
    let mut bb1: HashSet<String> = vec!["bbb11", "bbb22", "bbb33"].into_iter().collect();
    let mut cc1: HashSet<String> = vec!["ccc11", "ccc22", "ccc33"].into_iter().collect();
    
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
    let mut bb1: HashSet<String> = ["bbb11", "bbb22", "bbb33"].iter().map(|&ss| ss.to_owned()).collect();
    let mut cc1: HashSet<String> = ["ccc11", "ccc22", "ccc33"].iter().map(|&ss| ss.to_owned()).collect();
    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
use std::collections::HashSet;
let a = HashSet::from([1, 2, 3]);
let b = HashSet::from([4, 2, 3, 4]);

// Print 1, 2, 3, 4 in arbitrary order.
for x in a.union(&b) {
    print!("{}\n", x);
}

let union: HashSet<_> = a.union(&b).collect();
assert_eq!(union, [1, 2, 3, 4].iter().collect());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    print!("map3 (empty):            {:?}\n", map3);

    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    print!("map3 (insert some):      {:?}\n", map3);

    let mut aa2: HashSet<String> = ["aaaa77", "aaaa88"].iter().map(|&ss| ss.to_owned()).collect();
    print!("map3 (insert some):      {:?}\n", map3);
    aa2.union(&aa1);    
    print!("aa2 U aa1                {:?}\n", map3);
    
    match map3.get("aa1") {
        Some(val) => {
            
            print!("val                      {:?}\n", val);
            print!("aa2 U aa1:               {:?}\n", aa2);
            //print!("aa2 U aa1:               {:?}\n", aa2.union(val).into_iter().collect::<HashSet>());
        }
        None => {}
    }

   // collect::<Vec<&i32>>() 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
A variable already borrowed as immutable was borrowed as mutable.

Erroneous code example:

```
fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    let y = &a; // a is borrowed as immutable.
    bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed
            //        as immutable
    print!("{}\n", y);
}
```

To fix this error, ensure that you don't have any other references to the
variable before trying to access it mutably:

```
fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    bar(a);
    let y = &a; // ok!
    print!("{}\n", y);
}
```

For more information on Rust's ownership system, take a look at the
[References & Borrowing][references-and-borrowing] section of the Book.

[references-and-borrowing]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
~
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
rustc --explain E0502 | A variable already borrowed as immutable was borrowed as mutable.

Erroneous code example:

    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        let y = &a;         // a is borrowed as immutable.
        bar(a);             // error: cannot borrow `*a` as mutable because `a` is also borrowed as immutable
        print!("{}\n", y);
    }

To fix this error, ensure that you don't have any other references to the
variable before trying to access it mutably:

    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        bar(a);
        let y = &a; // ok!
        print!("{}\n", y);
    }

For more information on Rust's ownership system, take a look at the
[References & Borrowing][references-and-borrowing] section of the Book.

[references-and-borrowing]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
~
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */



η5»ξ g2_hash_maps_sets.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ g1_hash_maps_sets.rs  

#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};   // no need for self bec no io::BufReader, etc  
use std::path::Path;
use std::collections::{HashMap, HashSet};


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests

#[cfg(test)]
mod load_data_test {
    // use chrono::{};

    #[test]
    #[should_panic]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions: read_lines  print_lines

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {

    let ff = File::open(filename)?;
    Ok(BufReader::new(ff).lines())

}


fn print_lines(results: Result<Lines<BufReader<File>>>) {

    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                println!("    {}", val);
            }
        }
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 Starting: g1_hash_maps_sets.rs \n\n", C_LL);
    
    print!("{}📚 Testing HashMaps \n\n", C_LL);
    let mut map1: HashMap<&str, HashSet<String>> = HashMap::new();
    map1.insert("aaa", ["bbbb"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("bbb", ["cccc"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("ccc", ["dddd"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("ddd", ["eeee"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("eee", ["ffff"].iter().map(|&ss| ss.to_owned()).collect());
    map1.insert("bbb", ["xxxx"].iter().map(|&ss| ss.to_owned()).collect());
    let val1 = map1.get("ccc");
    println!("map1:                   {:?}", map1);
    println!("val1:                   {:?}", val1);

    print!("{}📚 Testing HashMaps with Vectors \n\n", C_LL);
    let mut map2 = HashMap::new();
    map2.insert("aaa", HashSet::from(["top", "aaaaa"]));
    map2.insert("bbb", HashSet::from(["top", "bbbbb"]));
    let val2 = map2.get("bbb");
    println!("map2:                   {:?}", map2);
    println!("val2:                   {:?}", val2);

    print!("{}📚 Testing HashMaps with HashSets \n\n", C_LL);
    let aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
    let bb1: HashSet<String> = ["bbb11", "bbb22", "bbb33"].iter().map(|&ss| ss.to_owned()).collect();
    let cc1: HashSet<String> = ["ccc11", "ccc22", "ccc33"].iter().map(|&ss| ss.to_owned()).collect();
    let dd1: HashSet<String> = ["ddd11", "ddd22", "ddd33"].iter().map(|&ss| ss.to_owned()).collect();
    let ee1: HashSet<String> = ["eee11", "eee22", "eee33"].iter().map(|&ss| ss.to_owned()).collect();
    for xx in aa1.union(&bb1) {
        println!("{}", xx);
    }
    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    println!("map3 (insert some):      {:?}", map3);

    let aa3: HashSet<String> = ["aaa77", "aaa88", "aaa99"].iter().map(|&ss| ss.to_owned()).collect();
    let aa4: HashSet<String> = (aa3.union(&dd1)).map(|ss| ss.to_owned()).collect();    
    map3.insert("aa4", aa4);
    println!("map3:                    {:?}", map3);

    match map3.get("aa1") {
        Some(val) => {
            println!("val                      {:?}", val);
            let aa5: HashSet<String> = (val.union(&ee1)).map(|ss| ss.to_owned()).collect();
            println!("aa2 U aa1:               {:?}", aa5);
            map3.insert("aa1", aa5);
            println!("map3:                    {:?}", map3);
        }
        None => {}
    }

    
}



/*λ The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
let s: String = "abcdefg".to_owned();
let s_slice: &str = &s[..];  // take a full slice of the string

    let mut aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
            //println!("aa2 U aa1:               {:?}", aa2.union(val).into_iter().collect::<HashSet>());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    let mut map3 = HashMap::new();
    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    println!("map3 (insert some):      {:?}", map3);

    let mut aa3 = HashSet::from(["aaa77", "aaa88", "aaa99"]);
    let aa4 = aa3.union(&aa1).collect().to_string();    
    println!("aa3 U aa1                {:?}", aa4);
    map3.insert("aa1", aa4);
    println!("map3 (insert some):      {:?}", map3);

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
use std::io::Read;

    let aa4: HashSet<_> = aa3.union(&aa1).to_string().collect();    
    let diff: HashSet<_> = a.difference(&b).collect();
How to type cast from std::collections::hash_set::Union  to HashSet 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Doesnt Work 
fn read_lines<P>(filename: &dyn AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>> {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
where P: AsRef<Result<Lines<BufReader<File>>>>, {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    if let Ok(lines) = results {
        for line in lines {
            if let Ok(val) = line {
                println!("    {}", val);
            }
        }
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
#[derive(debug)]
struct One2many {
    one: 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Try all of these in the following example:

    use std::collections::HashSet;

    fn main() {
        let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
        let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

        assert!(a.insert(4));
        assert!(a.contains(&4));

        // `HashSet::insert()` returns false if there was a value already present.
        assert!(b.insert(4), "Value 4 is already in set B!");
        // FIXME ^ Comment out this line

        b.insert(5);

        // If a collection's element type implements `Debug`,
        // then the collection implements `Debug`.
        // It usually prints its elements in the format `[elem1, elem2, ...]`
        println!("A: {:?}", a);
        println!("B: {:?}", b);

        // Print [1, 2, 3, 4, 5] in arbitrary order
        println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

        // This should print [1]
        println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

        // Print [2, 3, 4] in arbitrary order.
        println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

        // Print [1, 5]
        println!("Symmetric Difference: {:?}",
                 a.symmetric_difference(&b).collect::<Vec<&i32>>());
    }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1 = HashSet::from(["aaa11", "aaa22", "aaa33"]);
    let mut bb1 = HashSet::from(["bbb11", "bbb22", "bbb33"]);
    let mut cc1 = HashSet::from(["ccc11", "ccc22", "ccc33"]);

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1: HashSet<String> = vec!["aaa11", "aaa22", "aaa33"].into_iter().collect();
    let mut bb1: HashSet<String> = vec!["bbb11", "bbb22", "bbb33"].into_iter().collect();
    let mut cc1: HashSet<String> = vec!["ccc11", "ccc22", "ccc33"].into_iter().collect();
    
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut aa1: HashSet<String> = ["aaa11", "aaa22", "aaa33"].iter().map(|&ss| ss.to_owned()).collect();
    let mut bb1: HashSet<String> = ["bbb11", "bbb22", "bbb33"].iter().map(|&ss| ss.to_owned()).collect();
    let mut cc1: HashSet<String> = ["ccc11", "ccc22", "ccc33"].iter().map(|&ss| ss.to_owned()).collect();
    
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
use std::collections::HashSet;
let a = HashSet::from([1, 2, 3]);
let b = HashSet::from([4, 2, 3, 4]);

// Print 1, 2, 3, 4 in arbitrary order.
for x in a.union(&b) {
    println!("{}", x);
}

let union: HashSet<_> = a.union(&b).collect();
assert_eq!(union, [1, 2, 3, 4].iter().collect());

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let mut map3: HashMap<&str, HashSet<String>> = HashMap::new();
    println!("map3 (empty):            {:?}", map3);

    map3.insert("aa1", aa1);
    map3.insert("bb1", bb1);
    map3.insert("cc1", cc1);
    println!("map3 (insert some):      {:?}", map3);

    let mut aa2: HashSet<String> = ["aaaa77", "aaaa88"].iter().map(|&ss| ss.to_owned()).collect();
    println!("map3 (insert some):      {:?}", map3);
    aa2.union(&aa1);    
    println!("aa2 U aa1                {:?}", map3);
    
    match map3.get("aa1") {
        Some(val) => {
            
            println!("val                      {:?}", val);
            println!("aa2 U aa1:               {:?}", aa2);
            //println!("aa2 U aa1:               {:?}", aa2.union(val).into_iter().collect::<HashSet>());
        }
        None => {}
    }

   // collect::<Vec<&i32>>() 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */



η5»ξ main.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ main  Updated Mod─List: Opened d*_ files for data § DBs, Moved TDD -> t, Euler -> e, CodeWars -> c.  ι✧21․12․22✦06․11․23․  🌎η ✧22․06․21․ ✧22․03․27․ 

mod t7_tdd;                                         use crate::t7_tdd::{mod_main};

fn main() {
    mod_main()
}

/*λ The Code Pit

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// main  ι✧21․12․25✦16․50․24․  
/*
mod a0_aws1;                                        use crate::a0_aws1g::{mod_main};
mod b0_hello1;                                      use crate::b0_hello1g::{mod_main};
mod b1_basic_math0;                                 use crate::b1_basic_math0g::{mod_main};
mod b2_basic_strings0;                              use crate::b2_basic_strings0g::{mod_main};
mod b3_guess_game0;                                 use crate::b3_guess_game0g::{mod_main};
mod b4_arrays_vectors_tuples0;                      use crate::b4_arrays_vectors_tuples0g::{mod_main};
mod b5_functions_closures0;                         use crate::b5_functions_closures0g::{mod_main};
mod b6_owners_pointers_refs0;                       use crate::b6_owners_pointers_refs0g::{mod_main};
mod b7_structs_traits0;                             use crate::b7_structs_traits0g::{mod_main};
mod b7_structs_traits1;                             use crate::b7_structs_traits1g::{mod_main};
mod c0_time_date_zone0;                             use crate::c0_time_date_zone0g::{mod_main};
mod c1_time_date_zone0;                             use crate::c1_time_date_zone0g::{mod_main};
mod c2_time_date_zone0;                             use crate::c2_time_date_zone0g::{mod_main};

mod d__ PREFIX IS OPEN for Play DATA § DBs 
mod d0_simple_csv_in.csv;                           use crate::d0_simple_csv_in.csv::{mod_main};
mod d0_simple_csv_out.txt;                          use crate::d0_simple_csv_out.txt::{mod_main};
mod d1_timedate_in.txt;                             use crate::d1_timedate_in.txt::{mod_main};
mod d2_xls_data.numbers;                            use crate::d2_xls_data.numbers::{mod_main};
mod d2_xls_data_in.txt;                             use crate::d2_xls_data_in.txt::{mod_main};
mod d3_one_to_many_in.csv;                          use crate::d3_one_to_many_in.csv::{mod_main};
mod d4_xls2md;                                      use crate::d4_xls2mdg::{mod_main};
mod d5_md2xls;                                      use crate::d5_md2xlsg::{mod_main};

mod e__ PREFIX IS OPEN for PROJECT EULER FILES      
mod e001_sum_multiples;                             use crate::e001_sum_multiplesg::{mod_main};

mod f0_python_and_rust0;                            use crate::f0_python_and_rust0g::{mod_main};
mod f1_python_and_rust0;                            use crate::f1_python_and_rust0g::{mod_main};
mod f2_give_and_take;                               use crate::f2_give_and_takeg::{mod_main};
mod f3_borrow_or_clone;                             use crate::f3_borrow_or_cloneg::{mod_main};
mod g0_load_data;                                   use crate::g0_load_datag::{mod_main};
mod g1_hash_maps_sets;                              use crate::g1_hash_maps_setsg::{mod_main};
mod g2_hash_maps_sets;                              use crate::g2_hash_maps_setsg::{mod_main};

mod t__ PREFIX IS OPEN for TESTING § TDD 
mod t1_tdd;                                         use crate::t1_tddg::{mod_main};
mod t2_tdd_mocks;                                   use crate::t2_tdd_mocksg::{mod_main};
mod t7_tdd;                                         use crate::t7_tdd::{mod_main};
mod t8_tdd;                                         use crate::t8_tdd::{mod_main};
mod t9_tdd;                                         use crate::t9_tdd::{mod_main};

mod w__ PREFIX IS OPEN for CODE WARS FILES      
mod w001_1;                                         use crate::w001_1::{mod_main};

End Of The Code Pit */

η5»ξ t1_tdd.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ t1_tdd.rs  ι․22․05․20✦08․10․28․ 🌎η  

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;
use chrono_tz::US::Pacific;


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]
mod parse_test {
    // use chrono::{};

    #[test]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }


}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 STARTING: a0_hello.rs \n\n", C_LL);

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}


fn exp_2() -> i32 {
    2
}


fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


struct Rectangle {

    ll: u8,
    ww: u8
}


impl Rectangle {

    fn is_square(&self) -> bool {
        self.ll == self.ww
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  read, parse § write files

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn read_lines_from_file() {

    if let Ok(ff) = open_file("./data0.txt") {

        let lines = ff.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]                                        // only compiles module if: ∞ cargo test
mod parse_test {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    // use chrono::{DateTime, FixedOffset, TimeZone, NaiveDateTime, Utc, Local};
    // use chrono_tz::US::Pacific;
    // use chrono_tz::America::Chicago;

    #[test]
    #[should_panic]
    fn test_basic1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }


    #[test]
    fn test_basic2() {

        assert_eq!(super::exp_2(), super::add(1, 1));
        assert_ne!(super::exp_2(), super::add(1, 2));
    }


    #[test]
    fn test_struct1() {

        let rr = super::Rectangle {
            ll: 20,
            ww: 20
        };

        assert!(rr.is_square());
    }


    #[test]
    fn test_parse_method() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));
    }


    #[test]
    #[ignore]
    // test DateTime::parse_from_{ str, rfc2822, rfc3339}
    fn test_parse_from() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

        assert_ne!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+08:00"), Ok(fixed_dt.clone()));
    }


    #[test]
    // test chrono::Utc.datetime_from_strm
    fn test_datetime_from_str() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);

        assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
        assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

        // oops, the year is missing!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
        // oops, the format string does not include the year at all!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
        // oops, the weekday is incorrect!
        assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {

    print!("{}📚 STARTING: t1_tdd.rs - Test Driven Development  \n\n", C_LL);

    // read_lines_from_file();

    print!("{}📚 Testing  \n\n", C_LL);

    print!("\ndone!\n");



}

/* THE CODE PIT

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */

η5»ξ t2_tdd_mocks.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ t1_tdd_mock.rs  TEMPLATE ONLY (NO SIGNATURE)

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;
use chrono_tz::US::Pacific;


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]
mod parse_test {
    // use chrono::{};

    #[test]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }


}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 STARTING: a0_hello.rs \n\n", C_LL);

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}


fn exp_2() -> i32 {
    2
}


fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


struct Rectangle {

    ll: u8,
    ww: u8
}


impl Rectangle {

    fn is_square(&self) -> bool {
        self.ll == self.ww
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// read, parse § write files

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn read_lines_from_file() {

    if let Ok(ff) = open_file("./data0.txt") {

        let lines = ff.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]                                        // only compiles module if: ∞ cargo test
mod parse_test {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    // use chrono::{DateTime, FixedOffset, TimeZone, NaiveDateTime, Utc, Local};
    // use chrono_tz::US::Pacific;
    // use chrono_tz::America::Chicago;

    #[test]
    #[should_panic]
    fn test_basic1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }


    #[test]
    fn test_basic2() {

        assert_eq!(super::exp_2(), super::add(1, 1));
        assert_ne!(super::exp_2(), super::add(1, 2));
    }


    #[test]
    fn test_struct1() {

        let rr = super::Rectangle {
            ll: 20,
            ww: 20
        };

        assert!(rr.is_square());
    }


    #[test]
    fn test_parse_method() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));
    }


    #[test]
    #[ignore]
    // test DateTime::parse_from_{ str, rfc2822, rfc3339}
    fn test_parse_from() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

        assert_ne!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+08:00"), Ok(fixed_dt.clone()));
    }


    #[test]
    // test chrono::Utc.datetime_from_strm
    fn test_datetime_from_str() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);

        assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
        assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

        // oops, the year is missing!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
        // oops, the format string does not include the year at all!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
        // oops, the weekday is incorrect!
        assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {

    print!("{}📚 STARTING: t1_tdd_mocks.rs - TDD Mocking \n\n", C_LL);

    // read_lines_from_file();

    print!("{}📚 Testing  \n\n", C_LL);



    print!("\ndone!\n");


}

/* THE CODE PIT

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */


η5»ξ t7_tdd.rs
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

/*
    fn main() {
        let name = "Peter";
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
η5»ξ t8_tdd.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ a4_tdd.rs  [ιδ21.12.22 τ08:48:42]

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;
use chrono_tz::US::Pacific;


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]
mod parse_test {
    // use chrono::{};

    #[test]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }


}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 STARTING: a0_hello.rs \n\n", C_LL);

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}


fn exp_2() -> i32 {
    2
}


fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


struct Rectangle {

    ll: u8,
    ww: u8
}


impl Rectangle {

    fn is_square(&self) -> bool {
        self.ll == self.ww
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// read, parse § write files

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn read_lines_from_file() {

    if let Ok(ff) = open_file("./data0.txt") {

        let lines = ff.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]                                        // only compiles module if: ∞ cargo test
mod parse_test {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    // use chrono::{DateTime, FixedOffset, TimeZone, NaiveDateTime, Utc, Local};
    // use chrono_tz::US::Pacific;
    // use chrono_tz::America::Chicago;

    #[test]
    #[should_panic]
    fn test_basic1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }


    #[test]
    fn test_basic2() {

        assert_eq!(super::exp_2(), super::add(1, 1));
        assert_ne!(super::exp_2(), super::add(1, 2));
    }


    #[test]
    fn test_struct1() {

        let rr = super::Rectangle {
            ll: 20,
            ww: 20
        };

        assert!(rr.is_square());
    }


    #[test]
    fn test_parse_method() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));
    }


    #[test]
    #[ignore]
    // test DateTime::parse_from_{ str, rfc2822, rfc3339}
    fn test_parse_from() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

        assert_ne!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+08:00"), Ok(fixed_dt.clone()));
    }


    #[test]
    // test chrono::Utc.datetime_from_strm
    fn test_datetime_from_str() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);

        assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
        assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

        // oops, the year is missing!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
        // oops, the format string does not include the year at all!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
        // oops, the weekday is incorrect!
        assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {

    print!("{}📚 STARTING: a4_tdd.rs - Test Driven Development  \n\n", C_LL);

    // read_lines_from_file();

    print!("📚 Testing time, date and zones \n\n");
    println!("{:?}", chrono::offset::Local::now());
    println!("{:?}", chrono::offset::Utc::now());

    print!("📚 Testing time \n\n");
    let tnow = Local::now();
    let tutc = Utc::now();

    print!("local time: {}\n", tnow);
    print!("UTC time:   {}\n", tutc);

    print!("local time: {:?}\n", tnow);
    print!("UTC time:   {:?}\n", tutc);

    print!("📚 Testing Converting from one time-zone to another \n\n");

    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);


    print!("{}📚 Testing parsing from rfc2822 \n\n", C_LL);

    let tt = DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());

    let tt = DateTime::parse_from_rfc2822("28 Nov 2014 21:00:09 +0900");
    print!("28 Nov 2014 21:00:09 +0900:         {}\n", tt.unwrap());


    print!("{}📚 Testing DateTime parsing-from-string \n\n", C_LL);

    let tt = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    print!("2014-11-28 21:00:09 +09:00:         {}\n", tt.expect("oops!"));

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0900", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.expect("oops!"));

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0000", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 :         {}\n", tt.unwrap());

    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 +0000", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());

    let tt = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z");
    print!("5.8.1994 8:00 am +0000:             {}\n", tt.unwrap());

    // Note that the Z-Offset of: -0800, refers to the local time and the time diference to UTC
    // e.g., 1200 -08:00, means that it's noon in the local zone and 4:00 am in UTC.
    print!("{}📚 Testing parsing-from-string in CST Time Zone (UTC - 6) \n\n", C_LL);

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 -0600", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());

    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 -0600", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());


    print!("{}📚 Testing NaiveDateTime parsing-from-string \n\n", C_LL);

    let tt = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S");
    print!("23:56:04:                           {}\n", tt.unwrap());

    let tt = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d");
    print!("2015-09-05:                         {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
    print!("2015-09-05 23:56:04:                {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("23:56:04 2015-09-05", "%H:%M:%S %Y-%m-%d");
    print!("23:56:04 2015-09-05:                {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("23 56 04 2015 Aug 05", "%H %M %S %Y %b %d");
    print!("23 56 04 2015 Aug 05:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("23 56 04 Aug 05 2015", "%H %M %S %b %d %Y");
    print!("23 56 04 Aug 05 2015:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Aug 05 23 56 04 2015", "%b %d %H %M %S %Y");
    print!("Aug 05 23 56 04 2015:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Aug 05 23:56:04 2015", "%b %d %H:%M:%S %Y");
    print!("Aug 05 23:56:04 2015:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Nov 27 12:00:09 2021", "%b %d %H:%M:%S %Y");
    print!("Nov 27 12:00:09 2021:               {}\n", tt.unwrap());


    let tt = NaiveDateTime::parse_from_str("Fri Nov 28 12:00:09 2014", "%a %b %d %T %Y");
    print!("Fri Nov 28 12:00:09 2014:           {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Sat Nov 29 12:00:09 2014", "%a %b %d %T %Y");
    print!("Sat Nov 29 12:00:09 2014:           {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Dec 18, 1998 19.08;", "%b %d, %Y %H.%M;");
    print!("Dec 18, 1998 19.08:                 {}\n", tt.unwrap());


    print!("\ndone!\n");


}

/* THE CODE PIT

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Moved  => c0_..


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */
η5»ξ t9_tdd.rs
//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ a5_tdd.rs  [ιδ21.12.22 τ08:48:42]

#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use chrono::prelude::*;
use chrono_tz::US::Pacific;


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]
mod parse_test {
    // use chrono::{};

    #[test]
    fn test_fail() {

        let ok: bool = false;
        assert!(ok);
    }


}

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// Constants § Types

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";




//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {
    print!("{}📚 STARTING: a0_hello.rs \n\n", C_LL);

}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ functions

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}


fn exp_2() -> i32 {
    2
}


fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}


struct Rectangle {

    ll: u8,
    ww: u8
}


impl Rectangle {

    fn is_square(&self) -> bool {
        self.ll == self.ww
    }
}



//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// read, parse § write files

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("data1.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn open_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn read_lines_from_file() {

    if let Ok(ff) = open_file("./data0.txt") {

        let lines = ff.lines();

        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ unit tests
#[cfg(test)]                                        // only compiles module if: ∞ cargo test
mod parse_test {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    // use chrono::{DateTime, FixedOffset, TimeZone, NaiveDateTime, Utc, Local};
    // use chrono_tz::US::Pacific;
    // use chrono_tz::America::Chicago;

    #[test]
    #[should_panic]
    fn test_basic1() {

        let ok: bool = true;
        assert!(ok);
        panic!("ahh!")
    }


    #[test]
    fn test_basic2() {

        assert_eq!(super::exp_2(), super::add(1, 1));
        assert_ne!(super::exp_2(), super::add(1, 2));
    }


    #[test]
    fn test_struct1() {

        let rr = super::Rectangle {
            ll: 20,
            ww: 20
        };

        assert!(rr.is_square());
    }


    #[test]
    fn test_parse_method() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));
    }


    #[test]
    #[ignore]
    // test DateTime::parse_from_{ str, rfc2822, rfc3339}
    fn test_parse_from() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"), Ok(fixed_dt.clone()));
        assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

        assert_ne!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+08:00"), Ok(fixed_dt.clone()));
    }


    #[test]
    // test chrono::Utc.datetime_from_strm
    fn test_datetime_from_str() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);

        assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
        assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

        // oops, the year is missing!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
        // oops, the format string does not include the year at all!
        assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
        // oops, the weekday is incorrect!
        assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ mod_main
pub fn mod_main() {

    print!("{}📚 STARTING: a5_tdd.rs - Test Driven Development  \n\n", C_LL);

    // read_lines_from_file();

    print!("📚 Testing time, date and zones \n\n");
    println!("{:?}", chrono::offset::Local::now());
    println!("{:?}", chrono::offset::Utc::now());

    print!("📚 Testing time \n\n");
    let tnow = Local::now();
    let tutc = Utc::now();

    print!("local time: {}\n", tnow);
    print!("UTC time:   {}\n", tutc);

    print!("local time: {:?}\n", tnow);
    print!("UTC time:   {:?}\n", tutc);

    print!("📚 Testing Converting from one time-zone to another \n\n");

    let tz1 = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
    print!("From Pacific Time-Zone: {:?}\n", tz1);


    print!("{}📚 Testing parsing from rfc2822 \n\n", C_LL);

    let tt = DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());

    let tt = DateTime::parse_from_rfc2822("28 Nov 2014 21:00:09 +0900");
    print!("28 Nov 2014 21:00:09 +0900:         {}\n", tt.unwrap());


    print!("{}📚 Testing DateTime parsing-from-string \n\n", C_LL);

    let tt = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    print!("2014-11-28 21:00:09 +09:00:         {}\n", tt.expect("oops!"));

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0900", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.expect("oops!"));

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 +0000", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 :         {}\n", tt.unwrap());

    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 +0000", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());

    let tt = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z");
    print!("5.8.1994 8:00 am +0000:             {}\n", tt.unwrap());

    // Note that the Z-Offset of: -0800, refers to the local time and the time diference to UTC
    // e.g., 1200 -08:00, means that it's noon in the local zone and 4:00 am in UTC.
    print!("{}📚 Testing parsing-from-string in CST Time Zone (UTC - 6) \n\n", C_LL);

    let tt = DateTime::parse_from_str("Fri, 28 Nov 2014 21:00:09 -0600", "%a, %d %b %Y %H:%M:%S %z");
    print!("Fri, 28 Nov 2014 21:00:09 +0900:    {}\n", tt.unwrap());

    let tt = DateTime::parse_from_str("Dec 18, 1998 19.08 -0600", "%b %d, %Y %H.%M %z");
    print!("Dec 18, 1998 19.08 +0000:           {}\n", tt.unwrap());


    print!("{}📚 Testing NaiveDateTime parsing-from-string \n\n", C_LL);

    let tt = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S");
    print!("23:56:04:                           {}\n", tt.unwrap());

    let tt = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d");
    print!("2015-09-05:                         {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
    print!("2015-09-05 23:56:04:                {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("23:56:04 2015-09-05", "%H:%M:%S %Y-%m-%d");
    print!("23:56:04 2015-09-05:                {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("23 56 04 2015 Aug 05", "%H %M %S %Y %b %d");
    print!("23 56 04 2015 Aug 05:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("23 56 04 Aug 05 2015", "%H %M %S %b %d %Y");
    print!("23 56 04 Aug 05 2015:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Aug 05 23 56 04 2015", "%b %d %H %M %S %Y");
    print!("Aug 05 23 56 04 2015:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Aug 05 23:56:04 2015", "%b %d %H:%M:%S %Y");
    print!("Aug 05 23:56:04 2015:               {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Nov 27 12:00:09 2021", "%b %d %H:%M:%S %Y");
    print!("Nov 27 12:00:09 2021:               {}\n", tt.unwrap());


    let tt = NaiveDateTime::parse_from_str("Fri Nov 28 12:00:09 2014", "%a %b %d %T %Y");
    print!("Fri Nov 28 12:00:09 2014:           {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Sat Nov 29 12:00:09 2014", "%a %b %d %T %Y");
    print!("Sat Nov 29 12:00:09 2014:           {}\n", tt.unwrap());

    let tt = NaiveDateTime::parse_from_str("Dec 18, 1998 19.08;", "%b %d, %Y %H.%M;");
    print!("Dec 18, 1998 19.08:                 {}\n", tt.unwrap());


    print!("\ndone!\n");


}

/* THE CODE PIT

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Moved  => c0_..


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

END OF THE CODE PIT */
