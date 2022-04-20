#[macro_use]
extern crate lazy_static;
extern crate cuckoofilter;

mod generated;

use cuckoofilter::CuckooFilter;
use std::collections::hash_map::DefaultHasher;
extern crate wapc_guest as guest;

pub use generated::*;
use guest::prelude::*;

lazy_static! {
    static ref CF: CuckooFilter<DefaultHasher> = load_data();
}

fn load_data() -> CuckooFilter<DefaultHasher> {
    let words = vec!["foo", "bar", "xylophone", "milagro"];
    let mut new_cf: CuckooFilter<DefaultHasher> = CuckooFilter::new();
    for s in &words {
        new_cf.test_and_add(s).unwrap();
    }
    new_cf
}

#[no_mangle]
pub fn wapc_init() {
    Handlers::register_check_word_exists(check_word_exists);
    Handlers::register_handle_input(handle_input)
}

fn check_word_exists(member: String) -> HandlerResult<String> {
    let exists = CF.contains(&member);
    println!("{}", exists);
    Ok(exists.to_string())
}

fn handle_input(inp: generated::Input) -> HandlerResult<String> {
    let a = inp.x;
    Ok(a.to_string())
}
