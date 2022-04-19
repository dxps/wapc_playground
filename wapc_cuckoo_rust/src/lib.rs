#[macro_use]
extern crate lazy_static;
extern crate cuckoofilter;

mod generated;

//extern crate wasm_bindgen;
use cuckoofilter::CuckooFilter;
use std::collections::hash_map::DefaultHasher;
extern crate wapc_guest as guest;

pub use generated::*;
use guest::prelude::*;

lazy_static! {
    static ref cf: CuckooFilter<DefaultHasher> = {
        let CF: CuckooFilter<DefaultHasher> = load_data();
        CF
    };
}
fn load_data() -> CuckooFilter<DefaultHasher> {
    let words = vec!["foo", "bar", "xylophone", "milagro"];
    let mut new_cf: CuckooFilter<DefaultHasher> = CuckooFilter::new();
    // let mut insertions = 0;
    for s in &words {
        if new_cf.test_and_add(s) {
            // insertions += 1;
        }
    }
    new_cf
}

#[no_mangle]
pub fn wapc_init() {
    Handlers::register_check_word_exists(check_word_exists);
    Handlers::register_handle_input(handle_input)
}
fn check_word_exists(member: String) -> HandlerResult<String> {
    let exists = cf.contains(&member);
    println!("{}", exists);
    Ok(exists.to_string())
}

fn handle_input(inp: generated::Input) -> HandlerResult<String> {
    let a = inp.x;
    Ok(a.to_string())
}
