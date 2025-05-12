use crate::builtins::{count_builtins, list_builtins};

pub fn help(_args: &Vec<String>) {
    let _ = _args;
    println!("rsh: version 0.1.0");
    println!("There are {} builtin shell commands", count_builtins());
    list_builtins();
}
