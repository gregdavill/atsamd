use super::gfx_helper;
use super::color::Rgb;
use micromath::F32Ext;

extern crate nalgebra as na;
use na::{Rotation3,Matrix3};

/*** management variables ***/

static mut modno: u32 = 0;
static mut frame:u32 = 0;

/*** matrix info (initialized in init()) ***/

static mut mx: u32 = 0;		// matrix size
static mut my: u32 = 0;		// matrix size

static mut mx2: u32 = 0;
static mut my2: u32 = 0;		// matrix half size
static mut outputscale: f32 = 0;	// matrix output scale factor

/*** base effect coefficients. This is where you want to play around. ***/

// how many run variables?
const runvar_count:u32 = 16;

// run variable increments per frame
static runinc : [f32; runvar_count as usize] = [
  0.00123,   0.00651,  0.000471,  0.000973,
  0.000223,  0.000751,  0.000879,  0.000443,
  0.000373,  0.000459,  0.000321,  0.000247,
  0.000923,  0.00253,  0.00173,  0.000613
];

static M_PI:f32 = 3.14159;

// variable overflow limits (basically the modulus of the ring the run variable runs in)
static runmod: [f32; runvar_count as usize] = [
  1.0,      2.0*M_PI,     2.0*M_PI,     2.0*M_PI,
  2.0*M_PI,     2.0*M_PI,     2.0*M_PI,     2.0*M_PI,
  2.0*M_PI,     2.0*M_PI,     2.0*M_PI,     2.0*M_PI,
  2.0*M_PI,     2.0*M_PI,     2.0*M_PI,     2.0*M_PI
];

// the actual run variables
static mut runvar: [f32; runvar_count as usize] = [
  0.0,        0.0,        0.0,        0.0,
  0.0,        0.0,        0.0,        0.0,
  0.0,        0.0,        0.0,        0.0,
  0.0,        0.0,        0.0,        0.0
];

/* helper function: add on a ring
 */
fn addmod(x: f32, _mod: f32, delta: f32) -> f32 {
	//x = fmodf(x + delta, mod);
    x = (x+delta).rem_euclid(_mod);
	//x += x<0 ? mod : 0;
    x += if x < 0.0 {_mod} else {0.0};
	x
}


pub fn init(){
	unsafe {
		(mx,my) = gfx_helper::matrix_size();

		mx2 = mx/2;
		my2 = my/2;
		
		// scaling function thanks to @BenBE1987 on Twitter: https://twitter.com/BenBE1987/status/1003787341926985728
		outputscale = 1.5 * 2.powi(-( (mx.log2() - 3) + (if mx.log2() < 7 {0.5} else {0}) * (7 - mx.log2())));


		
		modno = moduleno;
		let d:u64 = 0xFa383240_192541ceu64;
		for i in runvar_count {
			runvar[i] = addmod(runvar[i as usize], runmod[i as usize], ((d>>(i/2)) & 0x00FF) / (255.0/M_PI));
		}
	}
}

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

            let r: u8 = (x as u16 * 256u16 / panel_x) as u8;
            let g: u8 = (256u16 - (y as u16 * 256u16 / panel_y)) as u8;
            let mut b: u8 = 0;
            if o % 16 == y as u16 % 16 {
                b = 255;
            }
            if o % 16 == x as u16 % 16 {
                b = 255;
            }


            let color: Rgb = Rgb {r: r, g: g, b: b};

            gfx_helper::set_pixel(buffer, x, y, color);
        }
    }


    let m: Matrix3 = Rotation3(runvar[12].cos * M_PI) * ;

	// compose transformation matrix out of 9 input matrices 
	// which are calculated from some of the run variables
	matrix3_3 m = composem3( 9,
		rotation3(cosf(runvar[12]) * M_PI),
		translation3(cosf(runvar[2])*mx*0.125, sinf(runvar[3])*my*0.125),
		scale3(outputscale, outputscale),
		rotation3(runvar[13]),
		translation3(sinf(runvar[4])*mx*0.25, cosf(runvar[5])*my*0.25),
		rotation3(sin(runvar[14]) * M_PI),
		translation3(sinf(runvar[6])*mx*0.125, cosf(runvar[7])*my*0.125),
		rotation3(runvar[15]),
		scale3(0.25+sinf(runvar[8])/4.0, 0.25+cosf(runvar[9])/4.0)
	);

	// pre-calculate some variables outside the loop
	f32 pc1 = cosf(runvar[1]);
	f32 pc121 = 0.125+((pc1/4) * sinf(runvar[11]));
	f32 pc01 = runvar[0] + pc1;
	f32 pc10 = (mx2*sinf(runvar[10]));

	// actual pixel loop
	for( int x = 0; x < mx; x++ ) {
		vec2 kernel_x = multm3v2_partx(m, x-(mx2));
		for( int y = 0; y < my; y++ ) {

			// transform x,y coordinates by the pre-composed matrix
			vec2 v = multm3v2_partxy(m, kernel_x, y-(my2));

			// calculate sine curve point
			f32 sc = sinestuff(v.x, v.y, pc10, runvar[11]);

			// add changing base hue to sine curve point
			f32 hue = pc01 + (sc * 0.5);

			// calculate byte value of HSV f32 value
			byte b_val = _min(255,(int)(_abs(sc)*512));

			// calculate byte value of HSV f32 hue ( [0.0..1.0] -> [0..255], overflows are intended! )
			byte b_hue = ((int)(hue*256) & 0xFF);

			// convert HSV to RGB
			RGB color = HSV2RGB(HSV( b_hue, 255, b_val ));

			// set pixel in matrix framebuffer
			matrix_set(x,y, color);
		}
	}
	increment_runvars();
}


/* The "canvas" function.
 */
fn sinestuff(x: f32, y: f32, v0: f32, v1: f32) -> f32 {
   (v1+x).cos() * (v1+y).sin() * (v0 + (x*x + y*y).sqrtf()).cos() 
}


/* increment all run variables while taking care of overflow
 */
fn increment_runvars() {
  for i in 0..runvar_count {
    runvar[i] = addmod(runvar[i], runmod[i], runinc[i]);
  }
}

/* helper function: returns the absolute value of a f32
 */
fn _abs(x: f32) -> f32 {
  return if x < 0.0 {-x} else {x};
}

/* helper function: returns the minimum of two ints
 */
fn _min(x: u32, y: u32) -> u32{
	return if x>y {y} else {x};
}