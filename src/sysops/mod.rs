// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ sysops::mod  ι✧21․12․25✦16․50․24․  🌎η ✧22․12․29․✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․
pub mod s2_hash;
pub mod s3_regex;
pub mod s4_metrics;
use lib3::q0_env::{EvType, log_event};

/// run(), runs the system 
pub fn run() -> Result<(), String> {

    let my_location = "sysops::mod::run";
    log_event(EvType::Trace, "", my_location, true);
    match lib3::q0_env::get_cmd_code() {
        None => Ok(()),
        Some(cmd_code) => {
            print!("cmd_code: {cmd_code} \n");
            match cmd_code.as_str() {
                "add-hs"  => {
                    match s2_hash::add_sets() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "sub-hs"  => {
                    match s2_hash::sub_sets() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "int-hs"  => {
                    match s2_hash::int_sets() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "add-hm"  => {
                    match s2_hash::add_maps() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "sub-hm"  => {
                    match s2_hash::sub_maps() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "int-hm"  => {
                    match s2_hash::int_maps() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "clean"  => {
                    match s3_regex::clean_csv() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "g0fmap"  => {
                    match s4_metrics::gen_fold_metrics_v0() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "g1fmap"  => {
                    match s4_metrics::gen_fold_metrics_v1() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "g0fnode"  => {
                    match s4_metrics::gen_node_metrics_v0() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "cp0node"  => {
                    match s4_metrics::check_print_fnode_v0() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "check"  => {
                    match check() {
                        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                        _ => Ok(()),
                    }
                },
                "help"  => {
                    Ok(())
                },
                _ => {
                    Err(format!("⟸ something went wrong@{my_location}"))
                }
            }
        }
    }
}


/// check runs each module integration-tests
pub fn check() -> Result<(), String> {

    let my_location = "sysops::check";
    log_event(EvType::Trace, "", my_location, true);

    match s2_hash::check() {
        Err(ee) => Err(format!("Trace: {ee}⟸ {my_location}")),
        _ => match s4_metrics::check() {
            Err(ee) => Err(format!("Trace: {ee}⟸ {my_location}")),
            _ => Ok(())
        }
    }
}


//λ The Code Pit
/*
   gen_fold_metrics
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() gets the function-code to be executed and runs the corresponding fn
pub fn run() -> Result<(), String> {

    
    log_event(EvType::Trace, "", "sysops::run", true);
    let cmd_code = get_cmd_code();

    let my_location = "s1_exec::run";
    match cmd_code {
        "2add-hs"  => {                // y2hs.csv <- x2hs1.csv + x2hs2.csv
            match s2_hash::add_hashsets() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }

        ...

}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// check int-tests the active system as a whole
pub fn check() -> Result<(), String> {

    
    match s1_metrics::check() {                                     // checking metrics calculations
        Err(ee) => Err(format!("{ee}⟸ {"sysops::check"}")),
        _ => Ok(()),
    }
}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    match s1_metrics::run() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
λ Crates § Modules

mod sysops_exec;
mod a84_re_multiline;               use a84_re_multiline::{check};
mod a83_regex_basics;               use a83_regex_basics::{check};
mod a82_string_methods;             use a82_string_methods::{check};


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
mod sysops_exec;                    use sysops_exec::{check};
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit

