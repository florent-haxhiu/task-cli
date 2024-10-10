// Task CLI

use std::{fs, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

//use clap::Parser;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    //#[arg(short, long)]
    name: String,

    //#[arg(short, long, default_value_t = false)]
    done: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tasks {
    data: Vec<Task>,
}

impl Tasks {
    fn add_task(&mut self, task: &Task) {
        let vec = &mut self.data;
        vec.push(task.clone());
    }
}

fn main() {
    let file = fs::File::open("src/data.json").expect("file should be open");
    let buffer: serde_json::Value = from_reader(file).expect("File should be proper JSON");
    let mut tasks: Tasks = serde_json::from_str(&buffer.to_string()).unwrap();

    println!("{:#?}", tasks);

    let name = String::from_str("Florent").unwrap();
    let task = Task { name, done: false };

    tasks.add_task(&task);

    let _serialized = serde_json::to_string(&task).unwrap();

    println!("{:#?}", tasks);
    println!("{}", buffer);

    //let args = Task::parse();

    //println!("Hello {}", args.name);
}
