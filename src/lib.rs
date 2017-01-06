#![feature(plugin)]

#![plugin(clippy)]

pub fn greeter(name: &str) {
    println!("hello, {}!", name)
}
