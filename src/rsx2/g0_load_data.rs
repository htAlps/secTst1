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




///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    print!("{}📚 Starting: g0_load_data.rs \n\n", C_LL);
    
    let results = read_lines("./d3_one_to_many_in.csv");
    print_lines(results);


    Ok(())
    // panic!("for: No Reason");
}




/*λ The Code Pit

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

End Of The Code Pit */


