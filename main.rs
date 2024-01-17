extern crate todo_list;
use todo_list::Task;

use std::io::Write;
use std::io::stdin;
use std::io::stdout;

fn runprompt(todo: &mut Vec<task>){
    loop{
        let mut stdout = stdout();
        print("(todo list)> "); 
        stdout.flush().exoect("cant flush the stdout"); 
    }
}