use super::hsv;
use super::gfx_helper;
use super::color::{Rgb,Hsv};

pub fn draw(buffer: &mut [u8]){
    static mut offset: u8 = 0;
    
    let mut o = 0u16;
    unsafe{
        o = offset as u16;
        offset += 1;
    }

    let (panel_x,panel_y) = gfx_helper::matrix_size();


    for y in 0..panel_y as usize {
        for x in 0..panel_x as usize {
            let color: Rgb = hsv::hsv2rgb(Hsv{h: (o as u8 + (x/4) as u8), s: 255, v: 128});
			gfx_helper::set_pixel(buffer, x, y, &color);
        }
    }
}
