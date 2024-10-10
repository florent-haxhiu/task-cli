// Task CLI

use std::{fs, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use clap::Parser;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Task {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = false)]
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

    let args = Task::parse();

    tasks.add_task(&args);

    let serialized = serde_json::to_string(&tasks).unwrap();

    println!("{:#?}", tasks);
    let _ = write_to_file("src/data.json", serialized);
}

fn write_to_file(data_path: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(data_path, &data)?;
    Ok(())
}
