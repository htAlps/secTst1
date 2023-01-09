// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b1-basic-math.rs [ιδ21.12.22 τ08:48:42]

#![allow(mixed_script_confusables)]  // note the '!', otherwise we get   => allow(mixed_script_confusables) is ignored unless specified at crate level
// #![allow(dead_code)]


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

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

    Ok(())
    // panic!("for: No Reason");
}

