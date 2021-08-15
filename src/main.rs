use std::env;
use rand::Rng;


fn handle_commands() {
    let mut arguments: Vec<String> = Vec::new();


    for argument in env::args() {
        arguments.push(String::from(argument));
    }arguments.remove(0);


    if arguments[0] == "new-key"{
        generate_key(arguments[1].parse::<u32>().unwrap());
        
    }else if arguments[0] == "code"{
        println!("code");
    }else if arguments[0] == "decode"{
        println!("decode");
    }

    println!("ARGUMENTS::: {:?}", arguments);
}


fn range(qnt:u32) -> u32 {
    rand::thread_rng().gen_range(std::ops::Range{start: 0, end: qnt})
}


fn generate_key(qnt:u32) {
    let mut key_vec:Vec<u32> = Vec::new();
    let mut n_for_comparation;
    let mut repeat:bool = true;

    let mut n:u32 = range(qnt);
    key_vec.push(n);
    
    for _i in (std::ops::Range{start: 0, end: qnt - 1}){    
        while repeat{
            n = range(qnt);
            n_for_comparation = n;
            if key_vec.iter().filter(|&n| *n == n_for_comparation).count() == 0{
                key_vec.push(n);
                repeat = false;
            }else{
                repeat = true;
            }
        }repeat = true;
    }
    println!("{:?}", key_vec);
}

fn code() {}

fn decode() {}



fn main() {
    handle_commands();
}
