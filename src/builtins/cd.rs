use std::env::set_current_dir;

pub fn cd(args: &Vec<String>) {
    if args.is_empty() {
        eprintln!("shell: expected argument to \"cd\"");
    } else {
        match set_current_dir(args[0].clone()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{e}");
            }
        };
    }
}
