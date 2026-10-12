// TODO:
// add arg handling to main loop
// flesh out help with messages
// make the command handler a fucntion in another file.
use std::io;
fn main() {
    println!("
    Welcome to Prim-Sys R \n
    Consult provided documentation in the docs folder provided with the installed release. \n 
    Version 0.0.1 [DEV] \n
    Brought to you by the Prim-Sys dev team and Collaborators. 
    ");
    'primery:loop {
        print!(">");
        let command = {
            let mut com= String::new(); 
            io::stdin().read_line(&mut com).expect("Failed to read line"); // Assings a value to the empy string com
            com //assings com as a non mut
        }; 
        
        let mut command = command.trim().split_whitespace().collect::<Vec<&str>>(); 
        //Splits command into command and argument via a a str reference
        if command.len() == 0 {
            print!("No command entered");
            continue;
        }
        if command.len() == 1 {
            command.push("");
        }
        match command[0] {
            "exit" => break 'primery,
            "help" => help(command[1]),
            "math" => math(command[1]),
            _ => println!("Unknow command")
        } //Matches the command and passes the argument into the pertinent function

    }
}

fn help(arg : &str){
    match arg {
        "" => println!("Test"),
        "math" => println!("Mathematics fuction, available flags: \n -f : function flag, enable the function parser for determining a fucntion [NOT IMPLEMENTED] 
                            \n -s : sin flag, for computing the respective sine of some angle [NOT IMPLEMENTED] 
                            \n -c : cos flag, for computing the respective cosine of some angle [NOT IMPLEMENTED] 
                            \n -m : memory flag, for saving the result of all following operations into a specified register,"),
        "exit" => println!("exit the user process"),
        _ => println!("Invalid argument")
    }
}

fn math(arg : &str) {
    match arg {
        "" => println!("Insert an argument for the required computation, fo the list of all arguments use \n help math"),
        _ => println!("Invalid argument")
    }
}
