// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ p2_apis_on_traits.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․08․19․ ✧22․04․21․ ✧21․11․07․
#![allow(dead_code)]
use std::{error::Error, time::Duration, thread::sleep};
use std::collections::{HashSet, HashMap};

/// Constants Types § Enums
const C_LL:         &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
const CLEAR_LINE:   &str = "\x1B[2J\x1B[1;1H";


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ Tests
#[cfg(test)]
mod fail_tests {

    // use chrono::{};
    #[test]
    #[should_panic]
    fn test_fail() {
        let ok: bool = false;
        assert!(ok);
    }
}

#[cfg(test)]
mod pass_tests {

    // iterate_through_array() iterates through an array 
    fn iterate_through_array() {
        let aa = [2, 1, 0];
        let mut aa_iter = aa.iter();
        assert_eq!(Some(&2), aa_iter.next()); 
        assert_eq!(Some(&1), aa_iter.next()); 
        assert_eq!(Some(&0), aa_iter.next()); 
        assert_eq!(None, aa_iter.next()); 
        assert_eq!(None, aa_iter.next()); 
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ Code

fn run_task(nn: &i32) {
    print!("\n {:?} \n", nn); 
    sleep(Duration::from_secs(1));
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// But we want a functional API iter().tracker_v1(),  i.e., an Iter tracker Extension: Iterator ⮕> TrackerV1 

pub struct TrackerV1<Iter> {                        // Iter ex: std::collections::hash_set::Iter
    iter: Iter,
    cnt: usize,
}

/// impl<Iter> TrackerV1<Iter> is a QUANTIFIED impl-block which associates methods with types;
/// we say: for all types `Iter` implement TrackerV1 of Iter 
impl<Iter> TrackerV1<Iter> {

    /// new is a static method because it doesnt take self as imput (since self does not exist at this time)
    pub fn new(iter: Iter) -> Self {
        TrackerV1{ iter: iter, cnt: 0 }
    }
}

impl<Iter> Iterator for TrackerV1<Iter>
where Iter: Iterator {
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(self.cnt));
        self.cnt += 1;
        self.iter.next()
    }
}

// trait IteratorTrackerV1Ext takes an iterator and returns a TrackerV1
trait IteratorTrackerV1Ext: Sized {      
    fn tracker_v1(self) -> TrackerV1<Self>;
}

impl<Iter> IteratorTrackerV1Ext for Iter
where Iter: Iterator {
    fn tracker_v1(self) -> TrackerV1::<Self> {
        TrackerV1::new(self)
    }
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// Next, we want to track bounded and unbounded iterators 

pub struct TrackerV2<Iter> {                        // Iter ex: std::collections::hash_set::Iter
    iter:   Iter,
    cnt:    usize,
    bound:  Option<usize>                           // unbounded = None, bounded = size of iterator 
}

impl<Iter> TrackerV2<Iter> 
where Iter: Iterator {
    pub fn new(iter: Iter) -> Self {
        TrackerV2{ iter: iter, cnt: 0, bound: None }
    }
}

impl<Iter> TrackerV2<Iter>                          // for bounded iterators we need 
where Iter: ExactSizeIterator {
    pub fn with_bound(mut self) -> Self {           // a function to get the size
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for TrackerV2<Iter>             // need to expand next to match for type of iterator 
where Iter: Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR_LINE);
        match self.bound {
            Some(bound) => print!("[{}{}]", "*".repeat(self.cnt), " ".repeat(bound - self.cnt)),
            None => print!("{}", "*".repeat(self.cnt)),
        };
        self.cnt += 1;
        self.iter.next()
    }
}

// trait IteratorTrackerV2Ext takes an iterator and returns a TrackerV2
trait IteratorTrackerV2Ext: Sized {      
    fn tracker_v2(self) -> TrackerV2<Self>;
}

impl<Iter> IteratorTrackerV2Ext for Iter
where Iter: Iterator {
    fn tracker_v2(self) -> TrackerV2::<Self> {
        TrackerV2::new(self)
    }
}


///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 p2_apis_on_traits.rs - functional style API \n\n", C_LL);
    // let _aa: Vec<i32> = [5, 4, 3, 2, 1];

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 tracking bounded and unbounded iterators \n", C_LL);
    
    let set1 = HashSet::from([35, 34, 33, 32]);
    for val in set1.iter().tracker_v2() {
        run_task(val);
    }

    let set1 = HashSet::from([35, 34, 33, 32]);
    for val in set1.iter().tracker_v2().with_bound() {
        run_task(val);
    }

    let mut map1 = HashMap::new();
    map1.insert(103, 3); map1.insert(102, 2); map1.insert(101, 1);
    for val in map1.iter().tracker_v2() {
        run_task(val.0)
    }
    
    let mut map1 = HashMap::new();
    map1.insert(203, 3); map1.insert(202, 2); map1.insert(201, 1);
    for val in map1.iter().tracker_v2().with_bound() {
        run_task(val.0)
    }
    
    for nn in (0 .. ).tracker_v2() {
        run_task(&nn);
    }

    print!("\n\n\nDone!\n"); 
    Ok(())
    // panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    // what we really want is an API with a functional look Like:
    let mut set1 = HashSet::from([35, 34, 33, 32]);
    for val in set1.iter().tracker_v1() {
        run_task(val);
    }

    // for nn in (0 .. ).tracker_v2().with_bound() {  // ⮕> 👎ς error[E0599]: the method `with_bound` exists for struct `TrackerV2<RangeFrom<{integer}>>`, but its trait bounds were not satisfied
    //     run_task(&nn);
    // }

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit



