#![feature(proc_macro)]
extern crate trait_derive;

use trait_derive::make_trait;

pub struct A;

#[make_trait]
impl A {
    pub fn hello_world(&self) {
        println!("Hello, world!");
    }
}



fn main() {

}
