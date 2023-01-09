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


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {



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

    Ok(())
    // panic!("for: No Reason");
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


