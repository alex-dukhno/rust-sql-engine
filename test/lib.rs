#![feature(plugin,const_fn)]
#![plugin(stainless)]
pub mod lexer_tests;
pub mod parser_tests;
pub mod database_object_tests;

#[macro_use(expect)]
extern crate expectest;

extern crate sql;
