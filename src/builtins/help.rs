use crate::builtins::count_builtins;

pub fn help() {
    println!("rshell: version 0.1.0");
    println!("There are {} builtin shell commands", count_builtins());
}
