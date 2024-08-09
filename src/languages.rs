use serde::Deserialize;

#[derive(Deserialize)]
pub struct Langs {
    pub sk : Translates,
    pub ua : Translates,
    pub en : Translates
}

#[derive(Deserialize)]
pub struct Translates {
    pub create_task  : String,
    pub task_creator : String,
    pub task_creator_intro : String,
    pub name : String,
    pub date : String,
    pub time : String,

    pub see_tasks : String
}

impl Langs {
   pub fn lang(&self, language_index : usize) -> &Translates {
       match language_index {
           0 => &self.sk,
           1 => &self.ua,
           2 => &self.en,
           _ => &self.en
       }
   } 
}
