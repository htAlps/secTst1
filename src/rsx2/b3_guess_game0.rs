// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b3-guess-game.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.01]   

use std::io::stdin;


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

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

    Ok(())
    // panic!("for: No Reason");
}

