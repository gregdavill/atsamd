use super::hsv;
use super::gfx_helper;
use super::color::{Rgb,Hsv};
use micromath::F32Ext;

pub fn draw(buffer: &mut [u8]){
    static mut offset: f32 = 0.0;
    
    let mut step: f32 = 0.0;
    unsafe{
        step = offset as f32;
        offset += 0.2;
    }

    let (panel_x,panel_y) = gfx_helper::matrix_size();
    let mut hue: u8 = 0;

    for x in 0..panel_x as usize {
        let a:f32 = (((x as f32 *step)/(11.0*3.141)) * 0.04);

        hue = (step + (37_f32 * a.sin()) as f32 )as u8;
        for y in 0..panel_y as usize {
            hue += (17.0 * (y as f32 / (5.0*3.141))) as u8;
            let b: f32 = (hue as f32 + step);
            let color: Rgb = hsv::hsv2rgb(Hsv{h: (hue + ((step as u32) & 0x000000FF) as u8), s: 255, v: (b.sin()*(255.0*3.141*0.003891)) as u8 });
			gfx_helper::set_pixel(buffer, x, y, &color);
        }
    }
}