#![feature(plugin,const_fn)]
#![plugin(stainless)]
pub mod lexer_unit_tests;

#[macro_use(expect)]
extern crate expectest;

extern crate sql;
