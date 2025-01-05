use anchor_lang::prelude::*;
use crate::errors::TodoError;

#[derive(Clone, AnchorDeserialize, AnchorSerialize, InitSpace)]
pub struct Todo{
    #[max_len(15)]
    pub title: String, 

    #[max_len(100)]
    pub description: Option<String>,

    pub done: bool,
}

#[account]
#[derive(InitSpace)]
pub struct TodosAccount {
    #[max_len(5)]
    pub todos: Vec<Option<Todo>>,
    pub bump: u8, 
}

impl TodosAccount {

    pub fn add_todo(&mut self, data:Todo)->Result<()>{
        
        let todos: &mut Vec<Option<Todo>> = &mut self.todos;

        let mut add_todo_ind: usize = 0;

        let mut limit_reached: bool = true;
        
        for (ind, todo) in todos.iter().enumerate() {

            if let None = todo {
                add_todo_ind = ind;
                limit_reached = false;
                break;
            }
        }

        require!(!limit_reached, TodoError::ReachedLimit);

        todos[add_todo_ind] = Some(data);

        Ok(())
    }
}