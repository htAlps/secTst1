//  •═══════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//  ✨λ main  ι✧21․12․25✦16․50․24․  🌎η ✧22․09․17․ ✧22․08․07․ ✧22․08․05․ ✧22․07․04․ ✧22․06․22․

use std::error::Error;
use crate::a32_cust_tp_enum_a1::{mod_main};                         mod a32_cust_tp_enum_a1;


fn main() -> Result<(), Box<dyn Error>> {

    match mod_main() {
        Err(e)  => panic!("{}", e),
        _       => Ok(()),
    }
}

//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
Recent Crates
use crate::a32_cust_tp_enum_a1::{mod_main};                             mod a32_cust_tp_enum_a1;
use crate::a31_cust_tp_structs_a1::{mod_main};                          mod a31_cust_tp_structs_a1;
use crate::e3_gen_gen_trait_a1::{mod_main};                             mod e3_gen_gen_trait_a1;
use crate::g5_trait_iter_a1::{mod_main};                                mod g5_trait_iter_a1;
use crate::g5_trait_iter_a2::{mod_main};                                mod g5_trait_iter_a2;

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
All Crates In Alphabetical Order

T = TDD,  U = DATA,   
mod u__ PREFIX IS OPEN for DATA § DBs
V (ω) = ALL KATA Excersizes  

