// This should fail even without validation
// compile-flags: -Zmir-emit-validate=0

#![feature(never_type)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

enum Void {}

fn f(v: Void) -> ! {
    match v {} //~ ERROR constant evaluation error [E0080]
    //~^ NOTE entered unreachable code
}

fn main() {
    let v: Void = unsafe {
        std::mem::transmute::<(), Void>(())
    };
    f(v); //~ inside call to `f`
}
