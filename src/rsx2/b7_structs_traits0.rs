// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// ✨λ b7-structs-traits-enums.rs [ιδ21.12.22 τ08:48:42] 🌐η [δ22.01.02] [δ22.01.01]

// use std::io::stdin;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// structs - a circle struct 
struct Circle {
    xx: f64,
    yy: f64,
    rr: f64
}

// a rectangle struct 
struct Rect {
    ll: f64,
    ww: f64
}

// the bad way: a fn to get the radius 
fn get_radius(cc: &Circle) -> f64 {
    cc.rr
}

impl Circle {
    pub fn get_xx(&self) -> f64 { self.xx }     // ugly way 
    pub fn get_yy(&self) -> f64 { self.yy }     // ugly way 
    pub fn get_rr(&self) -> f64 { self.rr }     // ugly way 
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// traits
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 { 3.14159 * self.rr * self.rr }
}

impl HasArea for Rect {
    fn area(&self) -> f64 { self.ll * self.ww }
}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•
// enumerations 
enum Hero {
    Fast,
    Strong(i8),
    Info {name: String, secret: String}
}

fn get_info(hh: Hero) {
    match hh {
        Hero::Fast                   => print!("fast hero\n"), 
        Hero::Strong(ii)             => print!("strong hero, {} rating \n", ii), 
        Hero::Info {name, secret}    => print!("{}'s is {}\n", name, secret),
    }
}


// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════•


///λ mod_main is the module suppervisor (a sub-main program); it can also serve as integration tester
pub fn mod_main() -> Result<(), Box<dyn Error>> {

    let C_LL = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";
    print!("{}📚 STARTING: b7-structs-traits.rs  \n\n", C_LL);

    print!("📚 Testing structs - the bad way\n\n");
    let c1 = Circle { xx: 3., yy: 4., rr: 5. };

    print!("get_radius(c1): {}\n", get_radius(&c1));
    print!("done!\n\n");

    print!("{}📚 Testing structs - the good way\n\n", C_LL);
    print!("c1.get_xx():     {}\n", c1.get_xx());
    print!("c1.get_yy():     {}\n", c1.get_yy());
    print!("c1.get_rr():     {}\n", c1.get_rr());
    print!("done!\n\n");

    print!("{}📚 Testing structs - the ugly wayY!!\n\n", C_LL);
    print!("c1.xx:          {}\n", c1.xx);
    print!("c1.yy:          {}\n", c1.yy);
    print!("c1.rr:          {}\n", c1.rr);
    print!("done!\n\n");

    print!("{}📚 Testing traits (interfaces) \n\n", C_LL);
    let r1 = Rect { ll: 5., ww: 7. };
    print!("c1.area():      {}\n", c1.area());
    print!("r1.area():      {}\n", r1.area());
    print!("done!\n\n");

    print!("{}📚 Testing enumerations \n\n", C_LL);
    let hulk    = Hero::Strong(100);
    let flash   = Hero::Fast;
    let batman  = Hero::Info {name: "Bruce Wayne".to_owned(), secret: "111".to_owned()};

    get_info(hulk);
    get_info(flash);
    get_info(batman);
    print!("done!\n\n");

    Ok(())
    // panic!("for: No Reason");
}

