# Notes CLI

A small interactive notes manager written in Rust language. Each note has a title and multiline content, and notes are stored locally as JSON.

## Features

- Add notes with a title and multiline content
- List saved notes
- View a note by its ID
- Delete a note by its ID
- Persist notes between runs in `notes.json`

## Requirements

- Rust and Cargo

## Run

From this project directory, start the app with:

```sh
cargo run
```

Choose an option from the menu. When adding a note, enter its content one line at a time and type `.` on a line by itself to finish.

## Data storage

The app reads and writes `notes.json` in its current working directory. If the file does not exist, it is created when the first note is saved. Keep this file to preserve your notes, or back it up to another location.
