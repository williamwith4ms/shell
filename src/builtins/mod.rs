mod common;
pub mod exit;
pub mod help;
pub mod cd;

use std::collections::HashMap;

pub type BuiltinFn = fn(&Vec<String>);

pub fn get_builtins() -> HashMap<&'static str, BuiltinFn> {
    let mut map = HashMap::new();
    map.insert("exit",      exit::exit as BuiltinFn);
    map.insert("help",      help::help as BuiltinFn);
    map.insert("cd",        cd::cd as BuiltinFn); 
    map
}


pub fn count_builtins() -> usize {
    get_builtins().len()
}

pub fn list_builtins() {
    let builtins = get_builtins();
    for cmd in builtins.keys() {
        println!("- {}", cmd);
    }
}

