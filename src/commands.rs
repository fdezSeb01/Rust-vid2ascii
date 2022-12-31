/* FFMPEG COMMANDS
trim video to 5 secs: ffmpeg -i input.mp4 -ss 0 -t 5 -c:v libx264 -c:a aac output.mp4
vid to 6 fps: ffmpeg -i input.mp4 -r 6 -vsync cfr output.mp4
resize to 100 pix height: ffmpeg -i input.mp4 -vf "scale=-1:100" output.mp4
resize to 100 pix width: ffmpeg -i input.mp4 -vf "scale=100:-1" output.mp4
vid to png sequence: ffmpeg -i resized.mp4  -q:v 2 "catVidSqc/%05d.png"
delete file if exists:  rm -f hoa.mp4

final prompt: mkdir ImgSqc138fbf84uw4du844edf && ffmpeg -i input.mp4 -vf "scale=-1:100,fps=6" -q:v 2 "ImgSqc138fbf84uw4du844edf/%05d.png"

*/
use std::process::Command;
use terminal_size::{Width, Height, terminal_size};



pub fn prepare_vid(path: &String){
    //creates a folder to store the frames and converts the video into a 100 pix height and 6ps to img sequence"scale=-1:100,fps=6"
    let h = new_height(vid_dimensions(path));
    let vf_arg = "scale=-1:".to_string() + &h.to_string() + ",fps=6";

    let _output = Command::new("rm")
        .arg("-f")
        .arg("-r")
        .arg("ImgSqc138fbf84uw4du844edf")
        .output()
        .expect("Can't delete folder");

    let _output = Command::new("mkdir")
        .arg("ImgSqc138fbf84uw4du844edf")
        .output()
        .expect("Failed to execute mkdir command");

    let _output = Command::new("ffmpeg")
        .arg("-i")
        .arg(path)
        .arg("-vf")
        .arg(vf_arg)
        .arg("-q:v")
        .arg("2")
        .arg("ImgSqc138fbf84uw4du844edf/%05d.png")
        .output()
        .expect("Failed to execute ffmpeg command");
}

pub fn vid_dimensions(path: &String)-> (u16,u16){
    let mut width=0;
    let mut height=0;

    let output = Command::new("ffprobe")
        .arg("-i")
        .arg(path)
        .arg("-show_streams")
        .output()
        .expect("Failed to execute ffprobe command");

    // Convert the output to a string
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Split the output string into lines
    let lines: Vec<&str> = output_str.split('\n').collect();

    // Iterate over the lines
    for line in lines {
        // Check if the line starts with "width="
        if line.starts_with("width=") {
            // Split the line into fields
            let fields: Vec<&str> = line.split('=').collect();

            // Get the second field (which contains the width)
            // width = fields[1];
            width = fields[1].parse::<u16>().unwrap();
        }
        // Check if the line starts with "height="
        if line.starts_with("height=") {
            // Split the line into fields
            let fields: Vec<&str> = line.split('=').collect();

            // Get the second field (which contains the height)
            height = fields[1].parse::<u16>().unwrap();
        }
    }
    (width,height)
}

pub fn new_height((w,h): (u16, u16)) -> u8{
    let size = terminal_size();
    if let Some((Width(wt), Height(ht))) = size {
        if h<=ht && w <=wt{
            h as u8
        }
        else if h > ht {
            let w = (w as f32 * (ht as f32/h as f32)) as u16;
            if w <= wt{
                ht as u8
            } else {
                (ht as f32 * (wt as f32/w as f32)) as u8
            }
        } else {
            (h as f32 * (wt as f32/w as f32)) as u8
        }
    } else {
        80
    }
}

pub fn end_program(){
    let _output = Command::new("rm")
        .arg("-f")
        .arg("-r")
        .arg("ImgSqc138fbf84uw4du844edf")
        .output()
        .expect("Can't delete folder");
}