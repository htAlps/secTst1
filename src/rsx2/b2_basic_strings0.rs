// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b2-basic-strings.rs [ιδ21.12.22 τ08:48:42]


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

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

    Ok(())
    // panic!("for: No Reason");
}

