use std::env;
fn main() {
    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: bin [script]");
        std::process::exit(exitcode::NOINPUT)       
    }else if args.len() == 2 {
        //runfile(file)
        // run_file(&(args[1])[..]);
        run_file(&args[1]);
    }else{
        //runprompt
        
    }

}
fn run_file(file_path: &str) {
    // let file = match std::fs::read_to_string(file_path){
    //                 Ok(t) => t,
    //                 Err(_) => String::from("Error, could not load file"),
    //             };
    match std::fs::read_to_string(file_path){
        Ok(file) => run(file),
        Err(_) => println!("Error: Could not load file"),
    };
}
fn run(file: String) {
    print!("Sucess! string: \"{}\"",file);
}
