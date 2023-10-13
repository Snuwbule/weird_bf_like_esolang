use std::env;
use std::process;
use std::fs;

fn main() {
    let usr_args: Vec<String> = env::args().collect();
    if usr_args.len() < 2 {
        println!("Heeyyyy gib me one arg, u silly!\n(dat arg being da file u wanna run)");
        process::exit(1);
    } else if usr_args.len() >2 {
        println!("That's too much args! you silly!");
        process::exit(1);
    }
    
    //let mut bites;
    //let mut cur_bite : i32;  
}
