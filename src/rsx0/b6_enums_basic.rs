//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ b6_enums_basic.rs
//  ✨λ n8_enums_basic.rs  ι✧21․12․15✦05․21․30․ 🌎η ✧22․12․08․✧22․12․03․✧22․11․27․✧22․11․12․✧22․10․22․✧22․08․19․✧22․04․21․✧21․12․15․
#![allow(dead_code)]
use std::iter::zip;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`
#[cfg(test)]
mod test_regex {
    use super::*;

    #[test]
    fn test_preamble() {
        print!("TEST_STR: {TEST_STR} \n")
    }
}


//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  Traits, Constants, Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

///λ enum_basics mimics Option and Result; playing with enums 
fn enum_basics() {
    enum MyOption<T> {
        None,
        Some(T),
    }
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }
    struct Null {
    }
}


///λ stg_to_int_unwrap: converts a Vector of Strings to a Vector of i32 values - panics on conversion error 
///  Note the Turbo─Fish Operator `::<>` 👍υ Super Cool; also [stg = String] and [stl = String Literal] 
fn stg_to_int_unwrap(vs: &Vec<String>) -> Vec<i32> {

    vs.into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

///λ stg_to_int_expect: same as above, except it provides an error msg as it panics 
fn stg_to_int_expect(vs: &Vec<String>) -> Vec<i32> {

    vs.into_iter().map(|x| x.parse::<i32>().expect("*** stg_to_int_expect::parse() ***")).collect()
}

///λ stg_to_int_unwrap_or: same as above, except it treats parse errors as `0` values
fn stg_to_int_unwrap_or(vs: &Vec<String>) -> Vec<i32> {

    vs.into_iter().map(|x| x.parse::<i32>().unwrap_or(0)).collect()
}

///λ converts a Vector of i32 values to a Vector of Strings
fn int_to_stg(vi: &Vec<i32>) -> Vec<String> {

    vi.iter().map(|x| x.to_string()).collect()
}

///λ stl_to_stg converts a Vector of string-literals to a Vector of Strings
fn stl_to_stg(vi: &Vec<&str>) -> Vec<String> {

    vi.iter().map(|x| x.to_string()).collect()
}

///λ stg_to_int_option: same as above, except it converts parse Result -> Option using ok()
fn stg_to_int_option(vs: &Vec<String>) -> Option<Vec<i32>> {

    vs.into_iter().map(|x| x.parse::<i32>().ok()).collect()
}



/// dot_int_arr1 returns the dot product of 2 arrays (slices) of ints, using imperative style 
fn dot_int_arr1(a1: &[i32], a2: &[i32]) -> i32 {

    let mut res = 0;
    for (vi1, vi2) in a1.iter().zip(a2.iter()) {
        res += vi1 * vi2;
    }
    res
}

/// dot_int_arr2 returns the dot product of 2 arrays (slices) of ints using map - and skiping the first 3 tuples
fn dot_int_arr2(a1: &[i32], a2: &[i32]) -> i32 {

    let iter = zip(
        a1.iter(),                              // .into_iter() works also
        a2.iter(),
    );
    iter.map(|(x, y)| x * y).skip(3).sum()
}

/// dot_int_vec1; same but for vectors - functional style - 
fn dot_int_vec1(vi1: &Vec<i32>, vi2: &Vec<i32>) -> i32 {

    let iter = zip( vi1.iter(), vi2.iter(),);
    iter.map(|(x, y)| x * y).sum()
}

/// dot_int_vec2; same but one─line functional style
fn dot_int_vec2(vi1: &Vec<i32>, vi2: &Vec<i32>) -> i32 {

    zip( vi1.iter(), vi2.iter(),).map(|(x, y)| x * y).sum()
}

///λ dot_stg_vec returns the dot product of 2 vectors of strings (stg) -> int, as a string, using unwrap_or() 
fn dot_stg_vec(vs1: &Vec<String>, vs2: &Vec<String>) -> String {

    let vi1 = stg_to_int_unwrap_or(&vs1);
    let vi2 = stg_to_int_unwrap_or(&vs2);
    let res = dot_int_vec2(&vi1, &vi2);

    res.to_string()
}


///λ dot_stl_vec returns the dot product of 2 vectors of string-literals (stl) -> Option<String>
fn dot_stl_vec(vs1: &Vec<&str>, vs2: &Vec<&str>) -> Option<String> {

    let v_stg1 = stl_to_stg(&vs1);
    let v_stg2 = stl_to_stg(&vs2);

    let vo1 = stg_to_int_option(&v_stg1);
    let vo2 = stg_to_int_option(&v_stg2);

    print!("vo1:                                                {:?} \n", vo1);
    print!("vo2:                                                {:?} \n", vo2);

    match vo1 {
        None => None,
        Some(vi1) => match vo2 {
            None => None,
            Some(vi2) => Some(dot_int_vec2(&vi1, &vi2).to_string()),
        }
    }
}



/*
fn int_to_stg(vs: Vec<&str>) -> Vec<String> {

    vs.iter().map(|x| x.int_to_stg()).collect()
}
*/


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), String> {
    print!("{}🎡𐡋 Checking: n8_enums_basic.rs  \n\n", C_LL);

    let ai1 = [1, 2, 3, 2, 2, 2, 2, 2];
    let ai2 = [2, 3, 1, 1, 1, 1, 1, 1];

    let vi1 = ai1.clone().to_vec();
    let vi2 = ai2.clone().to_vec();

    enum_basics();
    print!("dot_int_arr1(ai1, ai2):                             {}\n",  dot_int_arr1(&ai1, &ai2));
    print!("dot_int_arr2(ai1, ai2):                             {}\n",  dot_int_arr2(&ai1, &ai2));
    print!("dot_int_vec1:                                       {}      skipping fist 3 tuples \n", dot_int_vec1(&vi1, &vi2)); 
    print!("dot_int_vec2(vi1, vi2):                             {}\n",  dot_int_vec2(&vi1,  &vi2)); 
    print!("dot_stg_vec(int_to_stg(vi1), int_to_stg(vi2):       {}\n",  dot_stg_vec(&int_to_stg(&vi1), &int_to_stg(&vi2))); 

    let vs1 = ["1", "2", "3", "2", "2", "2", "2", "2"].to_vec();
    let vs2 = ["2", "3", "1", "1", "1", "1", "1", "abc"].to_vec();
    let vs3 = ["2", "3", "1", "1", "1", "1", "1", "1"].to_vec();

    print!("dot_stg_vec(stl_to_stg(vs1), stl_to_stg(vs2)):      {}\n",  dot_stg_vec(&stl_to_stg(&vs1), &stl_to_stg(&vs2))); 
    print!("dot_stl_vec(vs1, vs2):                              {:?}\n",  dot_stl_vec(&vs1, &vs2)); 

    Ok(())
    // panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ dot_stl_vec returns the dot product of 2 vectors of string-literals -> ints, as an Option 
fn dot_stl_vec(vs1: &Vec<&str>, vs2: &Vec<&str>) -> String {

    let v_stg1 = stl_to_stg(&vs1);
    let v_stg2 = stl_to_stg(&vs2);

    let vi1 = stg_to_int_unwrap_or(&v_stg1);
    let vi2 = stg_to_int_unwrap_or(&v_stg2);
    let res = dot_int_vec2(&vi1, &vi2);

    res.to_string()
}



•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("dot_stg_vec:   {}\n",  dot_stg_vec(int_to_stg(vi1.clone()), int_to_stg(vi2.clone()))); 
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit


