use super::hsv;
use super::gfx_helper;
use super::color::{Rgb,Hsv};
use micromath::F32Ext;

pub fn draw(buffer: &mut [u8], imu_x: i16, imu_y: i16, imu_z: i16){
    static mut offset: f32 = 0.0;
    
    let mut o = 0u16;
    unsafe{
        o = offset as u16;
        
    }

    static mut value_hold: u32 = 0;
    static mut last_imu_y: i16 = 0;
    
    let mut v = 0u8;
    unsafe{
        v = (value_hold as u8);

        let mut delta: i16 = (imu_y - last_imu_y) as i16;

        if((delta > 4000) || (delta < -4000)){
            delta = 0;
        }
        value_hold = ((value_hold as i32) + (delta) as i32 )as u32;

        last_imu_y = imu_y;
        
        offset += (imu_x as f32) / 2000.0;

        if(offset < 0.0){            
            offset = 32000.0;
        }
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
