use super::hsv;
use super::gfx_helper;
use super::color::{Rgb,Hsv};
use micromath::F32Ext;

pub fn draw(buffer: &mut [u8]){
    static mut offset: u16 = 0;
    
    let mut o = 0u16;
    unsafe{
        o = offset as u16;
        offset += 1;
    }

    let (panel_x,panel_y) = gfx_helper::matrix_size();


    for y in 0..panel_y as usize {
        for x in 0..panel_x as usize {
            let shift : f32 = (o as f32 + x as f32*(o as f32/150.0).sin()*2.0+2.1 + y as f32*(o as f32/200.0).cos()*1.0+1.1) / 40.0;

            let color: Rgb = hsv::hsv2rgb(Hsv{h: (shift.sin() * 126.0 + 129.0) as u8, s: 255, v: ((shift * 16.0).sin()*64.0) as u8 + 65});
			gfx_helper::set_pixel(buffer, x, y, &color);
        }
    }
}
