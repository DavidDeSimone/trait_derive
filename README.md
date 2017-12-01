trait_derive is a small crate that will generate a trait based on the impl blocks of your structs. This will save repitition in cases where you want to generate a trait for testing purposes, or you want to turn exsisting code into a trait for decoupling purposes.

To add it to your project, add the following line to your Cargo.toml

```
trait_derive = "0.1.0"
```

Currently, trait_derive requires you to be running _nightly_ Rust. This will hopefully change in the future, once proc macros are stable.

## Examples
Using this crate looks something like this: 
``` rust
#![feature(proc_macro)]
extern crate trait_derive;

use trait_derive::make_trait;

pub struct A;

#[make_trait] // This will generate a trait named 'TraitA' by default.
impl A {
    pub fn hello_world(&self) {
        println!("Hello, world!");
    }
}

// Behind the scenes, we have generated code equivalent to 
//trait TraitA {
//  fn hello_world(&self);
//}
//
//impl TraitA for A {
//  fn hello_world(&self) {
//    println!("Hello, world!");
//  }
//}

fn takes_trait(x: &TraitA) {
   x.hello_world();
}
    
fn basic_usage() {
   let a = A { };
   takes_trait(&a);
}

#[make_trait(Database)] // This will generate a trait named 'Database'
impl A {
    pub fn query(&self) -> u32 {
        32
    }
}
```