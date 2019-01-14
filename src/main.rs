use turtle;

use std::env;

const DEFAULT_PROJECT: &str = "default";

fn main() {
    let args: Vec<_> = env::args().collect();

    let project = if args.len() > 1 {
        args[1].to_lowercase()
    } else {
        String::from(DEFAULT_PROJECT)
    };

    turtle::run(&project);
}
