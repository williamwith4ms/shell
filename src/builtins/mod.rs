mod common;
pub mod exit;
pub mod help;

use std::collections::HashMap;

pub type BuiltinFn = fn();

pub fn get_builtins() -> HashMap<&'static str, BuiltinFn> {
    let mut map = HashMap::new();
    map.insert("exit", exit::exit as BuiltinFn);
    map.insert("help", help::help as BuiltinFn);
    
    map
}

pub fn count_builtins() -> usize {
    get_builtins().len()
}
