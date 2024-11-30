mod tokenizer;
use std::io::{ stdin, stdout, Write };
use crate::tokenizer::Tokenizer;

fn main(){
    println!("Welcome to the PlecakDB monitor");
    println!("Commands ends with ';'");
    println!("Type .help for help");

    let mut multiline_buffer = String::new();
    let mut command_log: Vec<String>  = Vec::new();
    loop{
        if multiline_buffer.is_empty(){
            print!("PlecakDB [(dbname)]> ");
        }
        else{
            print!("...>  ");
        }
        let _ = stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Input error");
        let input = input.trim();
        if input.is_empty(){
            continue;
        }

        if input.starts_with('.') && multiline_buffer.is_empty(){
            match input{
                ".exit" => {
                    println!("Goodbye!");
                    break;
                } 
                ".help" =>  {
                    println!("Available commands:");
                    println!("  .exit      - Exit the REPL");
                    println!("  .history   - Show history of commands");
                    println!("  All other inputs are treated as SQL commands.");
                }
                ".history" => {
                    for i in (0..command_log.len()).rev(){
                        println!("{}.  {}", i+1, command_log[i]);
                    }
                }
                _ => {
                    println!("Wrong command!");
                }
            }
            continue;
        }
        if !input.ends_with(';'){
            multiline_buffer.push_str(input);
            multiline_buffer.push(' ');
            continue;
        } 
        else{
            multiline_buffer.push_str(input);
        }
        let command = multiline_buffer.trim().to_string();
        multiline_buffer.clear();
        let mut tokenizer = Tokenizer::new(command.as_str());
        let tokens = tokenizer.tokenize();
        for token in tokens{
            println!("{:?}", token);
        }
        command_log.push(command);
    }
}