use std::env;
use std::process;
use cardflip;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("There should be exactly one argument");
        process::exit(1);
    }

    let strategy = cardflip::run(&args);

    if let Err(e) = strategy {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    } else if let Ok(res) = strategy {
        println!("The winning strategy is: {:?}", res)
    }
}
