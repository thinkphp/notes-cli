use serde::{Deserialize, Serialize};
use std::{error::Error, fs, io, io::Write, path::Path};

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: u64,
    title: String,
    content: String,
}

fn load_notes(path: &Path) -> Result<Vec<Note>, Box<dyn Error>> {
    match fs::read_to_string(path) {
        Ok(data) => Ok(serde_json::from_str(&data)?),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error.into()),
    }
}

fn save_notes(path: &Path, notes: &[Note]) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(notes)?;
    fs::write(path, data)?;
    Ok(())
}

fn read_line(prompt: &str) -> io::Result<Option<String>> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut input = String::new();
    if io::stdin().read_line(&mut input)? == 0 {
        return Ok(None);
    }
    Ok(Some(input.trim_end_matches(['\r', '\n']).to_owned()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let storage_path = Path::new("notes.json");
    let mut notes = load_notes(storage_path)?;

    'menu: loop {
        println!("\n=== Notes ===");
        println!("1. Add a note");
        println!("2. List notes");
        println!("3. View a note");
        println!("4. Delete a note");
        println!("0. Exit");

        let Some(choice) = read_line("Choose an option: ")? else {
            break;
        };
        match choice.trim() {
            "1" => {
                let Some(title) = read_line("Title: ")? else {
                    break;
                };
                if title.trim().is_empty() {
                    println!("The title cannot be empty.");
                    continue;
                }

                println!("Content (type . on a separate line to finish):");
                let mut content_lines = Vec::new();
                loop {
                    let Some(line) = read_line("")? else {
                        break 'menu;
                    };
                    if line == "." {
                        break;
                    }
                    content_lines.push(line);
                }

                let id = notes
                    .iter()
                    .map(|note| note.id)
                    .max()
                    .unwrap_or(0)
                    .checked_add(1)
                    .ok_or("Cannot generate a new note ID")?;
                notes.push(Note {
                    id,
                    title,
                    content: content_lines.join("\n"),
                });
                save_notes(storage_path, &notes)?;
                println!("Note saved.");
            }
            "2" => {
                if notes.is_empty() {
                    println!("No notes saved.");
                } else {
                    for note in &notes {
                        println!("{}. {}", note.id, note.title);
                    }
                }
            }
            "3" => {
                let Some(id_input) = read_line("Note ID: ")? else {
                    break;
                };
                let id = id_input.parse::<u64>();
                match id
                    .ok()
                    .and_then(|id| notes.iter().find(|note| note.id == id))
                {
                    Some(note) => println!("\n{}\n\n{}", note.title, note.content),
                    None => println!("Note not found."),
                }
            }
            "4" => {
                let Some(id_input) = read_line("Note ID to delete: ")? else {
                    break;
                };
                let id = id_input.parse::<u64>();
                match id {
                    Ok(id) => {
                        let original_len = notes.len();
                        notes.retain(|note| note.id != id);
                        if notes.len() == original_len {
                            println!("Note not found.");
                        } else {
                            save_notes(storage_path, &notes)?;
                            println!("Note deleted.");
                        }
                    }
                    Err(_) => println!("Invalid ID."),
                }
            }
            "0" => break,
            _ => println!("Invalid option."),
        }
    }

    Ok(())
}
