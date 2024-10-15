use clap::Parser;
use rusqlite::Connection;
use task_cli::get_free_id;

use crate::table::table::connect_to_db;

fn get_id() -> i32 {
    let conn: Connection = connect_to_db().unwrap();
    get_free_id(&conn).unwrap()
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Task {
    #[arg(short, long, default_value_t = get_id())]
    pub id: i32,
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub done: bool,
}
