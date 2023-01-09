// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b4-arrays-vectors-tuples.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.01]


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b4-arrays.rs \n\n", C_LL);

    print!("📚 Testing array (inmutable) \n\n");

    let aa = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    print!("aa[*]:      {:?}\n",  &aa);
    print!("aa.len():   {}\n",    aa.len());
    print!("aa[2]:      {}\n",    &aa[2]);
    print!("aa[4..7]:   {:?}\n",  &aa[4..7]);
    print!("done!\n");

    print!("{}📚 Testing vectors \n\n", C_LL);
    let mut vv = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];     // 89. 144, ...
    print!("vv[4..7]:   {:?}\n",  &vv[4..7]);

    print!("\nranging on vector vv: ");
    for elem in &vv {
        print!("{}, ", elem);
    }
    print!("\ndone!\n");

    print!("{}📚 vector pop § push \n\n", C_LL);
    print!("vv.pop():   {:?}\n", vv.pop());
    print!("vv.pop():   {:?}\n", vv.pop());
    
    let elem = vv.pop();
    print!("elem:       {:?}\n", elem);
    print!("done!\n\n");

    print!("📚 vector push 100, 101 \n");
    vv.push(100);
    vv.push(101);
    print!("vv[*]:      {:?}\n",  &vv);
    print!("done!\n\n");

    print!("{}📚 Testing tuples \n\n", C_LL);
    let tt: (&str, i32) = ("aaa", 111);
    let tt2             = ("bbb", 222);
    let tt3             = ("ccc", 333, "normal");
    print!("tt.0: {}, tt.1: {} \n", tt.0, tt.1);
    print!("tt2-pretty:\n{:#?}\n", tt2);
    print!("tt3-tuple:  {:?} \n", tt3);
    print!("done!\n\n");

    Ok(())
    // panic!("for: No Reason");
}