use crate::a10_hello::{mod_main};                                       mod a10_hello;
use crate::a11_hi_comment::{mod_main};                                  mod a11_hi_comment;
use crate::a12_hi_print::{mod_main};                                    mod a12_hi_print;
use crate::a20_primitives::{mod_main};                                  mod a20_primitives;
use crate::a21_prm_literals::{mod_main};                                mod a21_prm_literals;
use crate::a22_prm_tuples::{mod_main};                                  mod a22_prm_tuples;
use crate::a23_prm_array::{mod_main};                                   mod a23_prm_array;
use crate::a30_custom_types::{mod_main};                                mod a30_custom_types;
use crate::a31_cust_tp_structs::{mod_main};                             mod a31_cust_tp_structs;
use crate::a31_cust_tp_structs_a1::{mod_main};                          mod a31_cust_tp_structs_a1;
use crate::a32_cust_tp_enum::{mod_main};                                mod a32_cust_tp_enum;
use crate::a32_cust_tp_enum_a1::{mod_main};                             mod a32_cust_tp_enum_a1;
use crate::a33_cust_tp_enum_use::{mod_main};                            mod a33_cust_tp_enum_use;
use crate::a34_cust_tp_enum_c_like::{mod_main};                         mod a34_cust_tp_enum_c_like;
use crate::a35_cust_tp_enum_testc_linked_list::{mod_main};              mod a35_cust_tp_enum_testc_linked_list;
use crate::a36_cust_tp_constants::{mod_main};                           mod a36_cust_tp_constants;
use crate::a4a_var_bindings::{mod_main};                                mod a4a_var_bindings;
use crate::a4b_var_binds_mut::{mod_main};                               mod a4b_var_binds_mut;
use crate::a4c_var_binds_scope::{mod_main};                             mod a4c_var_binds_scope;
use crate::a4d_var_binds_declare::{mod_main};                           mod a4d_var_binds_declare;
use crate::a50_types::{mod_main};                                       mod a50_types;
use crate::a51_types_cast::{mod_main};                                  mod a51_types_cast;
use crate::a52_types_literals::{mod_main};                              mod a52_types_literals;
use crate::a53_types_inference::{mod_main};                             mod a53_types_inference;
use crate::a54_types_alias::{mod_main};                                 mod a54_types_alias;
use crate::a60_conversion::{mod_main};                                  mod a60_conversion;
use crate::a61_conv_from_into::{mod_main};                              mod a61_conv_from_into;
use crate::a62_conv_try_from_try_into::{mod_main};                      mod a62_conv_try_from_try_into;
use crate::a63_conv_string::{mod_main};                                 mod a63_conv_string;
use crate::a60_expression::{mod_main};                                  mod a60_expression;
use crate::a70_flow_control::{mod_main};                                mod a70_flow_control;
use crate::a81_fl_ctrl_if_else::{mod_main};                             mod a81_fl_ctrl_if_else;
use crate::a82_fl_ctrl_loop::{mod_main};                                mod a82_fl_ctrl_loop;
use crate::a83_fl_ctrl_loop_nested::{mod_main};                         mod a83_fl_ctrl_loop_nested;
use crate::a84_fl_ctrl_loop_return::{mod_main};                         mod a84_fl_ctrl_loop_return;
use crate::a85_fl_ctrl_while::{mod_main};                               mod a85_fl_ctrl_while;
use crate::a86_fl_ctrl_for::{mod_main};                                 mod a86_fl_ctrl_for;
use crate::a87_fl_ctrl_match::{mod_main};                               mod a87_fl_ctrl_match;
use crate::a88_fl_ctrl_match_destructuring::{mod_main};                 mod a88_fl_ctrl_match_destructuring;
use crate::a89_fl_ctrl_match_destr_tuple::{mod_main};                   mod a89_fl_ctrl_match_destr_tuple;
use crate::a8a_fl_ctrl_match_destr_slice::{mod_main};                   mod a8a_fl_ctrl_match_destr_slice;
use crate::a8b_fl_ctrl_match_destr_enum::{mod_main};                    mod a8b_fl_ctrl_match_destr_enum;
use crate::a8c_fl_ctrl_match_destr_pointers::{mod_main};                mod a8c_fl_ctrl_match_destr_pointers;
use crate::a8d_fl_ctrl_match_destr_structures::{mod_main};              mod a8d_fl_ctrl_match_destr_structures;
use crate::a8e_fl_ctrl_match_guard::{mod_main};                         mod a8e_fl_ctrl_match_guard;
use crate::a8f_fl_ctrl_match_binding::{mod_main};                       mod a8f_fl_ctrl_match_binding;
use crate::a8g_fl_ctrl_if_let::{mod_main};                              mod a8g_fl_ctrl_if_let;
use crate::a8h_fl_ctrl_while_let::{mod_main};                           mod a8h_fl_ctrl_while_let;
use crate::a90_fn::{mod_main};                                          mod a90_fn;
use crate::a91_fn_methods::{mod_main};                                  mod a91_fn_methods;
use crate::a92_fn_closures::{mod_main};                                 mod a92_fn_closures;
use crate::a93_fn_closure_capture::{mod_main};                          mod a93_fn_closure_capture;
use crate::a94_fn_closure_input_pars::{mod_main};                       mod a94_fn_closure_input_pars;
use crate::a95_fn_closure_anonymity::{mod_main};                        mod a95_fn_closure_anonymity;
use crate::a96_fn_closure_input_funcs::{mod_main};                      mod a96_fn_closure_input_funcs;
use crate::a97_fn_closure_output_pars::{mod_main};                      mod a97_fn_closure_output_pars;
use crate::a98_fn_closure_ex::{mod_main};                               mod a98_fn_closure_ex;
use crate::a99_fn_closure_ex_iter_any::{mod_main};                      mod a99_fn_closure_ex_iter_any;
use crate::a9a_fn_closure_ex_iter_find::{mod_main};                     mod a9a_fn_closure_ex_iter_find;
use crate::a9b_fn_hof::{mod_main};                                      mod a9b_fn_hof;
use crate::a9c_fn_diverging::{mod_main};                                mod a9c_fn_diverging;
use crate::aA0_modules::{mod_main};                                     mod aA0_modules;
use crate::aA1_mod_visibility::{mod_main};                              mod aA1_mod_visibility;
use crate::aA2_mod_struct_visibility::{mod_main};                       mod aA2_mod_struct_visibility;
use crate::aA3_mod_use::{mod_main};                                     mod aA3_mod_use;
use crate::aA4_mod_super::{mod_main};                                   mod aA4_mod_super;
use crate::aA5_mod_split::{mod_main};                                   mod aA5_mod_split;
use crate::b0_crates::{mod_main};                                       mod b0_crates;
use crate::b1_crates_lib::{mod_main};                                   mod b1_crates_lib;
use crate::b2_crates_using_lib::{mod_main};                             mod b2_crates_using_lib;
use crate::c0_cargo::{mod_main};                                        mod c0_cargo;
use crate::c1_cargo_deps::{mod_main};                                   mod c1_cargo_deps;
use crate::c2_cargo_conventions::{mod_main};                            mod c2_cargo_conventions;
use crate::c3_cargo_test::{mod_main};                                   mod c3_cargo_test;
use crate::c4_cargo_build_scripts::{mod_main};                          mod c4_cargo_build_scripts;
use crate::d0_attribute::{mod_main};                                    mod d0_attribute;
use crate::d1_attrib_unused::{mod_main};                                mod d1_attrib_unused;
use crate::d2_attrib_crate::{mod_main};                                 mod d2_attrib_crate;
use crate::d3_attrib_cfg::{mod_main};                                   mod d3_attrib_cfg;
use crate::d4_attrib_cfg_custom::{mod_main};                            mod d4_attrib_cfg_custom;
use crate::e0_generics::{mod_main};                                     mod e0_generics;
use crate::e1_gen_gen_fn::{mod_main};                                   mod e1_gen_gen_fn;
use crate::e2_gen_impl::{mod_main};                                     mod e2_gen_impl;
use crate::e3_gen_gen_trait::{mod_main};                                mod e3_gen_gen_trait;
use crate::e3_gen_gen_trait_a1::{mod_main};                             mod e3_gen_gen_trait_a1;
use crate::e3_gen_gen_trait_a2::{mod_main};                             mod e3_gen_gen_trait_a2;
use crate::e4_gen_bounds::{mod_main};                                   mod e4_gen_bounds;
use crate::e5_gen_bounds_testc_empty::{mod_main};                       mod e5_gen_bounds_testc_empty;
use crate::e6_gen_multi_bounds::{mod_main};                             mod e6_gen_multi_bounds;
use crate::e7_gen_where::{mod_main};                                    mod e7_gen_where;
use crate::e8_gen_new_types::{mod_main};                                mod e8_gen_new_types;
use crate::e9_gen_assoc_items::{mod_main};                              mod e9_gen_assoc_items;
use crate::ea_gen_assoc_items_the_problem::{mod_main};                  mod ea_gen_assoc_items_the_problem;
use crate::eb_gen_assoc_items_types::{mod_main};                        mod eb_gen_assoc_items_types;
use crate::ec_gen_phantom::{mod_main};                                  mod ec_gen_phantom;
use crate::ed_gen_phantom_testc_units::{mod_main};                      mod ed_gen_phantom_testc_units;
use crate::f0_scope::{mod_main};                                        mod f0_scope;
use crate::f1_scope_raii::{mod_main};                                   mod f1_scope_raii;
use crate::f2_scope_move::{mod_main};                                   mod f2_scope_move;
use crate::f3_scope_move_mut::{mod_main};                               mod f3_scope_move_mut;
use crate::f4_scope_move_partial_move::{mod_main};                      mod f4_scope_move_partial_move;
use crate::f5_scope_borrow::{mod_main};                                 mod f5_scope_borrow;
use crate::f6_scope_borrow_mut::{mod_main};                             mod f6_scope_borrow_mut;
use crate::f7_scope_borrow_alias::{mod_main};                           mod f7_scope_borrow_alias;
use crate::f8_scope_borrow_ref::{mod_main};                             mod f8_scope_borrow_ref;
use crate::f9_scope_lifetime::{mod_main};                               mod f9_scope_lifetime;
use crate::fa_scope_lifetime_explicit::{mod_main};                      mod fa_scope_lifetime_explicit;
use crate::fb_scope_lifetime_fn::{mod_main};                            mod fb_scope_lifetime_fn;
use crate::fc_scope_lifetime_methods::{mod_main};                       mod fc_scope_lifetime_methods;
use crate::fd_scope_lifetime_struct::{mod_main};                        mod fd_scope_lifetime_struct;
use crate::fe_scope_lifetime_trait::{mod_main};                         mod fe_scope_lifetime_trait;
use crate::ff_scope_lifetime_lifetime_bounds::{mod_main};               mod ff_scope_lifetime_lifetime_bounds;
use crate::fg_scope_lifetime_lifetime_coercion::{mod_main};             mod fg_scope_lifetime_lifetime_coercion;
use crate::fh_scope_lifetime_static_lifetime::{mod_main};               mod fh_scope_lifetime_static_lifetime;
use crate::fi_scope_lifetime_elision::{mod_main};                       mod fi_scope_lifetime_elision;
use crate::g0_trait::{mod_main};                                        mod g0_trait;
use crate::g1_trait_derive::{mod_main};                                 mod g1_trait_derive;
use crate::g2_trait_dyn::{mod_main};                                    mod g2_trait_dyn;
use crate::g3_trait_ops::{mod_main};                                    mod g3_trait_ops;
use crate::g4_trait_drop::{mod_main};                                   mod g4_trait_drop;
use crate::g5_trait_iter::{mod_main};                                   mod g5_trait_iter;
use crate::g5_trait_iter_a1::{mod_main};                                mod g5_trait_iter_a1;
use crate::g5_trait_iter_a2::{mod_main};                                mod g5_trait_iter_a2;
use crate::g5_trait_iter_a3::{mod_main};                                mod g5_trait_iter_a3;
use crate::g6_trait_impl_trait::{mod_main};                             mod g6_trait_impl_trait;
use crate::g7_trait_clone::{mod_main};                                  mod g7_trait_clone;
use crate::g8_trait_supertraits::{mod_main};                            mod g8_trait_supertraits;
use crate::g9_trait_disambiguating::{mod_main};                         mod g9_trait_disambiguating;
use crate::h0_macros::{mod_main};                                       mod h0_macros;
use crate::h1_macros_syntax::{mod_main};                                mod h1_macros_syntax;
use crate::h2_macros_designators::{mod_main};                           mod h2_macros_designators;
use crate::h3_macros_overload::{mod_main};                              mod h3_macros_overload;
use crate::h4_macros_repeat::{mod_main};                                mod h4_macros_repeat;
use crate::h5_macros_dry::{mod_main};                                   mod h5_macros_dry;
use crate::h6_macros_dsl::{mod_main};                                   mod h6_macros_dsl;
use crate::h7_macros_variadics::{mod_main};                             mod h7_macros_variadics;
use crate::i0_error::{mod_main};                                        mod i0_error;
use crate::i1_err_panic::{mod_main};                                    mod i1_err_panic;
use crate::i2_err_abort_unwind::{mod_main};                             mod i2_err_abort_unwind;
use crate::i3_err_option_unwrap::{mod_main};                            mod i3_err_option_unwrap;
use crate::i4_err_option_unwrap_q_mark::{mod_main};                     mod i4_err_option_unwrap_q_mark;
use crate::i5_err_option_unwrap_map::{mod_main};                        mod i5_err_option_unwrap_map;
use crate::i6_err_option_unwrap_and_then::{mod_main};                   mod i6_err_option_unwrap_and_then;
use crate::i7_err_result::{mod_main};                                   mod i7_err_result;
use crate::i8_err_res_map::{mod_main};                                  mod i8_err_res_map;
use crate::i9_err_res_alias::{mod_main};                                mod i9_err_res_alias;
use crate::ia_err_res_early_returns::{mod_main};                        mod ia_err_res_early_returns;
use crate::ib_err_res_enter_q_mark::{mod_main};                         mod ib_err_res_enter_q_mark;
use crate::ic_err_multiple_err_types::{mod_main};                       mod ic_err_multiple_err_types;
use crate::id_err_mult_err_types_option_result::{mod_main};             mod id_err_mult_err_types_option_result;
use crate::ie_err_mult_err_types_define_err_type::{mod_main};           mod ie_err_mult_err_types_define_err_type;
use crate::if_err_mult_err_types_boxing_errors::{mod_main};             mod if_err_mult_err_types_boxing_errors;
use crate::ig_err_mult_err_types_reenter_q_mark::{mod_main};            mod ig_err_mult_err_types_reenter_q_mark;
use crate::ih_err_mult_err_types_wrap_error::{mod_main};                mod ih_err_mult_err_types_wrap_error;
use crate::ii_err_iter_result::{mod_main};                              mod ii_err_iter_result;
use crate::j0_std::{mod_main};                                          mod j0_std;
use crate::j1_std_box::{mod_main};                                      mod j1_std_box;
use crate::j2_std_vec::{mod_main};                                      mod j2_std_vec;
use crate::j3_std_str::{mod_main};                                      mod j3_std_str;
use crate::j4_std_option::{mod_main};                                   mod j4_std_option;
use crate::j5_std_result::{mod_main};                                   mod j5_std_result;
use crate::j6_std_res_q_mark::{mod_main};                               mod j6_std_res_q_mark;
use crate::j7_std_panic::{mod_main};                                    mod j7_std_panic;
use crate::j8_std_hash::{mod_main};                                     mod j8_std_hash;
use crate::j9_std_hash_alt_key_types::{mod_main};                       mod j9_std_hash_alt_key_types;
use crate::ja_std_hash_hashset::{mod_main};                             mod ja_std_hash_hashset;
use crate::jb_std_rc::{mod_main};                                       mod jb_std_rc;
use crate::jc_std_arc::{mod_main};                                      mod jc_std_arc;
use crate::k0_std_misc::{mod_main};                                     mod k0_std_misc;
use crate::k1_std_misc_threads::{mod_main};                             mod k1_std_misc_threads;
use crate::k2_std_misc_threads_testc_mapreduce::{mod_main};             mod k2_std_misc_threads_testc_mapreduce;
use crate::k3_std_misc_channels::{mod_main};                            mod k3_std_misc_channels;
use crate::k4_std_misc_path::{mod_main};                                mod k4_std_misc_path;
use crate::k5_std_misc_file::{mod_main};                                mod k5_std_misc_file;
use crate::k6_std_misc_file_open::{mod_main};                           mod k6_std_misc_file_open;
use crate::k7_std_misc_file_create::{mod_main};                         mod k7_std_misc_file_create;
use crate::k8_std_misc_file_read_lines::{mod_main};                     mod k8_std_misc_file_read_lines;
use crate::k9_std_misc_process::{mod_main};                             mod k9_std_misc_process;
use crate::ka_std_misc_process_pipe::{mod_main};                        mod ka_std_misc_process_pipe;
use crate::kb_std_misc_process_wait::{mod_main};                        mod kb_std_misc_process_wait;
use crate::kc_std_misc_fs::{mod_main};                                  mod kc_std_misc_fs;
use crate::kd_std_misc_arg::{mod_main};                                 mod kd_std_misc_arg;
use crate::ke_std_misc_arg_matching::{mod_main};                        mod ke_std_misc_arg_matching;
use crate::kf_std_misc_ffi::{mod_main};                                 mod kf_std_misc_ffi;
use crate::l0_unsafe::{mod_main};                                       mod l0_unsafe;
use crate::l1_unsafe_asm::{mod_main};                                   mod l1_unsafe_asm;
use crate::m0_compatibility::{mod_main};                                mod m0_compatibility;
use crate::m1_comp_raw_identifiers::{mod_main};                         mod m1_comp_raw_identifiers;
use crate::n0_meta::{mod_main};                                         mod n0_meta;
use crate::n1_meta_doc::{mod_main};                                     mod n1_meta_doc;
use crate::n2_meta_playpen::{mod_main};                                 mod n2_meta_playpen;
                                                                        
T = TDD                                                                 
use crate::t0_testing::{mod_main};                                      mod t0_testing;
use crate::t1_test_unit_testing::{mod_main};                            mod t1_test_unit_testing;
use crate::t2_test_doc_testing::{mod_main};                             mod t2_test_doc_testing;
use crate::t3_test_integration_testing::{mod_main};                     mod t3_test_integration_testing;
use crate::t4_test_dev_dependencies::{mod_main};                        mod t4_test_dev_dependencies;
U = DATA 

*/
// End Of The Code Pit

