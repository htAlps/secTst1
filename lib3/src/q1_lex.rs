// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ lib3::q1_lex  ι✧21․12․25✦16․50․24․  🌎η ✧22․11․16․✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․

use std::fmt;
use std::collections::HashSet;


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Default Strings - obj: to shorten the essential String delimiters - using explicitly derived style

#[derive(Clone)]
enum SubType {
    Cm,         // Comma
    Dot,        // Dot
    Col,        // Colon
    Els,        // Else
    End,        // End
    Eof,        // Eof
    Er,         // Error
    Fld,        // Field
    Id,         // Id
    Iff,        // If
    Lb1,        // LeftSquareBr
    Lb2,        // LeftDoubleBr
    Ls1,        // LeftCurlySqrBr
    Ls2,        // LeftDoubleCurlyBr
    Nl,         // NewLine
    Num,        // Number
    Pip,        // Pipe
    Qt1,        // DoubleQuote
    Qt2,        // SingleQuote
    Qt3,        // TikQuote
    Rb1,        // RightSquareBr
    Rb2,        // RightDoubleSqrBr
    Rng,        // Range
    Rs1,        // RightCurlyBr
    Rs2,        // RightDoubleCurlyBr
    Sp,         // Space
    Stg,        // String
    Stl,        // StrLiteral
    Txt,        // Text
}


impl SubType {

    fn name(&self) -> String {
        match self {
            SubType::Cm    => "Comma".to_string(),               
            SubType::Dot   => "Dot".to_string(),                 
            SubType::Col   => "Colon".to_string(),               
            SubType::Els   => "Else".to_string(),                
            SubType::End   => "End".to_string(),                 
            SubType::Eof   => "Eof".to_string(),                 
            SubType::Er    => "Error".to_string(),               
            SubType::Fld   => "Field".to_string(),               
            SubType::Id    => "Id".to_string(),                  
            SubType::Iff   => "If".to_string(),                  
            SubType::Lb1   => "LeftSquareBr".to_string(),        
            SubType::Lb2   => "LeftDoubleBr".to_string(),        
            SubType::Ls1   => "LeftCurlySqrBr".to_string(),      
            SubType::Ls2   => "LeftDoubleCurlyBr".to_string(),   
            SubType::Nl    => "NewLine".to_string(),             
            SubType::Num   => "Number".to_string(),              
            SubType::Pip   => "Pipe".to_string(),                
            SubType::Qt1   => "DoubleQuote".to_string(),         
            SubType::Qt2   => "SingleQuote".to_string(),         
            SubType::Qt3   => "TikQuote".to_string(),            
            SubType::Rb1   => "RightSquareBr".to_string(),       
            SubType::Rb2   => "RightDoubleSqrBr".to_string(),    
            SubType::Rng   => "Range".to_string(),               
            SubType::Rs1   => "RightCurlyBr".to_string(),        
            SubType::Rs2   => "RightDoubleCurlyBr".to_string(),  
            SubType::Sp    => "Space".to_string(),               
            SubType::Stg   => "String".to_string(),              
            SubType::Stl   => "StrLiteral".to_string(),          
            SubType::Txt   => "Text".to_string(),                
        }
    }
}


struct Lexium {
    subtype:  SubType,
    value:    String,
}



impl Lexium {
    fn new(lst: SubType) -> Self {
        Lexium {
            subtype:  lst.clone(),
            value:    Self::new_val(&lst),
        }
    }

    fn new_val(lst: &SubType) -> String {
        match lst {
            SubType::Cm    => ",".to_string(),
            SubType::Dot   => ".".to_string(),
            SubType::Col   => ":".to_string(),
            SubType::Els   => "else".to_string(),
            SubType::End   => "end".to_string(),
            SubType::Eof   => "eof".to_string(),
            SubType::Er    => "".to_string(),
            SubType::Fld   => "".to_string(),
            SubType::Id    => "".to_string(),
            SubType::Iff   => "if".to_string(),
            SubType::Lb1   => "[".to_string(),
            SubType::Lb2   => "[[".to_string(),
            SubType::Ls1   => "{".to_string(),
            SubType::Ls2   => "{{".to_string(),
            SubType::Nl    => "\n".to_string(),
            SubType::Num   => "0".to_string(),
            SubType::Pip   => "|".to_string(),
            SubType::Qt1   => "\"".to_string(),
            SubType::Qt2   => "'".to_string(),
            SubType::Qt3   => "`".to_string(),
            SubType::Rb1   => "]".to_string(),
            SubType::Rb2   => "]]".to_string(),
            SubType::Rng   => "..".to_string(),
            SubType::Rs1   => "}".to_string(),
            SubType::Rs2   => "}}".to_string(),
            SubType::Sp    => " ".to_string(),
            SubType::Stg   => "".to_string(),
            SubType::Stl   => "".to_string(),
            SubType::Txt   => "".to_string(),
        }
    }


