// Task CLI
use commands::{args::cli, task::Task};
use lib::get_free_id;
use rusqlite::Result;
use table::table::{add_task, connect_to_db, create_table, show_all_tasks};

mod commands;
mod lib;
mod table;

fn main() -> Result<()> {
    let conn = connect_to_db().unwrap();
    let _ = create_table(&conn);

    let matches = cli(&conn).get_matches();

    let all_tasks = show_all_tasks(&conn);
    println!("{:#?}", all_tasks);

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            let task = Task {
                id: get_free_id(&conn)?,
                name: name.to_string(),
                done: false,
            };
            let _ = add_task(&conn, &task);
        }
        Some(("remove", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("id", id_matches)) => {
                    let id = id_matches.get_one::<String>("ID").unwrap();
                    println!("{:#?}", id)
                }
                _ => {
                    eprintln!("No id was provided")
                }
            }
        }
    }
    Ok(())
}
