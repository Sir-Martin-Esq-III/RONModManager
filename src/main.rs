use fltk::{
    app, button::Button, enums::Color, frame::Frame, prelude::*, text::TextDisplay, window::Window,
};

extern crate directories;
use directories::UserDirs;

use std::{fs, io};

//Check the game location for a list of the paks
fn fetch_loaded_paks() -> Vec<String> {
    let dir = "T:/Steam/steamapps/common/Ready Or Not/ReadyOrNot/Content/Paks/";
    let mut pakList: Vec<String> = vec![];
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let tmp = path.unwrap().path();
        let currentPath: Vec<&str> = tmp.to_str().unwrap().split('/').collect();
        pakList.push(currentPath[currentPath.len() - 1].to_string());
    }
    pakList
}
/*
On load, try and create a RONMODLOADER file
if err ? folder exists: created successfully
*/
fn check_for_unloaded_pak_folder() {
    if let Some(user_dirs) = UserDirs::new() {
        let folder = user_dirs.document_dir().unwrap().to_str().unwrap();
        let find_folder = format!("{}/RONMODLOADER", folder);
        let dir = fs::create_dir(&find_folder);
        match dir {
            Err(_e) => print!("folder already exists"),
            Ok(_e) => println!("Created succesfully"),
        }
    };
}
fn main() {
    check_for_unloaded_pak_folder();
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Ready or Not Mod Manager");
    wind.set_color(Color::White);
    let pak_list = fetch_loaded_paks();

    println!("{:?}", pak_list);
    for l in 0..pak_list.len() {
        let offset = l as i32;
        let mut newText = Frame::new(50, 20 * offset, 200, 20, "text").with_label(&pak_list[l]);
        let mut newButton = Button::new(300, 20 * offset, 100, 50, "text");
    }

    let mut textDisply = TextDisplay::new(150, 400, 200, 200, "title");
    wind.end();
    wind.show();
    app.run().unwrap();
}
