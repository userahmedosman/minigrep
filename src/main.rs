use std::{env, process};
use minigrep::*;
fn main() {

   
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e | {
        
        eprintln!("problem parsing argument: {e}");
        process::exit(1);
    });

    

   if let Err(e) = run(config){
    eprintln!("Application error: {e}");
    process::exit(1);
   }
       

}

