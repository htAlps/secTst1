// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b6-owners-pointers-refs.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.02] [δ22.01.01]

// use std::io::stdin;

fn sum_vector(vv: &Vec<i32>) -> i32 {

    // folds is an iterator adapter that takes an initial value and applies a closure to all the values of the vector
    let sum = vv.iter().fold(0, |mut sum, &xx| { sum += xx; sum });     // it seems xx ranges through the vector elements
    return sum;
}



///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b6-owners-pointers-refs.rs \n\n", C_LL);

    // One binding for each resource (not a primitive val or var)
    print!("{}📚 Testing loosing a binding \n\n", C_LL);

    let mut v1 = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];     // 89. 144, ...
    print!("v1[4..7]:   {:?}\n",  &v1[4..7]);
    v1.pop();

    let v2 = v1;
    print!("v2[4..7]:   {:?}\n",  &v2[4..7]);
    // print!("v1[4..7]:   {:?}\n",  &v1[4..7]);        //  👎ς FAIL! value borrowed after move. More Info: `∞ rustc --explain E0382`
    print!("done!\n\n");

    print!("{}📚 Testing folding adapter \n\n", C_LL);
    print!("v2:             {:?}\n",  &v2);
    print!("sum_vector(v2): {}\n", sum_vector(&v2));
    print!("done!\n\n");

    Ok(())
    // panic!("for: No Reason");
}

