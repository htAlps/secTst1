//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//  ✨λ a0_hello.rs


#![allow(dead_code)]

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
//λ functions


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
//λ check_mod
pub fn check_mod() {
    print!("{}📚 STARTING: a0_hello.rs \n\n", C_LL);

}


//λ The Code Pit
//λ The Code Pit
/*λ The Code Pit
/*λ The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
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


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */


//λ a0_hello.rs
// ✨λ a0_hello.rs
