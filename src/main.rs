use std::env;


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


fn generate_key(qnt:u32) {
    println!("{}", qnt);
}

fn code() {}

fn decode() {}




fn main() {
    handle_commands();
}
