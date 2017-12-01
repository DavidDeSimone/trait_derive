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

#[make_trait(Database)]
impl A {
    pub fn x(&self) -> u32 {
        32
    }
}

pub struct B<T>(*mut T);

#[make_trait]
impl<T> B <T> {
    pub fn ptr_t(x: *mut T) -> B<T> {
        B(x)
    }
}

fn main() {
    println!("Run cargo test to make sure that everything compiles and runs");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn takes_trait(x: &TraitA) {
        x.hello_world();
    }

    fn takes_db(y: &Database) {
        y.x();
    }
    
    #[test]
    fn basic() {
        let a = A { };
        takes_trait(&a);
        takes_db(&a);
    }

    struct C;
    impl<C> TraitB<C> for C {
        fn ptr_t(x: *mut C) -> B<C> {
            B(x)
        }
    }
    
    #[test]
    fn generic() {
        let ptr = ::std::ptr::null_mut() as *mut C;
        C::ptr_t(ptr);
    }

}
