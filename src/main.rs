use std::env;
use std::net::IpAddr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    //take in string vector references as arguments, and return the argument struct in ok
    //portion (first) or static string in err portion (second). Used static to send back errors to
    //main function and have it handle the errors
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
       
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
}
