use super::gfx_helper;
use super::color::{Rgb,Hsv};
use super::hsv;

use micromath::F32Ext;

pub fn draw(buffer: &mut [u8]){
    static mut offset: u16 = 0;
    static mut led_sweep: u16 = 0;
    
    let mut o = 0u16;
    let mut j = 0u16;
    unsafe{
        o = offset as u16;
        offset += 1;

        led_sweep += 1;
        if led_sweep == 16{
            led_sweep = 0;
        }
        j = (((offset as f32) / 8.0).sin() * 8.0 + 8.5) as u16;
    }

    let (panel_x,panel_y) = gfx_helper::matrix_size();

    for y in 0..panel_y as usize {
        for x in 0..panel_x as usize {


            //let color: Rgb = hsv::hsv2rgb(Hsv{h: (o as u8 + (x/4) as u8), s: 255, v: 128});
			//gfx_helper::set_pixel(buffer, x, y, &color);

            let mut r: u8 = 0;//(x as u16 * 256u16 / panel_x) as u8;
            let mut g: u8 = 0;//(256u16 - (y as u16 * 256u16 / panel_y)) as u8;
            let mut b: u8 = 0;
            if j == y as u16 % 16  || 
               j == x as u16 % 16  ||
               j == 15 - (y+x) as u16 % 16 {
                let color: Rgb = hsv::hsv2rgb(Hsv{h: (128 + ((o) as u8 + (x/4) as u8) % 255), s: 255, v: 128});
			    gfx_helper::set_pixel(buffer, x, y, &color);
            }
            else {
                let color: Rgb = hsv::hsv2rgb(Hsv{h: ((o) as u8), s: 255, v: 32});
			    gfx_helper::set_pixel(buffer, x, y, &color);
            }



        }
    }
}

