use std::io::Write;



enum TodoState {
    TODO,
    DOING,
    DONE
}

#[derive(Clone, Debug)]
struct Todo {
    name: String,
    description: String
}

impl Todo {
    fn to_string(&self) -> String{
        return format!("Todo (name: '{}', description: '{}')", self.name, self.description);
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool{
       return self.name == other.name
    }        
}

#[derive(Debug)]
struct KanBanBoard {
    todo: Vec<Todo>,
    doing: Vec<Todo>,
    done: Vec<Todo>
}

impl KanBanBoard {
    fn add(&mut self, todo: &Todo, state: TodoState) {
        match state {
            TodoState::TODO => self.todo.push(*todo),
            TodoState::DOING => self.doing.push(*todo),
            TodoState::DONE => self.done.push(*todo)
        }
    }

    fn remove(&mut self, todo: &Todo, state: TodoState){
        match state {
            TodoState::TODO => self.todo.retain(|x| x != todo),
            TodoState::DOING => self.doing.retain(|x| x != todo),
            TodoState::DONE => self.done.retain( |x| x != todo)
        }

    }

    fn change_state(&mut self, todo: &Todo, from: TodoState, to: TodoState) {
        self.remove(todo, from);
        self.add(todo, to)
        
    }

    fn find_todo(&mut self, name: &String) -> Option<(&Todo, TodoState)> {
    
        match self.todo.iter().find(|probe| (**probe).name == *name) {
            Some(todo) => Some((todo, TodoState::DOING)),
            None => {
                match self.doing.iter().find(|probe| (**probe).name == *name) {
                    Some(todo) => Some((todo, TodoState::DOING)),
                    None => {
                        match self.done.iter().find(|probe| (**probe).name == *name) {
                            Some(todo) => Some((todo, TodoState::DOING)),
                            None => {None}
                        }
                    }
                }
            }
        }
    }
}


fn read_stdin(message: String) -> String{
    use std::io;
    let mut input = String::new();

    print!("{}: ", message);
    io::stdout().flush();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.ends_with("\n") {
                input.pop();
                
                if input.ends_with("\r") { 
                    input.pop();
                }
            }
            input
        }
        Err(error) => {
            println!("error: {error}");
            input
        },
    }
}

struct View { 
    kanban: KanBanBoard
}

impl View {
    fn new_todo(&mut self) {
        let name = read_stdin("Nome da task".to_string());
        let description = read_stdin("Descricao".to_string());

        let todo = Todo {name, description};
        self.kanban.add(&todo, TodoState::TODO);        
    }

    fn move_todo(&mut self) {
        for i in self.kanban.todo.iter() {
            println!("TODO LANE: {:?}", i);
        }

        for i in self.kanban.doing.iter() {
            println!("DOING LANE: {:?}", i);
        }
        
        let name = read_stdin("Qual o nome da tarefa que deseja mover?".to_string());
        let to = match read_stdin("Digite 1 para TODO, 2 para DOING ou 3 para DONE".to_string()).as_str() {
            "1" => TodoState::TODO,
            "2" => TodoState::DOING,
            "3" => TodoState::DONE,
            _ => { println!("Entrada Inválida. Considerando 1 como padrão."); TodoState::TODO }
        };
        
        match self.kanban.find_todo(&name) {
            Some((todo, from)) => self.kanban.change_state(todo, from, to),
            None => { }
        }
        
    }
}

fn main(){

    let mut kb = KanBanBoard { todo: vec![], doing: vec![], done: vec![] };

    loop {
        
        let name = read_stdin("Nome da task".to_string());
        let description = read_stdin("Descricao".to_string());

        let todo = Todo {name, description};
        kb.add(&todo, TodoState::TODO);

        println!("{:?}", kb);

    }


}