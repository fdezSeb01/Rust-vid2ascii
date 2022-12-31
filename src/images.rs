use image::{ImageError, io::Reader, GrayImage, GenericImageView};

#[derive(Debug)]
pub enum ImageDataError {
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
}

pub fn find_image_from_path(path: String) -> Result<(u32,u32,GrayImage), ImageDataError>{
    match Reader::open(&path) { //hace match de si la promesa de result es Ok prosigue, si es error manda el error
        Ok(image_reader)=> {
            match image_reader.format() { //como format regresa un Option (Some o none), hace el caso de que si regrese algo o else si no
                Some(_d) => {
                    match image_reader.decode() { //otra vez un match para que regrese algo si es Ok o error si no
                        Ok(image) => Ok((image.dimensions().0,image.dimensions().1,image.to_luma8())), //la regresa como escala de grises
                        Err(e) => Err(ImageDataError::UnableToDecodeImage(e))
                    }
                },
                None => Err(ImageDataError::UnableToFormatImage(path))
            }
        },
        Err(e) => Err(ImageDataError::UnableToReadImageFromPath(e))
    }
    //regresa una tupla con la imagen y su formato si todo sale bien o un error si no
}


pub fn to_ascii(pic: GrayImage, density: &[char], width: u16, height: u16) -> Result<(), ImageDataError>{
    // println!("{}", pic[(5,5)].0.get(0).unwrap());
    for i in 0..height{
        for j in 0..width{
            let index = get_index(*pic[(j as u32,i as u32)].0.get(0).unwrap());
            let pix: char = density[index];
            print!("{} ",pix);
        }
        println!("");
    }

    Ok(())
}

pub fn get_index(value: u8) -> usize {
    //necesito regresar un valor entre 0-15
    //dados valores entre 0-255
    let index: usize;
    index = ((value as f32/255 as f32) as f32 * 19.00) as usize;

    index
}