// Task CLI
use commands::{args::cli, task::Task};
use rusqlite::Result;
use table::queries::{
    add_task, complete_specific_task, connect_to_db, create_table, delete_specific_task,
    get_task_from_db, remove_all_inside_table, show_all_tasks,
};
use task_cli::get_free_id;

mod commands;
mod table;

fn main() -> Result<()> {
    let conn = connect_to_db().unwrap();
    let _ = create_table(&conn);

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            let task = Task {
                id: get_free_id(&conn).unwrap(),
                name: name.to_string(),
                done: false,
            };
            let _ = add_task(&conn, &task, false);
        }
        Some(("remove", sub_matches)) => match sub_matches.subcommand() {
            Some(("id", id_matches)) => {
                let id = id_matches.get_one::<String>("ID").unwrap();
                let _ = delete_specific_task(&conn, id);
            }
            _ => {
                let _ = remove_all_inside_table(&conn);
                println!("Removed all tasks")
            }
        },
        Some(("complete", sub_matches)) => match sub_matches.subcommand() {
            _ => {
                let id = sub_matches.get_one::<String>("ID").unwrap();
                let _ = complete_specific_task(&conn, id);
            }
        },
        Some(("show", _sub_matches)) => match _sub_matches.subcommand() {
            Some(("id", id_matches)) => {
                let id = id_matches.get_one::<String>("ID").unwrap();
                let _ = get_task_from_db(&conn, id);
            }
            _ => {
                let _ = show_all_tasks(&conn);
            }
        },
        _ => {
            eprintln!("What the freak")
        }
    }

    Ok(())
}
