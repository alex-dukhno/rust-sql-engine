#![feature(plugin,const_fn)]
#![plugin(stainless)]
pub mod lexer_tests;
pub mod parser_tests;
pub mod database_object_tests;
pub mod integration;

#[macro_use(expect)]
extern crate expectest;

extern crate sql;
