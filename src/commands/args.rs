use clap::{Arg, Command};
use rusqlite::Connection;

use crate::table::table::drop_table;

fn cli(conn: &Connection) -> Command {
    let what = drop_table(&conn);

    Command::new("task")
        .about("A task cli to keep track of shit to do")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .short_flag('a')
                .long_flag("add")
                .about("Add task")
        )
        .subcommand(
            Command::new("nuke")
                .short_flag('d')
                .long_flag("nuke")
                .about("Remove all tasks")
                .arg(
                    Arg::new("Placeholder")
                )
        )
        .subcommand(
            Command::new("complete").about("Complete a task").arg(
                Arg::new("")
            )
            
        )
}
