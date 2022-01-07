use super::super::mod_c;

pub fn test() {
    println!("I'm mod_b");
}

fn call_mod_c() {
    mod_c::test();
}
