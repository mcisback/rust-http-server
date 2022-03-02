// use std::io::stdin;
use std::env;
mod server;

fn help(args: &Vec<String>) -> () {
    use std::process;

    println!("\nUsage:");

    println!("\t{} <bind_to_host> <bind_to_port>\n", args[0]);

    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    if args.len() < 3 {
        help(&args);
    }
    
    println!("Host: {}\nPort: {}\n", args[1], args[2]);
    
    // for arg in args {
    //     println!("ARG: {}", arg);
    // }


    server::run(args[1].as_str(), args[2].as_str());
}
