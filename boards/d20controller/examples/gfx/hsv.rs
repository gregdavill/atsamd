use super::color::{Rgb, Hsv};

pub fn hsv2rgb(hsv: Hsv) -> Rgb
{
    let mut rgb: Rgb = Default::default();
    let region: u8;
    let remainder: u8;
    let p: u8;
    let q: u8;
    let t: u8;

    if (hsv.s == 0)
    {
        rgb.r = hsv.v;
        rgb.g = hsv.v;
        rgb.b = hsv.v;
        return rgb;
    }

    region = hsv.h / 43;
    remainder = (hsv.h - (region * 43)) * 6;

    p = ((hsv.v as u16 * (255 - hsv.s) as u16) as u16 >> 8) as u8;
    q = ((hsv.v as u16 * (255 - ((hsv.s as u16 * remainder as u16) as u16 >> 8) as u16)) as u16 >> 8) as u16 as u8;
    t = ((hsv.v as u16 * (255 - ((hsv.s as u16 * (255 - remainder) as u16) as u16 >> 8))) as u16 >> 8) as u16 as u8;

    match region
    {
        0 => {
            rgb.r = hsv.v; 
            rgb.g = t;
            rgb.b = p; 
            },
        1 => {
            rgb.r = q;
						rgb.g = hsv.v;
						rgb.b = p;
            },
        2 => {
            rgb.r = p;
						rgb.g = hsv.v;
						rgb.b = t;
        },
        3 => {
            rgb.r = p;
						rgb.g = q;
						rgb.b = hsv.v;
        },
        4 => {
            rgb.r = t;
						rgb.g = p;
						rgb.b = hsv.v;
            },
        _ => {
            rgb.r = hsv.v;
						rgb.g = p;
						rgb.b = q;
        }
    }

    return rgb;
}