    fn t(&self) -> SubType {
        self.subtype.clone()
    }

    fn v(&self) -> String {
        self.value.clone()
    }
}


///λ Implements Display for Lexium so we can print individual lexia the way we want
impl fmt::Display for Lexium{
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        write!( ff, "{}", format!("{{{}: {}}}", &self.t().name(), &self.v()) )
    }
}


pub struct Lex {
    cm:   Lexium,
    dot:  Lexium,
    col:  Lexium,
    els:  Lexium,
    end:  Lexium,
    eof:  Lexium,
    er:   Lexium,
    fld:  Lexium,
    id:   Lexium,
    iff:  Lexium,
    lb1:  Lexium,
    lb2:  Lexium,
    ls1:  Lexium,
    ls2:  Lexium,
    nl:   Lexium,
    num:  Lexium,
    pip:  Lexium,
    qt1:  Lexium,
    qt2:  Lexium,
    qt3:  Lexium,
    rb1:  Lexium,
    rb2:  Lexium,
    rng:  Lexium,
    rs1:  Lexium,
    rs2:  Lexium,
    sp:   Lexium,
    stg:  Lexium,
    stl:  Lexium,
    txt:  Lexium,
}


impl Lex {
    pub fn new() -> Self {
        Lex {
            cm:   Lexium::new(SubType::Cm),
            dot:  Lexium::new(SubType::Dot),
            col:  Lexium::new(SubType::Col),
            els:  Lexium::new(SubType::Els),
            end:  Lexium::new(SubType::End),
            eof:  Lexium::new(SubType::Eof),
            er:   Lexium::new(SubType::Er),
            fld:  Lexium::new(SubType::Fld),
            id:   Lexium::new(SubType::Id),
            iff:  Lexium::new(SubType::Iff),
            lb1:  Lexium::new(SubType::Lb1),
            lb2:  Lexium::new(SubType::Lb2),
            ls1:  Lexium::new(SubType::Ls1),
            ls2:  Lexium::new(SubType::Ls2),
            nl:   Lexium::new(SubType::Nl),
            num:  Lexium::new(SubType::Num),
            pip:  Lexium::new(SubType::Pip),
            qt1:  Lexium::new(SubType::Qt1),
            qt2:  Lexium::new(SubType::Qt2),
            qt3:  Lexium::new(SubType::Qt3),
            rb1:  Lexium::new(SubType::Rb1),
            rb2:  Lexium::new(SubType::Rb2),
            rng:  Lexium::new(SubType::Rng),
            rs1:  Lexium::new(SubType::Rs1),
            rs2:  Lexium::new(SubType::Rs2),
            sp:   Lexium::new(SubType::Sp),
            stg:  Lexium::new(SubType::Stg),
            stl:  Lexium::new(SubType::Stl),
            txt:  Lexium::new(SubType::Txt),
        }
    }
}

///λ Implements Display for Lex so we can print a lex the way we want
impl fmt::Display for Lex{
    fn fmt(&self, ff: &mut fmt::Formatter) -> fmt::Result {
        write!( ff, "[ {} ]", 
            format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", 
                &self.cm,
                &self.dot,
                &self.col,
                &self.els,
                &self.end,
                &self.eof,
                &self.er,
                &self.fld,
                &self.id,
                &self.iff,
                &self.lb1,
                &self.lb2,
                &self.ls1,
                &self.ls2,
                &self.nl,
                &self.num,
                &self.pip,
                &self.qt1,
                &self.qt2,
                &self.qt3,
                &self.rb1,
                &self.rb2,
                &self.rng,
                &self.rs1,
                &self.rs2,
                &self.sp,
                &self.stg,
                &self.stl,
                &self.txt
            )
        )
    }
}



///λ check int-tests lib3's q1_lex module 
pub fn check() -> Result<(), String> {

    /*
    */
    let my_location = "q1_lex::check";
    print!("\n🎡𐡋 {my_location} \n");
    Ok(())                                                              // don't: panic!("for: No Reason");
}


//λ The Code Pit
/*
Check scratch_pad, a lot of code framgents moved there
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

pub fn check() -> Result<(), String> {

    let stg = prt_cmd();
    let my_location = "q1_lex::check";
    print!("{C_LL}🎡𐡋 {my_location} \n", );

    print!("🎡𐡋 prt_cmd \n");
    prt_cmd();

    let l = Lex::new();
    print!("l: Lex\n{l}");

    Ok(())
    // don't panic!("for: No Reason");
}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Recent Crates

mod s1_lex;                     use s1_lex::{check};
mod sysop_exec;                 use sysop_exec::{check};
mod a84_re_multiline;           use a84_re_multiline::{check};
mod a83_regex_basics;           use a83_regex_basics::{check};
mod a82_string_methods;         use a82_string_methods::{check};

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

impl Default for Lex {
    fn default() -> Self {
        let lex = Lex::new();
        lex
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

