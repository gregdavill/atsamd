use super::hsv;
use super::gfx_helper;
use super::color::{Rgb,Hsv};
use micromath::F32Ext;

pub fn draw(buffer: &mut [u8], imu_x: i16, imu_y: i16, imu_z: i16){
    static mut offset: u32 = 0;

    static mut last_imu: i16 = 0;
    
    let mut c = 0u8;
    unsafe{
        c = (offset as u8) / 4;

        let mut delta: i16 = (imu_x - last_imu) as i16;

        if((delta > 4000) || (delta < -4000)){
            delta = 0;
        }
        offset = ((offset as i32) + (delta / 8) as i32) as u32;

        last_imu = imu_x;
    }

    static mut value_hold: u32 = 0;
    static mut last_imu_y: i16 = 0;
    
    let mut v = 0u8;
    unsafe{
        c = (value_hold as u8) / 4;

        let mut delta: i16 = (imu_y - last_imu_y) as i16;

        if((delta > 4000) || (delta < -4000)){
            delta = 0;
        }
        value_hold = ((value_hold as i32) + (delta / 8) as i32 )as u32;

        last_imu_y = imu_y;
    }

    let (panel_x,panel_y) = gfx_helper::matrix_size();


    for y in 0..panel_y as usize {
        for x in 0..panel_x as usize {
            //let shift : f32 = (o as f32 + x as f32*(o as f32/(imu_x as f32)).sin()*4.0+4.1 + y as f32*(o as f32/(imu_z as f32)).cos()*4.0+4.1) / 40.0;
            let color: Rgb = hsv::hsv2rgb(Hsv{h: ((imu_x / 128) as f32 * ((panel_x as f32).sin() + 0.5)) as u8, s: 180, v: (imu_z / 128) as u8 | 8});
			gfx_helper::set_pixel(buffer, x, y, &color);
        }
    }
}
