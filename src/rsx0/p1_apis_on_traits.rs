// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ p1_apis_on_traits.rs  ι✧21․08․16✦06․40․20․ 🌎η ✧22․08․19․ ✧22․04․21․ ✧21․11․07․
#![allow(dead_code)]
use std::{error::Error, time::Duration, thread::sleep};
use std::collections::{HashMap, HashSet};

///  Constants Types § Enums
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

fn prt_chars(ss: &String) {
    print!("    ss.chars(): ");
    for val in ss.chars() {
        print!("{val} ");
    }
    print!("\n");
}

fn run_task0(_n: i32) {
    sleep(Duration::from_secs(1));
}

fn run_task1(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn run_task2(_n: usize) {
    sleep(Duration::from_secs(1));
}

fn run_task3(_n: (&i32, &i32)) {
    sleep(Duration::from_secs(1));
}

fn run_task4(nn: &i32) {
    print!("\n {:?} \n", nn); 
    sleep(Duration::from_secs(1));
}

fn track1_count(count: usize) {
    print!("\nrunning: \n");
    for ii in 1..count {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(ii));
        sleep(Duration::from_secs(3));
    }
}

fn track2_vec(vec: Vec<i32>) {
    print!("\nrunning: \n");
    let mut cnt: usize = 0;
    for ii in vec {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(cnt));
        cnt+=1;
        run_task0(ii);
    }
}

fn track3_vec(vec: Vec<i32>) {
    print!("\nrunning: \n");
    let mut cnt: usize = 0;
    for ii in vec {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(cnt));
        cnt+=1;
        run_task0(ii);
    }
}

/// track4_vec tracks runing ff over each element of struct: vec
fn track4_vec<T>(vec: Vec<T>, ff: fn(T) -> ()) {            // T is type Item (the type of the elements being iterated over)
    print!("\nrunning: \n");                                // ff is the function we are tracking 
    let mut cnt: usize = 0;
    for ii in vec {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(cnt));
        cnt+=1;
        ff(ii);
    }
}

/// track5_iter tracks running ff over each element of iterator-trait: iter
fn track5_iter<T, Iter>(iter: Iter, ff: fn(T) -> ())
where Iter: Iterator<Item = T> {                            // where Iter implements Iterator 
    print!("\nrunning: \n");
    let mut cnt: usize = 0;
    for val in iter {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(cnt));
        cnt+=1;
        ff(val);
    }
}

/// track6 is the beautifull version of above 
fn track6<Iter>(iter: Iter, ff: fn(Iter::Item) -> ())
where Iter: Iterator {                                      // where Iter implements Iterator 
    print!("\nrunning: \n");
    let mut cnt: usize = 0;
    for val in iter {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(cnt));
        cnt+=1;
        ff(val);
    }
}

/// track is the idiomatic Rust version  
fn track<T>(iter: T, ff: fn(T::Item) -> ())
where T: Iterator {                                         // where Iter implements Iterator 
    print!("\nrunning: \n");
    let mut cnt: usize = 0;
    for val in iter {
        print!("{}{}\n", CLEAR_LINE, "*".repeat(cnt));
        cnt+=1;
        ff(val);
    }
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// TrackerV1 - But we want a functional API iter().tracker_v1(),  i.e., an Iter tracker Extension: Iterator ⮕> TrackerV1 

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


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ check_mod is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn check_mod() -> Result<(), Box<dyn Error>> {

    print!("{}🎡𐡋 Testing: p1_apis_on_traits.rs \n\n", C_LL);
    // let _aa: Vec<i32> = [5, 4, 3, 2, 1];

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Testing run_tasks \n", C_LL);
    track1_count(5);
    let vec1 =  vec![3, 2, 1];
    track2_vec(vec1);

    let vec1 =  vec![5, 2, 1];
    track3_vec(vec1);
    
    let vec1 =  vec![3, 5, 1];
    track4_vec(vec1, run_task2);
    
    let arr1 =  [2, 1];
    track5_iter(arr1.iter(), run_task4);                    // Now we can run on any iterable object 
    
    let vec1 =  vec![3, 1];
    track5_iter(vec1.iter(), run_task4);
    
    let mut map1 = HashMap::new();
    map1.insert(3, 2); map1.insert(2, 1);
    track5_iter(map1.iter(), run_task3);
    
    let mut map1 = HashMap::new();
    map1.insert(2, 3); map1.insert(4, 2); map1.insert(1, 0);
    track(map1.iter(), run_task3);

    let set1 = HashSet::from([2, 4, 1]);
    track(set1.iter(), run_task4);

    let set1 = HashSet::from([7, 3, 5]);
    let iter1 = set1.iter();    
    track(iter1, run_task4);

    // First What we want from the API is:
    // 1. make a set
    let set1 = HashSet::from([15, 14, 13, 12]);
    // 2. make an iterator 
    let iter1 = set1.iter();    
    // 2. make a tracker 
    let trk1 = TrackerV1::new(iter1);
    // 3. loop through iterator 
    for val in trk1 {
         run_task4(val);
    }
    
    // symplifying above code 
    let set1 = HashSet::from([25, 24, 23, 22]);
    for val in TrackerV1::new(set1.iter()) {
        run_task4(val);
    }


    // •════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    // but this is not a good API design, what we really want is an API with a functional look Like: iter().tracker_v1().bounded()...
    print!("{}🎡𐡋 tracking bounded and unbounded iterators \n", C_LL);
    
    let set1 = HashSet::from([35, 34, 33, 32]);
    for val in set1.into_iter().tracker_v2() {                  // must use into_iter() to consume the array 
        run_task0(val);
    }

    let set1 = HashSet::from([35, 34, 33, 32]);
    for val in set1.into_iter().tracker_v2().with_bound() {     // must use into_iter() to consume the array 
        run_task0(val);
    }

    for nn in (0 .. ).tracker_v2() {
        run_task1(&nn);
    }

    print!("\n\n\nDone!\n"); 
    Ok(())
    // panic!("for: No Reason");
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
pub struct Tracker_v2<T> {
    cnt: usize,
    iter: dyn Iterator<Item = T>,
}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

    fn next(&mut self) -> Option<Item = T> {
        let cnt = 2;
        print!("{}{}\n", CLEAR_LINE, "*".repeat(self.cnt));
        self.iter.next()
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    
    // but this is not a good API design 
    // what we really want is an API with a functional look Like:
    let mut set1 = HashSet::from([35, 34, 33, 32]);
    for val in set1.iter().tracker() {
        run_task4(val);
        print!("\n{val} \n");
        sleep(Duration::from_secs(1));
    }

    print!("\n\n\nDone!\n"); 
    Ok(())
    
    // Now, What we want is:  1. make an iterator 

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit


