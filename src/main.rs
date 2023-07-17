// TODO: Organize the code into different files
mod api;
mod clean;
mod database;
mod note;
mod output;
mod prompts;

use crate::note::Note;
use crate::prompts::{confirm::confirm, input::input, multiselect::multiselect, select::select};
use async_std::path::PathBuf;
use chrono::prelude::*;
use directories::BaseDirs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_directory: PathBuf = BaseDirs::new().unwrap().config_dir().into();
    let db_file = data_directory.join("Notabena").join("notes.db");
    api::init_db(&data_directory, &db_file)?;
    cursor_to_origin()?;
    println!("Welcome to Notabena!");

    loop {
        let options = vec![
            "New note",
            "View note",
            "Edit note",
            "Delete note",
            "About",
            "Exit",
        ];

        match select("What do you want to do?", &options) {
            0 => new_note(&db_file).expect("Creating a new note failed"),
            1 => show_notes(&db_file).expect("Failed fetching the notes"),
            2 => edit_notes(&db_file).expect("Editing the note failed"),
            3 => delete_notes(&db_file).expect("Deleting the note failed"),
            4 => display_about().expect("Viewing about failed"),
            _ => return Ok(()),
        }
    }
}

fn new_note(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let inputted_note = Note {
        id: api::get_notes(db_file)?.len(),
        name: input("Name:", "".to_string()),
        content: input("Content:", "".to_string()),
        created: format!("{}", Local::now().format("%A %e %B, %H:%M")),
    };

    cursor_to_origin()?;
    println!("This is the note you're about to create:");
    display_note(&inputted_note)?;

    match confirm("Do you want to save this note?") {
        true => {
            api::save_note(&inputted_note, db_file)?;
            cursor_to_origin()?;
            println!("Note created successfully.");
            Ok(())
        }
        false => Ok(()),
    }
}

fn show_notes(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes(db_file)?;

    if saved_notes.is_empty() {
        println!("You don't have any notes.");
        Ok(())
    } else {
        let mut options: Vec<String> = Vec::new();
        truncated_note(&mut options, db_file)?;
        let selection = select("Select the note that you want to view:", &options);
        let selected_note = &saved_notes[selection];
        display_note(selected_note)?;

        Ok(())
    }
}

fn edit_notes(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let saved_notes = api::get_notes(db_file)?;
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options, db_file)?;
    let selection = select("Select the note that you want to edit:", &options);

    if saved_notes.is_empty() {
        cursor_to_origin()?;
        println!("You can't edit notes, because there are none.");
        Ok(())
    } else {
        let selected_note = &saved_notes[selection];
        let updated_note = Note {
            id: selection,
            name: input("Name:", selected_note.name.clone()),
            content: input("Content:", selected_note.content.clone()),
            created: selected_note.created.clone(),
        };

        match confirm("Are you sure that you want to edit this note?") {
            true => {
                api::edit_note(&updated_note, selection, db_file)?;
                cursor_to_origin()?;
                println!("Note updated successfully.");
                Ok(())
            }
            false => Ok(()),
        }
    }
}

fn delete_notes(db_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut options: Vec<String> = Vec::new();
    truncated_note(&mut options, db_file)?;
    let selections = multiselect(
        "Select the note(s) that you want to delete:\nSpace to select, Enter to confirm.",
        options,
    );

    let mut prompt = "Are you sure that you want to delete these notes?";
    if selections.len() == 1 {
        prompt = "Are you sure that you want to delete this note?";
    }

    cursor_to_origin()?;
    if api::get_notes(db_file)?.is_empty() {
        println!("You can't delete notes, because there are none.");
        Ok(())
    } else if selections.is_empty() {
        println!("You didn't select any notes.");
        return Ok(());
    } else {
        if confirm(prompt) {
            api::delete_notes(selections, db_file)?;
        }
        println!("Notes deleted successfully.");
        return Ok(());
    }
}

fn display_about() -> Result<(), Box<dyn std::error::Error>> {
    println!("Notabena is a FOSS note-taking CLI tool, written in Rust.");
    println!("License: GPL v3\n");
    println!("COPYRIGHT (c) 2023 NOTABENA ORGANISATION\nPROJECT LEADS @ThatFrogDev, @MrSerge01, GITHUB CONTRIBUTORS");

    Ok(())
}

fn display_note(note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================");
    println!("Name: {}", note.name);
    println!("Content: {}", note.content);
    println!("Created at: {}", note.created);
    println!("=======================");

    Ok(())
}

fn truncated_note(
    options: &mut Vec<String>,
    db_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    for note in &api::get_notes(db_file)? {
        let mut truncated_content: String = note.content.chars().take(10).collect();
        if truncated_content.chars().count() == 10 {
            truncated_content += "...";
        }

        options.push(format!(
            "{} | {} | {}",
            note.name, truncated_content, note.created
        ));
    }
    Ok(())
}

fn cursor_to_origin() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).spawn()?.wait()?;
        Ok(())
    } else {
        Command::new("clear").spawn()?.wait()?;
        Ok(())
    }
}
