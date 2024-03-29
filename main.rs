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
          //take the args into the run fn of lib and get the result of the run fn
          let args: Vec<&str> = buffer.split_whitespace().collect();

          todo_list::run(args, todo);
      }
  }

  fn main(){

    let mut todo: Vec<Task> = Vec::new();

    runprompt(&mut todo);
  }

  pub fn run (args: Vec<&str>, todo: &mut Vec<Task>){

    parse_arguments(args, todo);
  }

  #[derive(Debug)]
  pub struct Task {
    task: String,
    done_status: bool,
    id: u64,
  }

  fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>){
    let command = args[0];

    match command{
      var_name => {
          let var_name = "add";
          {

            if let Some(value) = args.get(1){
              let new_task = *value;
              add_new_task(todo_list, new_task);
              display_todo(todo_list);
            }else{
              println!("Please provide a new name for the task");
            }
          }
      },

      "show" => {
        display_todo(todo_list);
      },

      delete => {

        match &args[1].parse::<u64>(){
          Ok(value) => {
            remove_task(todo_list, *value);
          }

          Err(message) => {
            println!("{}", message.to_string());
          }
        }
      },

      "update" => {
      //possibilty 1: id parsing error
      match &args[1].parse::<u64>(){
        Ok(*value) => {

          //possibilty 2: task parsing error
        if let Ok(task) = get_task(todo_list, *value){

          //possibilty 3: no third argument provided
          if let Some(value) = args.get(2){
            let new_task = *value;
            task.update_task(new_task.into());
          }else{
            println!("no new task provided");
          }

        }else{
          println!("Task not found");
          }
        },

        Err(message) => {
          print!("{}", message);
        }
      }
      },

      "done" => {

        match &args[1].parse::<u64>(){
          Ok(value) => {
            if let Ok(task) = get_task(todo_list, *value){
              task.update_status();
          }else{
            println!("Task id not found list");
          }
        },
        Err(message) => {
          println!("{}", message.to_string());
          }
        }
      },

      "exit" => {
        std::process::exit(0);
      }

      "help" | _ => {
        println!("Commands: ");
        println!("add <task> - add a new task");
        println!("show - show all tasks");
        println!("delete <id> - delete a task");
        println!("update <id> <new task> - update a task");
        println!("done <id> - mark a task as done");
        println!("exit - exit the program");
      }
    }

    // instantiate a new AtomicU64 type to 1 and store in UNIQUE_ID.
    static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);
    fn add_new_task(todo_list: &mut Vec<Task>, task_string: &str){

        let id_no = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);

        let task: Task = Task{
            task: task_string.into(),
            done_status: false,
            id: id_no,
        };

        todo_list.push(task);

        println!("{} added to the todo list: ", task_string);
    }

    fn display_todo(todo_list: &Vec<Task>){
        if todo_list.len() < 1 {
            println!("Empty todo list");
            return;
        }

        for item in todo_list{
            println!("id: {}, name: {}, done: {}", item.id, item.task, item.done_status);
        }
    }
    fn remove_task(todo_list: &mut Vec<Task>, id_no: u64){

        todo_list.retain(|task| task.id != id_no);

    }

    #[derive(Debug)]
    pub struct Task{
        task: String,
        done_status: bool,
        id: u64,
    }

    impl Task{
        fn update_status(&mut self){
            self.done_status = true;
        }

        fn update_task(&mut self, new_name: String){
            self.task = new_name;
        }
    }

    fn get_task(todo_list: &mut Vec<Task>, task_id: u64) -> Result<&mut Task, &str>{

        for task in todo_list{
            if task.id == task_id{
                return Ok(task);
            }else{
                continue;
            }
        };

        return Err("Task not found in todo list");

    }
  }
