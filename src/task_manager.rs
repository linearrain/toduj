use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Task {
    pub name    : String,

    pub yyyy    : u16,
    pub mm      : u16,
    pub dd      : u16,

    pub hrs     : u16,
    pub mnt     : u16,
    pub crossed : bool,
}

impl Task {
    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new().create(true)
            .write(true).append(true).open("tasks.csv")?;

        let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
        wtr.serialize(self)?;
        wtr.flush()?;
        Ok(())
    }
}

pub fn get_data() -> Result<Vec<Task>, Box<dyn Error>> {
    let mut tasks : Vec<Task> = vec![];

    let mut rdr = ReaderBuilder::new().has_headers(false).from_path("tasks.csv")?;

    for task in rdr.records() {
        let t = task?;
        let task_record = Task {
            name: t.get(0).unwrap().to_string(),
            yyyy: t.get(1).unwrap().parse::<u16>()?,
            mm:   t.get(2).unwrap().parse::<u16>()?,
            dd:   t.get(3).unwrap().parse::<u16>()?,
            hrs:  t.get(4).unwrap().parse::<u16>()?,
            mnt:  t.get(5).unwrap().parse::<u16>()?,
            crossed: t.get(6).unwrap().parse::<bool>()?,
        };
        tasks.push(task_record);
    }

    Ok(tasks)
}

pub enum Operation {
    Delete, Mark
}

pub fn perform_operation(op : Operation, filterout : &str, index : usize) -> Result<(), Box<dyn Error>> {
    let mut data = get_data()?;
    data.sort();

    match op {
        Operation::Delete => data = data.into_iter().filter(|el| el.name != filterout.to_string()).collect(),
        Operation::Mark   => data[index].crossed = true, 
    }

    let file = OpenOptions::new().create(true)
            .write(true).append(false).truncate(true).open("tasks.csv")?;

    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);
    for task in data {
        wtr.serialize(task)?;
    }
    wtr.flush()?;
    
    Ok(())
}

pub fn delete_task(filterout : &str) -> Result<(), Box<dyn Error>> {
    perform_operation(Operation::Delete, filterout, 0)?;
    Ok(())
}

pub fn mark_task(index : usize) -> Result<(), Box<dyn Error>> {
    perform_operation(Operation::Mark, "", index)?;
    Ok(())
}
