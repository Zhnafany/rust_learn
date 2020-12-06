use std::collections::HashMap;
use crate::read_from_input::read_string;

#[derive(Debug)]
struct Command {
    sort: CommandType,
    params: String,
    full_text: String
}

#[derive(Debug)]
enum CommandType {
    Add,
    Department,
    All,
    Help,
    Quit
}

impl Command {
    pub fn new(text_line: &str) -> Command {
        let mut command = Command {
            sort: CommandType::Help,
            params: String::new(),
            full_text: text_line.to_string()
        };
        command.parse();
        command
    }

    fn parse(&mut self)  {
        self.set_command();    
        self.set_params();
    }

    fn set_command(&mut self) {
       self.sort = match self.full_text.split_whitespace().next() {
           Some(command) => match command {
               "Add" => CommandType::Add,
               "Department" => CommandType::Department,
               "All" => CommandType::All,
               "Quit" => CommandType::Quit,
               _ => CommandType::Help,
           },
           None => CommandType::Help
        };
    }

    fn set_params(&mut self) {
         let right_side = match self.full_text.find(' ') {
            Some(i) => self.full_text.split_at(i).1.trim(),
            None => ""
        };
        self.params = right_side.to_string();
    }
}

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}


impl Company {
    fn add_people_to_depart(&mut self, params: &str) {
        let ppl_and_dep: Vec<&str> = params.split_terminator(" to ").collect();
        let ppl_in_dep = self.departments.entry(ppl_and_dep[1].into())
                                 .or_insert(Vec::new());
        ppl_in_dep.push(ppl_and_dep[0].into());
    }

    fn print_depart_with_ppl(&self, dep_name: &str) {
        match self.departments.get_key_value(dep_name) {
            Some((dep, ppl_vec)) => {
                println!("{}: ", dep);
                for ppl in ppl_vec.iter() {
                    println!(" - {}", ppl);
                }
            },
            None => println!("No such department!"),
        }
    }

    fn print_all(&self) {
        let mut sorted_deps = self.departments.keys().collect::<Vec<&String>>();
        sorted_deps.sort();
        for dep in sorted_deps {
            self.print_depart_with_ppl(dep);
        }
    }


    fn help(&self) {
        println!("Commands: Add, Department, All, Help, Quit");
    }

    fn company_cli_loop(&mut self) {
        loop {
            let command_line = read_string("Enter command(write Help for help): ");
            let command = Command::new(&command_line[..]);
            match command.sort {
                CommandType::Add => self.add_people_to_depart(&command.params),
                CommandType::Department => self.print_depart_with_ppl(&command.params),
                CommandType::All => self.print_all(),
                CommandType::Help => self.help(),
                CommandType::Quit => break,
            }
        };
    }

    pub fn company_cli() {
        let mut company = Company {
            departments: HashMap::new()
        };
        
        company.company_cli_loop();
    }
}
