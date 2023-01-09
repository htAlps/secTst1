//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ main  ι✧21․12․25✦16․50․24․  🌎η ✧22․09․17․ ✧22․08․07․ ✧22․08․05․ ✧22․07․04․ ✧22․06․22․
mod a32_cust_tp_enum_a1;

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`

//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•


/// check_mod is this system's exec fn; it selects a module from list below and runs it's suppervisor fn
pub fn check_mod() -> Result<(), String> {
    print!("🎡𐡋 rsx0::mod.rs  \n");

    let my_location = "rsx1::mod.rs"; 
    match a32_cust_tp_enum_a1::check_mod() {
        Err(ee) => Err(format!("{ee}@{my_location}.")),
        _       => Ok(()),
    }
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Recent Crates
mod a32_cust_tp_enum_a1;                            use a32_cust_tp_enum_a1::{check_mod};                             
mod a31_cust_tp_structs_a1;                         use a31_cust_tp_structs_a1::{check_mod};                          
mod e3_gen_gen_trait_a1;                            use e3_gen_gen_trait_a1::{check_mod};                             
mod g5_trait_iter_a1;                               use g5_trait_iter_a1::{check_mod};                                
mod g5_trait_iter_a2;                               use g5_trait_iter_a2::{check_mod};                                
                                                    
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
OLD STYLE Crate Defs  

??? Check

  • T = TDD,  U = DATA,   
  • mod u__ PREFIX IS OPEN for DATA § DBs
  • V (ω) = ALL KATA Excersizes  


pub fn run_checks() -> Result<(), Box<dyn Error>> {

    match check_mod() {
        Err(ee) => panic!("{}", ee),
        _       => Ok(()),
    }
}

mod a10_hello;                                      use a10_hello::{check_mod};                                       
mod a11_hi_comment;                                 use a11_hi_comment::{check_mod};                                  
mod a12_hi_print;                                   use a12_hi_print::{check_mod};                                    
mod a20_primitives;                                 use a20_primitives::{check_mod};                                  
mod a21_prm_literals;                               use a21_prm_literals::{check_mod};                                
mod a22_prm_tuples;                                 use a22_prm_tuples::{check_mod};                                  
mod a23_prm_array;                                  use a23_prm_array::{check_mod};                                   
mod a30_custom_types;                               use a30_custom_types::{check_mod};                                
mod a31_cust_tp_structs;                            use a31_cust_tp_structs::{check_mod};                             
mod a31_cust_tp_structs_a1;                         use a31_cust_tp_structs_a1::{check_mod};                          
mod a32_cust_tp_enum;                               use a32_cust_tp_enum::{check_mod};                                
mod a32_cust_tp_enum_a1;                            use a32_cust_tp_enum_a1::{check_mod};                             
mod a33_cust_tp_enum_use;                           use a33_cust_tp_enum_use::{check_mod};                            
mod a34_cust_tp_enum_c_like;                        use a34_cust_tp_enum_c_like::{check_mod};                         
mod a35_cust_tp_enum_testc_linked_list;             use a35_cust_tp_enum_testc_linked_list::{check_mod};              
mod a36_cust_tp_constants;                          use a36_cust_tp_constants::{check_mod};                           
mod a4a_var_bindings;                               use a4a_var_bindings::{check_mod};                                
mod a4b_var_binds_mut;                              use a4b_var_binds_mut::{check_mod};                               
mod a4c_var_binds_scope;                            use a4c_var_binds_scope::{check_mod};                             
mod a4d_var_binds_declare;                          use a4d_var_binds_declare::{check_mod};                           
mod a50_types;                                      use a50_types::{check_mod};                                       
mod a51_types_cast;                                 use a51_types_cast::{check_mod};                                  
mod a52_types_literals;                             use a52_types_literals::{check_mod};                              
mod a53_types_inference;                            use a53_types_inference::{check_mod};                             
mod a54_types_alias;                                use a54_types_alias::{check_mod};                                 
mod a60_conversion;                                 use a60_conversion::{check_mod};                                  
mod a61_conv_from_into;                             use a61_conv_from_into::{check_mod};                              
mod a62_conv_try_from_try_into;                     use a62_conv_try_from_try_into::{check_mod};                      
mod a63_conv_string;                                use a63_conv_string::{check_mod};                                 
mod a60_expression;                                 use a60_expression::{check_mod};                                  
mod a70_flow_control;                               use a70_flow_control::{check_mod};                                
mod a81_fl_ctrl_if_else;                            use a81_fl_ctrl_if_else::{check_mod};                             
mod a82_fl_ctrl_loop;                               use a82_fl_ctrl_loop::{check_mod};                                
mod a83_fl_ctrl_loop_nested;                        use a83_fl_ctrl_loop_nested::{check_mod};                         
mod a84_fl_ctrl_loop_return;                        use a84_fl_ctrl_loop_return::{check_mod};                         
mod a85_fl_ctrl_while;                              use a85_fl_ctrl_while::{check_mod};                               
mod a86_fl_ctrl_for;                                use a86_fl_ctrl_for::{check_mod};                                 
mod a87_fl_ctrl_match;                              use a87_fl_ctrl_match::{check_mod};                               
mod a88_fl_ctrl_match_destructuring;                use a88_fl_ctrl_match_destructuring::{check_mod};                 
mod a89_fl_ctrl_match_destr_tuple;                  use a89_fl_ctrl_match_destr_tuple::{check_mod};                   
mod a8a_fl_ctrl_match_destr_slice;                  use a8a_fl_ctrl_match_destr_slice::{check_mod};                   
mod a8b_fl_ctrl_match_destr_enum;                   use a8b_fl_ctrl_match_destr_enum::{check_mod};                    
mod a8c_fl_ctrl_match_destr_pointers;               use a8c_fl_ctrl_match_destr_pointers::{check_mod};                
mod a8d_fl_ctrl_match_destr_structures;             use a8d_fl_ctrl_match_destr_structures::{check_mod};              
mod a8e_fl_ctrl_match_guard;                        use a8e_fl_ctrl_match_guard::{check_mod};                         
mod a8f_fl_ctrl_match_binding;                      use a8f_fl_ctrl_match_binding::{check_mod};                       
mod a8g_fl_ctrl_if_let;                             use a8g_fl_ctrl_if_let::{check_mod};                              
mod a8h_fl_ctrl_while_let;                          use a8h_fl_ctrl_while_let::{check_mod};                           
mod a90_fn;                                         use a90_fn::{check_mod};                                          
mod a91_fn_methods;                                 use a91_fn_methods::{check_mod};                                  
mod a92_fn_closures;                                use a92_fn_closures::{check_mod};                                 
mod a93_fn_closure_capture;                         use a93_fn_closure_capture::{check_mod};                          
mod a94_fn_closure_input_pars;                      use a94_fn_closure_input_pars::{check_mod};                       
mod a95_fn_closure_anonymity;                       use a95_fn_closure_anonymity::{check_mod};                        
mod a96_fn_closure_input_funcs;                     use a96_fn_closure_input_funcs::{check_mod};                      
mod a97_fn_closure_output_pars;                     use a97_fn_closure_output_pars::{check_mod};                      
mod a98_fn_closure_ex;                              use a98_fn_closure_ex::{check_mod};                               
mod a99_fn_closure_ex_iter_any;                     use a99_fn_closure_ex_iter_any::{check_mod};                      
mod a9a_fn_closure_ex_iter_find;                    use a9a_fn_closure_ex_iter_find::{check_mod};                     
mod a9b_fn_hof;                                     use a9b_fn_hof::{check_mod};                                      
mod a9c_fn_diverging;                               use a9c_fn_diverging::{check_mod};                                
mod aA0_modules;                                    use aA0_modules::{check_mod};                                     
mod aA1_mod_visibility;                             use aA1_mod_visibility::{check_mod};                              
mod aA2_mod_struct_visibility;                      use aA2_mod_struct_visibility::{check_mod};                       
mod aA3_mod_use;                                    use aA3_mod_use::{check_mod};                                     
mod aA4_mod_super;                                  use aA4_mod_super::{check_mod};                                   
mod aA5_mod_split;                                  use aA5_mod_split::{check_mod};                                   
mod b0_crates;                                      use b0_crates::{check_mod};                                       
mod b1_crates_lib;                                  use b1_crates_lib::{check_mod};                                   
mod b2_crates_using_lib;                            use b2_crates_using_lib::{check_mod};                             
mod c0_cargo;                                       use c0_cargo::{check_mod};                                        
mod c1_cargo_deps;                                  use c1_cargo_deps::{check_mod};                                   
mod c2_cargo_conventions;                           use c2_cargo_conventions::{check_mod};                            
mod c3_cargo_test;                                  use c3_cargo_test::{check_mod};                                   
mod c4_cargo_build_scripts;                         use c4_cargo_build_scripts::{check_mod};                          
mod d0_attribute;                                   use d0_attribute::{check_mod};                                    
mod d1_attrib_unused;                               use d1_attrib_unused::{check_mod};                                
mod d2_attrib_crate;                                use d2_attrib_{check_mod};                                        
mod d3_attrib_cfg;                                  use d3_attrib_cfg::{check_mod};                                   
mod d4_attrib_cfg_custom;                           use d4_attrib_cfg_custom::{check_mod};                            
mod e0_generics;                                    use e0_generics::{check_mod};                                     
mod e1_gen_gen_fn;                                  use e1_gen_gen_fn::{check_mod};                                   
mod e2_gen_impl;                                    use e2_gen_impl::{check_mod};                                     
mod e3_gen_gen_trait;                               use e3_gen_gen_trait::{check_mod};                                
mod e3_gen_gen_trait_a1;                            use e3_gen_gen_trait_a1::{check_mod};                             
mod e3_gen_gen_trait_a2;                            use e3_gen_gen_trait_a2::{check_mod};                             
mod e4_gen_bounds;                                  use e4_gen_bounds::{check_mod};                                   
mod e5_gen_bounds_testc_empty;                      use e5_gen_bounds_testc_empty::{check_mod};                       
mod e6_gen_multi_bounds;                            use e6_gen_multi_bounds::{check_mod};                             
mod e7_gen_where;                                   use e7_gen_where::{check_mod};                                    
mod e8_gen_new_types;                               use e8_gen_new_types::{check_mod};                                
mod e9_gen_assoc_items;                             use e9_gen_assoc_items::{check_mod};                              
mod ea_gen_assoc_items_the_problem;                 use ea_gen_assoc_items_the_problem::{check_mod};                  
mod eb_gen_assoc_items_types;                       use eb_gen_assoc_items_types::{check_mod};                        
mod ec_gen_phantom;                                 use ec_gen_phantom::{check_mod};                                  
mod ed_gen_phantom_testc_units;                     use ed_gen_phantom_testc_units::{check_mod};                      
mod f0_scope;                                       use f0_scope::{check_mod};                                        
mod f1_scope_raii;                                  use f1_scope_raii::{check_mod};                                   
mod f2_scope_move;                                  use f2_scope_move::{check_mod};                                   
mod f3_scope_move_mut;                              use f3_scope_move_mut::{check_mod};                               
mod f4_scope_move_partial_move;                     use f4_scope_move_partial_move::{check_mod};                      
mod f5_scope_borrow;                                use f5_scope_borrow::{check_mod};                                 
mod f6_scope_borrow_mut;                            use f6_scope_borrow_mut::{check_mod};                             
mod f7_scope_borrow_alias;                          use f7_scope_borrow_alias::{check_mod};                           
mod f8_scope_borrow_ref;                            use f8_scope_borrow_ref::{check_mod};                             
mod f9_scope_lifetime;                              use f9_scope_lifetime::{check_mod};                               
mod fa_scope_lifetime_explicit;                     use fa_scope_lifetime_explicit::{check_mod};                      
mod fb_scope_lifetime_fn;                           use fb_scope_lifetime_fn::{check_mod};                            
mod fc_scope_lifetime_methods;                      use fc_scope_lifetime_methods::{check_mod};                       
mod fd_scope_lifetime_struct;                       use fd_scope_lifetime_struct::{check_mod};                        
mod fe_scope_lifetime_trait;                        use fe_scope_lifetime_trait::{check_mod};                         
mod ff_scope_lifetime_lifetime_bounds;              use ff_scope_lifetime_lifetime_bounds::{check_mod};               
mod fg_scope_lifetime_lifetime_coercion;            use fg_scope_lifetime_lifetime_coercion::{check_mod};             
mod fh_scope_lifetime_static_lifetime;              use fh_scope_lifetime_static_lifetime::{check_mod};               
mod fi_scope_lifetime_elision;                      use fi_scope_lifetime_elision::{check_mod};                       
mod g0_trait;                                       use g0_trait::{check_mod};                                        
mod g1_trait_derive;                                use g1_trait_derive::{check_mod};                                 
mod g2_trait_dyn;                                   use g2_trait_dyn::{check_mod};                                    
mod g3_trait_ops;                                   use g3_trait_ops::{check_mod};                                    
mod g4_trait_drop;                                  use g4_trait_drop::{check_mod};                                   
mod g5_trait_iter;                                  use g5_trait_iter::{check_mod};                                   
mod g5_trait_iter_a1;                               use g5_trait_iter_a1::{check_mod};                                
mod g5_trait_iter_a2;                               use g5_trait_iter_a2::{check_mod};                                
mod g5_trait_iter_a3;                               use g5_trait_iter_a3::{check_mod};                                
mod g6_trait_impl_trait;                            use g6_trait_impl_trait::{check_mod};                             
mod g7_trait_clone;                                 use g7_trait_clone::{check_mod};                                  
mod g8_trait_supertraits;                           use g8_trait_supertraits::{check_mod};                            
mod g9_trait_disambiguating;                        use g9_trait_disambiguating::{check_mod};                         
mod h0_macros;                                      use h0_macros::{check_mod};                                       
mod h1_macros_syntax;                               use h1_macros_syntax::{check_mod};                                
mod h2_macros_designators;                          use h2_macros_designators::{check_mod};                           
mod h3_macros_overload;                             use h3_macros_overload::{check_mod};                              
mod h4_macros_repeat;                               use h4_macros_repeat::{check_mod};                                
mod h5_macros_dry;                                  use h5_macros_dry::{check_mod};                                   
mod h6_macros_dsl;                                  use h6_macros_dsl::{check_mod};                                   
mod h7_macros_variadics;                            use h7_macros_variadics::{check_mod};                             
mod i0_error;                                       use i0_error::{check_mod};                                        
mod i1_err_panic;                                   use i1_err_panic::{check_mod};                                    
mod i2_err_abort_unwind;                            use i2_err_abort_unwind::{check_mod};                             
mod i3_err_option_unwrap;                           use i3_err_option_unwrap::{check_mod};                            
mod i4_err_option_unwrap_q_mark;                    use i4_err_option_unwrap_q_mark::{check_mod};                     
mod i5_err_option_unwrap_map;                       use i5_err_option_unwrap_map::{check_mod};                        
mod i6_err_option_unwrap_and_then;                  use i6_err_option_unwrap_and_then::{check_mod};                   
mod i7_err_result;                                  use i7_err_result::{check_mod};                                   
mod i8_err_res_map;                                 use i8_err_res_map::{check_mod};                                  
mod i9_err_res_alias;                               use i9_err_res_alias::{check_mod};                                
mod ia_err_res_early_returns;                       use ia_err_res_early_returns::{check_mod};                        
mod ib_err_res_enter_q_mark;                        use ib_err_res_enter_q_mark::{check_mod};                         
mod ic_err_multiple_err_types;                      use ic_err_multiple_err_types::{check_mod};                       
mod id_err_mult_err_types_option_result;            use id_err_mult_err_types_option_result::{check_mod};             
mod ie_err_mult_err_types_define_err_type;          use ie_err_mult_err_types_define_err_type::{check_mod};           
mod if_err_mult_err_types_boxing_errors;            use if_err_mult_err_types_boxing_errors::{check_mod};             
mod ig_err_mult_err_types_reenter_q_mark;           use ig_err_mult_err_types_reenter_q_mark::{check_mod};            
mod ih_err_mult_err_types_wrap_error;               use ih_err_mult_err_types_wrap_error::{check_mod};                
mod ii_err_iter_result;                             use ii_err_iter_result::{check_mod};                              
mod j0_std;                                         use j0_std::{check_mod};                                          
mod j1_std_box;                                     use j1_std_box::{check_mod};                                      
mod j2_std_vec;                                     use j2_std_vec::{check_mod};                                      
mod j3_std_str;                                     use j3_std_str::{check_mod};                                      
mod j4_std_option;                                  use j4_std_option::{check_mod};                                   
mod j5_std_result;                                  use j5_std_result::{check_mod};                                   
mod j6_std_res_q_mark;                              use j6_std_res_q_mark::{check_mod};                               
mod j7_std_panic;                                   use j7_std_panic::{check_mod};                                    
mod j8_std_hash;                                    use j8_std_hash::{check_mod};                                     
mod j9_std_hash_alt_key_types;                      use j9_std_hash_alt_key_types::{check_mod};                       
mod ja_std_hash_hashset;                            use ja_std_hash_hashset::{check_mod};                             
mod jb_std_rc;                                      use jb_std_rc::{check_mod};                                       
mod jc_std_arc;                                     use jc_std_arc::{check_mod};                                      
mod k0_std_misc;                                    use k0_std_misc::{check_mod};                                     
mod k1_std_misc_threads;                            use k1_std_misc_threads::{check_mod};                             
mod k2_std_misc_threads_testc_mapreduce;            use k2_std_misc_threads_testc_mapreduce::{check_mod};             
mod k3_std_misc_channels;                           use k3_std_misc_channels::{check_mod};                            
mod k4_std_misc_path;                               use k4_std_misc_path::{check_mod};                                
mod k5_std_misc_file;                               use k5_std_misc_file::{check_mod};                                
mod k6_std_misc_file_open;                          use k6_std_misc_file_open::{check_mod};                           
mod k7_std_misc_file_create;                        use k7_std_misc_file_create::{check_mod};                         
mod k8_std_misc_file_read_lines;                    use k8_std_misc_file_read_lines::{check_mod};                     
mod k9_std_misc_process;                            use k9_std_misc_process::{check_mod};                             
mod ka_std_misc_process_pipe;                       use ka_std_misc_process_pipe::{check_mod};                        
mod kb_std_misc_process_wait;                       use kb_std_misc_process_wait::{check_mod};                        
mod kc_std_misc_fs;                                 use kc_std_misc_fs::{check_mod};                                  
mod kd_std_misc_arg;                                use kd_std_misc_arg::{check_mod};                                 
mod ke_std_misc_arg_matching;                       use ke_std_misc_arg_matching::{check_mod};                        
mod kf_std_misc_ffi;                                use kf_std_misc_ffi::{check_mod};                                 
mod l0_unsafe;                                      use l0_unsafe::{check_mod};                                       
mod l1_unsafe_asm;                                  use l1_unsafe_asm::{check_mod};                                   
mod m0_compatibility;                               use m0_compatibility::{check_mod};                                
mod m1_comp_raw_identifiers;                        use m1_comp_raw_identifiers::{check_mod};                         
mod n0_meta;                                        use n0_meta::{check_mod};                                         
mod n1_meta_doc;                                    use n1_meta_doc::{check_mod};                                     
mod n2_meta_playpen;                                use n2_meta_playpen::{check_mod};                                 
                                                                                                                     
                                                    T = TDD                                                          
mod t0_testing;                                     use t0_testing::{check_mod};                                      
mod t1_test_unit_testing;                           use t1_test_unit_testing::{check_mod};                            
mod t2_test_doc_testing;                            use t2_test_doc_testing::{check_mod};                             
mod t3_test_integration_testing;                    use t3_test_integration_testing::{check_mod};                     
mod t4_test_dev_dependencies;                       use t4_test_dev_dependencies::{check_mod};                        
U = DATA 

*/
// End Of The Code Pit

//λ mod.rs
// ✨λ mod.rs
