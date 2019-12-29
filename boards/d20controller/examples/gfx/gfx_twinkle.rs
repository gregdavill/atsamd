// "She sat down on the balcony of her tower, watching the stars sparkle."
// Contributed by 20kdc. First non-test/stupid module!!

use super::hsv;
use super::gfx_helper;
use super::color::{Rgb,Hsv};
use super::rand::Rand;


const TWINKLE_LEVELS: u32 = 16;
//const TWINKLE_FRAMETIME: u32 = (50 * T_MILLISECOND);
//const TWINKLE_FRAMES: u32 =  (TIME_LONG * 20);

const fn TWINKLE_COL(v: u8) -> Rgb {
    Rgb{r:((v) * 3) / 4,g: ((v) * 3) / 4,b: (v)}
}

const twinkle_level_tab: [Rgb; TWINKLE_LEVELS as usize]  = [
    TWINKLE_COL(0),
	TWINKLE_COL(16),
	TWINKLE_COL(32),
	TWINKLE_COL(116),
	TWINKLE_COL(192),
	TWINKLE_COL(208),
	TWINKLE_COL(224),
	TWINKLE_COL(240),
	TWINKLE_COL(255),
	TWINKLE_COL(224),
	TWINKLE_COL(192),
	TWINKLE_COL(160),
	TWINKLE_COL(128),
	TWINKLE_COL(96),
	TWINKLE_COL(64),
	TWINKLE_COL(32)
];

const BUFF_LENGTH: usize = 32*3*24*2;
static mut twinkle_levels: [u8 ;BUFF_LENGTH] = [0; BUFF_LENGTH];


static mut twinkle_moduleno: u32 = 0;
static mut twinkle_nexttick: u32 = 0;
static mut twinkle_framecount: u32 = 0;


static mut rng: Rand = Rand::new(0);


pub fn draw(buffer: &mut [u8]){

    let (panel_x,panel_y) = gfx_helper::matrix_size();
     

    let mut i: usize = 0;
    for x in 0..panel_x as usize {
        let mut lineactivity: u32 = 0;
        for y in 0..panel_y as usize {
            unsafe{
                if twinkle_levels[i] == 0 {
                    if ((rng.rand() & 511) == 0){
						twinkle_levels[i] = 1;
                    }
                }else {
                    lineactivity += 1;
                    twinkle_levels[i] += 1;
                    twinkle_levels[i] %= TWINKLE_LEVELS as u8;
                }
                //let color: Rgb = Rgb{r:3, g:2, b:6};

                gfx_helper::set_pixel(buffer, x, y, &twinkle_level_tab[twinkle_levels[i] as usize]);
                
                i += 1;
            }
        }
    }

    i = 0;
    for y in 0..(panel_y - 1) as usize {
        for x in 0..(panel_x) as usize {
            unsafe {
                twinkle_levels[i] = twinkle_levels[i + 24*2];
                i += 1;
            }
        }
    }

    for x in 0..(panel_x) as usize {
            unsafe {
                twinkle_levels[i] = 0;
                i += 1;
            }
        }
}
