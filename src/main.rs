mod commands;
mod args;
mod images;

use std::{thread, time};
use std::io::{stdout, Write};
use termion::clear;
use termion::cursor;
use std::fs;

//ImgSqc138fbf84uw4du844edf <- folder name
fn main() -> Result<(), images::ImageDataError>{
    ctrlc::set_handler(move || {
        commands::end_program();
    })
    .expect("Error setting Ctrl-C handler");


    const DENSITY: [char; 18] = [' ','.','´','·','-','º','"','=','+','f','n','o','m','M','#','W','Ñ','@'];
    let args = args::Args::new();
    commands::prepare_vid(&args.path); //en este punto ya esta la carpeta con todas las fotos

    print!("{}{}", clear::All, cursor::Goto(1, 1));
    for img in fs::read_dir("ImgSqc138fbf84uw4du844edf").expect("failed to read directory ImgSqc138fbf84uw4du844edf"){
        let img = img.expect("Failed to read file");
        let path = img.path().to_str().unwrap().to_string();

        let (w,h,pic) = images::find_image_from_path(path)?;
        images::to_ascii(pic, &DENSITY, w as u16 , h as u16)?;
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(166));
        print!("{}{}", clear::All, cursor::Goto(1, 1));
    }
    commands::end_program();

    Ok(())
}