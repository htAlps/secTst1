// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b5-functions-closures.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.01]

fn say_hello(to: &str) {
    print!("hello {}\n", to);
}

fn add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}




///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b5-functions-closures.rs \n\n", C_LL);

    print!("📚 fn say_hello \n");

    say_hello("Samael");
    print!("done!\n\n");

    print!("📚 fn add \n");
    print!("add(3, 4) = {} \n", add(3, 4));
    print!("done!\n\n");

    print!("📚 bind fn to a var - let sum = add \n");
    let sum = add;
    print!("sum(5, 7) = {} \n", sum(5, 7));
    print!("done!\n\n");

    print!("{}📚 Testing closures \n\n", C_LL);

    let closure_add = |ii: i32, jj: i32| ii + jj;
    print!("closure_add(5, 7) = {}\n", closure_add(5, 7));

    let closure_incr = |ii: i32| ii + 1;
    print!("closure_incr(5) = {}\n", closure_incr(5));

    let closure_counter1 = |seed: i32| {
        let mut index: i32 = seed;
        let counter = move || {
            index += 1;
            index
        };
        counter
    };

    let mut counter1 = closure_counter1(12);
    print!("counter1 - tic: {}, tic: {}, tic: {}\n", counter1(), counter1(), counter1()); 
    print!("done!\n\n");

    Ok(())
    // panic!("for: No Reason");
}

