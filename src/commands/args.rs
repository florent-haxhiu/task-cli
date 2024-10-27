use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("task")
        .about("A task cli to keep track of shit to do")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .short_flag('a')
                .about("Add task")
                .arg(arg!(<NAME> "the name of the task")),
        )
        .subcommand(
            Command::new("remove")
                .short_flag('r')
                .about("Remove all tasks")
                .subcommand(
                    Command::new("id")
                        .short_flag('i')
                        .about("remove specific task")
                        .arg(arg!(<ID> "id of task to remove")),
                ),
        )
        .subcommand(
            Command::new("complete")
                .about("Complete a task")
                .short_flag('c')
                .arg(arg!(<ID> "id of task to complete")),
        )
        .subcommand(
            Command::new("show")
                .about("Show tasks")
                .short_flag('s')
                .subcommand(
                    Command::new("id")
                        .about("Get specific task")
                        .short_flag('i')
                        .arg(arg!(<ID> "the id of task")),
                ),
        )
}
