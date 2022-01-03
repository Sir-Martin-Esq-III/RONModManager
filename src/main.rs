use fltk::{app, button::Button, enums::Color, frame::Frame, prelude::*, window::Window};

extern crate directories;
use directories::UserDirs;

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
//Make impl for this
fn main() {
    check_for_unloaded_pak_folder();
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Ready or Not Mod Manager");
    wind.set_color(Color::White);
    let loaded_paks =
        fetch_packs_from_dir("T:/Steam/steamapps/common/Ready Or Not/ReadyOrNot/Content/Paks/");

    println!("{:?}", loaded_paks);

    let mut pak_list_2: Vec<Pak_editor> = vec![];

    for l in 0..loaded_paks.len() {
        let offset = l as i32;
        let mut newPak = Pak_editor {
            name: Frame::new(50, 20 * offset, 200, 20, "text").with_label(&loaded_paks[l]),
            remove: Button::new(300, 20 * offset, 100, 50, "-"),
        };
        pak_list_2.push(newPak);
    }

    //scroll.
    wind.end();
    wind.show();

    //Test button callback
    pak_list_2[0].remove.set_callback(move |_| {
        move_file_to_dir(
            "T:/Steam/steamapps/common/Ready Or Not/ReadyOrNot/Content/Paks/",
            "T:/Steam/steamapps/common/",
            "pakchunk99-Mods_WeaponUnlocker_P.pak",
        );
        wind.redraw(); // Is not correct...
    });

    app.run().unwrap();
}
