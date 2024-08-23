use serde::Deserialize;

#[derive(Deserialize)]

// Structure, that contains available languages
pub struct Langs {
    pub sk : Translates,
    pub ua : Translates,
    pub en : Translates,
    pub cz : Translates,
}

// All the translated phrases
#[derive(Deserialize)]
pub struct Translates {
    pub create_task  : String,
    pub task_creator : String,
    pub task_creator_intro : String,
    pub name : String,
    pub date : String,
    pub time : String,

    pub see_tasks : String,
    
    pub tasks_commands  : String,
    pub unknown_command : String,
}

// An implement for Languages structure, which involves the language setting process
impl Langs {
   pub fn lang(&self, language_index : usize) -> &Translates {
       match language_index {
           0 => &self.sk,
           1 => &self.ua,
           3 => &self.cz,
           _ => &self.en,
       }
   } 
}
