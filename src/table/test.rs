use rusqlite::Connection;

use crate::commands::task::Task;

use super::queries::*;

#[test]
fn test_get_specific_task_from_db() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    let _ = create_table(&conn);
    let task = Task {
        id: 10,
        name: "what".to_string(),
        done: false,
    };
    let task_two = Task {
        id: 9,
        name: "huh".to_string(),
        done: false,
    };
    let _ = add_task(&conn, &task);
    let _ = add_task(&conn, &task_two);

    let actual_task = get_task_from_db(&conn, &10.to_string())?;

    assert_eq!(task, actual_task);

    Ok(())
}

#[test]
fn test_retuns_all_tasks_in_db() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    let _ = create_table(&conn);
    let tasks: Vec<Task> = vec![
        Task {
            id: 1,
            name: "bob".to_string(),
            done: false,
        },
        Task {
            id: 2,
            name: "jon".to_string(),
            done: false,
        },
        Task {
            id: 3,
            name: "joe".to_string(),
            done: false,
        },
        Task {
            id: 4,
            name: "dan".to_string(),
            done: false,
        },
    ];

    for task in tasks.clone().into_iter() {
        let _ = add_task(&conn, &task);
    }

    let actual_vec = show_all_tasks(&conn)?;

    assert_eq!(tasks, actual_vec, "Didn't match");

    Ok(())
}
