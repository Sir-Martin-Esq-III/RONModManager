use fltk::{
    app,
    button::{self, Button},
    enums::{self, Color},
    frame::Frame,
    prelude::*,
    window::Window,
};

extern crate directories;
use directories::UserDirs;
use fltk_flex::Flex;

use std::{fs, io};

//Moves the file from one location to the other
fn move_file_to_dir(start_loc: &str, end_loc: &str, file_name: &str) {
    let from = format!("{}{}", start_loc, file_name);
    let to = format!("{}{}", end_loc, file_name);
    let res = fs::rename(from, to);
    match res {
        Err(_e) => println!("Faced an error when copying file {:?}", _e),
        Ok(_e) => println!("Copied succesfully"),
    };
}

//Check the path in the argument for paks and return them
fn fetch_packs_from_dir(dir: &str) -> Vec<String> {
    let dir = dir;
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
            Err(_e) => println!("folder already exists"),
            Ok(_e) => println!("Created succesfully"),
        }
    };
}
struct Pak_editor {
    name: Frame,
    remove: Button,
}

impl Pak_editor {
    pub fn new(h: i32, pak_name: &String) -> Pak_editor {
        let mut flex = Flex::default().with_size(400, h).row();
        let mut name = Frame::default().with_label(pak_name);
        let mut remove = Button::default().with_label("-");
        let mut add = Button::default().with_label("+").with_size(20, 20);
        //let mut add = Button::new(x + 200, y, 20, h, "+");

        flex.end();

        add.set_callback({
            let pak_n_clone = pak_name.clone();
            move |_| {
                move_file_to_dir(
                    "T:/Documents/RONMODLOADER/",
                    "T:/Steam/steamapps/common/Ready Or Not/ReadyOrNot/Content/Paks/",
                    &pak_n_clone,
                );
            }
        });

        remove.set_callback({
            let pak_n_clone = pak_name.clone();
            move |_| {
                move_file_to_dir(
                    "T:/Steam/steamapps/common/Ready Or Not/ReadyOrNot/Content/Paks/",
                    "T:/Documents/RONMODLOADER/",
                    &pak_n_clone,
                );
            }
        });
        Pak_editor { name, remove }
    }
}
fn main() {
    let gameFolder = "T:/Steam/steamapps/common/Ready Or Not/ReadyOrNot/Content/Paks/";
    let testUnloadedFolder = "T:/Documents/RONMODLOADER/";
    check_for_unloaded_pak_folder();
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 600, 300, "Ready or Not Mod Manager");
    wind.set_color(Color::White);
    let loaded_paks = fetch_packs_from_dir(gameFolder);

    let un_loaded_paks = fetch_packs_from_dir(testUnloadedFolder);

    println!("{:?}", loaded_paks);
    println!("{:?}", un_loaded_paks);

    let mut flex = Flex::default().with_size(600, 300).column();
    for l in 0..loaded_paks.len() {
        let offset = l as i32;
        Pak_editor::new(50, &loaded_paks[l]);
    }

    let unloadedOffset = 20 + (30 * loaded_paks.len() as i32);
    let unloaded = Frame::new(100, unloadedOffset + 50, 50, 50, "unloaded").set_label("unloaded");

    for j in 0..un_loaded_paks.len() {
        let offset = j as i32;
        Pak_editor::new(50, &un_loaded_paks[j]);
    }
    flex.end();

    wind.end();
    wind.make_resizable(true);
    wind.show();

    app.run().unwrap();
}
