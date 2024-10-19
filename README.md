# Task CLI

## Setting up the project

### Setting up the DB

To create the db, you will need `sqlite3` (you can install in various ways)

To install with pacman (PACkage MANager)
```bash
pacman -S sqlite3
```

Then to create the DB, you just need to run
```bash
sqlite3 task-cli-db.db
```

### Running

To run the project in dev mode

```bash
cargo run -- --help
```

This will allow you to get the options of the cli

### Building

To build the project, just run

```bash
cargo build
```

Then the cli tool should be in the `target` folder

```bash
./target/debug/task-cli --help
```


## Functionality
- Task
    - [x] Get
    - [x] Remove
    - [ ] Update
    - [ ] Complete
- Data Storage
    - [x] Read from json
    - [x] Save in json
    - [x] Migrate to some db
    - [x] Save to db
