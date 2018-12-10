#[macro_use]
extern crate inherit_derive;

struct A {
    x: i32,
    foo: &'static str,
}

#[derive(Inherit)]
struct B {
    base: A,
    y: i32,
}

fn main() {
    let b = B {
        base: A {
            x: 0,
            foo: "FFF",
        },
        y: 0
    };

    println!("Hello, world! {}", b.foo);
}
