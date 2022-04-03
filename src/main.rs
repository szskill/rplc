use std::env;

fn main() {
    let mut args = env::args();

    // Ignore executable
    args.next();

    let text = args.next().unwrap();
    let from = args.next().unwrap();
    let to = args.next().unwrap();

    println!("{}", text.replace(&from, to.as_str()));
}
