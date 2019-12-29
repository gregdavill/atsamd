use super::color::Rgb;

pub fn set_pixel(buffer: &mut [u8], 
            x: usize, y: usize,
            color: &Rgb){

    let (px, py) = matrix_size();

    buffer[y*(px as usize )*3 + x*3    ] = color.r;
    buffer[y*(px as usize )*3 + x*3 + 1] = color.g;
    buffer[y*(px as usize )*3 + x*3 + 2] = color.b; 
}


pub fn matrix_size() -> (u16, u16) {
    super::super::matrix_size()
}