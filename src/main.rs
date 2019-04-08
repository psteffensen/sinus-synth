extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
//extern crate portaudio as pa; // For audio I/O

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use graphics::*;
use std::fs::File;  // for opening and saving file
use std::io::prelude::*; // for opening and saving file 

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    wave: f64,
    t: f64,
    pi: f64,
    sound: [[f64; 4];64],
}

enum Pitch {
    c0, cs0, df0, d0, ds0, ef0, e0, f0, fs0, gf0, g0, gs0, af0, a0, as0, bf0, b0, 
    c1, cs1, df1, d1, ds1, ef1, e1, f1, fs1, gf1, g1, gs1, af1, a1, as1, bf1, b1, 
    c2, cs2, df2, d2, ds2, ef2, e2, f2, fs2, gf2, g2, gs2, af2, a2, as2, bf2, b2, 
    c3, cs3, df3, d3, ds3, ef3, e3, f3, fs3, gf3, g3, gs3, af3, a3, as3, bf3, b3, 
    c4, cs4, df4, d4, ds4, ef4, e4, f4, fs4, gf4, g4, gs4, af4, a4, as4, bf4, b4, 
    c5, cs5, df5, d5, ds5, ef5, e5, f5, fs5, gf5, g5, gs5, af5, a5, as5, bf5, b5, 
    c6, cs6, df6, d6, ds6, ef6, e6, f6, fs6, gf6, g6, gs6, af6, a6, as6, bf6, b6, 
    c7, cs7, df7, d7, ds7, ef7, e7, f7, fs7, gf7, g7, gs7, af7, a7, as7, bf7, b7, 
    c8, cs8, df8, d8, ds8, ef8, e8, f8, fs8, gf8, g8, gs8, af8, a8, as8, bf8, b8, 
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        let rotation = self.rotation;
        let (x, y) = (args.width / 2.0, args.height / 2.0);
        let pitch = 200_f64;
        let pos_x = 0_u32;
        let pos_y = 0_u32;
        let pos_rotate = 0_u32;
        let wave = self.wave;
        let pi = self.pi;
        let line_width = 200_f64;
        //let contents = self.contents;
        //let contents = std::io::Result<String>;

        let file_name = "sound.sin";
        let contents = App::open_sound("sound_test.sin");
        //App::save_sound(file_name);
 
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            // Rotate the vibrating lines
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-line_width / 2.0, 0.0);

            Virtualization::vibrating_line(&c, gl, transform, wave, pi, pitch, pos_x, pos_y, pos_rotate);
       });
    }

    fn open_sound(file_name: &str) -> std::io::Result<(String)> { 
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut tone: [[f64; 6]; 128] = [[0.0; 6]; 128];
        let mut adsr: [[f64; 4]; 1];
        let mut lfo: [f64; 2];
        let mut seq: [[f64; 2]; 2048];


        for line in contents.split("\r\n") {
            let mut i  = 1 as usize;
            if line.starts_with('#')
            {
                
            }
            else 
            {
                let params: Vec<&str> = line.split(|x| (x == ',') || (x == ':')).collect();
                dbg!(&params);
                match params[0].as_ref() {
                    "tone" => { i = params[1].trim().parse::<usize>().unwrap(); // number. Trim takes away white spaces, parse parses to f64 and unwrap unwraps from Ok(1) to 1
                                dbg!(i);
                                tone[i][0] = i as f64;                                  // Tone number
                                println!("i={} tone={:?}", i, tone[i][0]);
                                tone[i][1] = params[3].trim().parse::<f64>().unwrap();  // Amplitude
                                tone[i][2] = params[5].trim().parse::<f64>().unwrap();  // Pitch
                                tone[i][3] = {
                                                if tone[i][2] == 0 as f64 { 
                                                    params[7].trim().parse::<f64>().unwrap()
                                                }
                                                else {
                                                    App::pitch2freq(params[7].trim())
                                                }
                                            }; //Pitch, fx. "c2"
                                tone[i][4] = params[9].trim().parse::<f64>().unwrap();  // Pan
                                tone[i][5] = params[11].trim().parse::<f64>().unwrap();  // Phase
                                println!("tone {} = {:?}", i, tone[i][0]);
                                },
                    "lfo"  => {println!("lfo {}", i)},
                    _ => { println!("{}","Empty") }
                }
            }
        }
        Ok(contents)
    }   
    
    // https://en.wikipedia.org/wiki/Piano_key_frequencies
    fn pitch2freq(pitch: &str) -> f64 {
        match pitch {
            Pitch::c0 => 16.35160,
            Pitch::cs0 => 17.32391,
            Pitch::df0 => 17.32391,
            Pitch::d0 => 18.35405,
            Pitch::ds0 => 19.44544,
            Pitch::ef0 => 19.44544,
            Pitch::e0 => 20.60172,
            Pitch::f0 => 21.82676,
            Pitch::fs0 => 23.12465,
            Pitch::gf0 => 23.12465,
            Pitch::g0 => 24.49971,
            Pitch::gs0 => 25.95654,
            Pitch::af0 => 25.95654,
            Pitch::a0 => 27.50000, 
            Pitch::as0 => 29.13524, 
            Pitch::bf0 => 29.13524, 
            Pitch::b0 => 30.86771, 
            Pitch::c1 => 32.70320, 
            Pitch::cs1 => 34.64783, 
            Pitch::df1 => 34.64783, 
            Pitch::d1 => 36.70810, 
            Pitch::ds1 => 38.89087, 
            Pitch::ef1 => 38.89087, 
            Pitch::e1 => 41.20344, 
            Pitch::f1 => 43.65353, 
            Pitch::fs1 => 46.24930, 
            Pitch::gf1 => 46.24930, 
            Pitch::g1 => 48.99943, 
            Pitch::gs1 => 51.91309, 
            Pitch::af1 => 51.91309,
            Pitch::a1 => 55.00000, 
            Pitch::as1 => 58.27047, 
            Pitch::bf1 => 58.27047, 
            Pitch::b1 => 61.73541, 
            Pitch::c2 => 65.40639, 
            Pitch::cs2 => 69.29566, 
            Pitch::df2 => 69.29566, 
            Pitch::d2 => 73.41619, 
            Pitch::ds2 => 77.78175, 
            Pitch::ef2 => 77.78175, 
            Pitch::e2 => 82.40689, 
            Pitch::f2 => 87.30706, 
            Pitch::fs2 => 92.49861, 
            Pitch::gf2 => 92.49861, 
            Pitch::g2 => 97.99886, 
            Pitch::gs2 => 103.8262, 
            Pitch::af2 => 103.8262,
            Pitch::a2 => 110.0000, 
            Pitch::as2 => 116.5409, 
            Pitch::bf2 => 116.5409, 
            Pitch::b2 => 123.4708, 
            Pitch::c3 => 130.8128, 
            Pitch::cs3 => 138.5913, 
            Pitch::df3 => 138.5913, 
            Pitch::d3 => 146.8324, 
            Pitch::ds3 => 155.5635, 
            Pitch::ef3 => 155.5635, 
            Pitch::e3 => 164.8138, 
            Pitch::f3 => 174.6141, 
            Pitch::fs3 => 184.9972, 
            Pitch::gf3 => 184.9972, 
            Pitch::g3 => 195.9977, 
            Pitch::gs3 => 207.6523, 
            Pitch::af3 => 207.6523,
            Pitch::a3 => 220.0000, 
            Pitch::as3 => 233.0819, 
            Pitch::bf3 => 233.0819, 
            Pitch::b3 => 246.9417, 
            Pitch::c4 => 261.6256, 
            Pitch::cs4 => 277.1826, 
            Pitch::df4 => 277.1826, 
            Pitch::d4 => 293.6648, 
            Pitch::ds4 => 311.1270, 
            Pitch::ef4 => 311.1270, 
            Pitch::e4 => 329.6276, 
            Pitch::f4 => 349.2282, 
            Pitch::fs4 => 369.9944, 
            Pitch::gf4 => 369.9944, 
            Pitch::g4 => 391.9954, 
            Pitch::gs4 => 415.3047, 
            Pitch::af4 => 415.3047,
            Pitch::a4 => 440.0000, 
            Pitch::as4 => 466.1638, 
            Pitch::bf4 => 466.1638, 
            Pitch::b4 => 493.8833, 
            Pitch::c5 => 523.2511, 
            Pitch::cs5 => 554.3653, 
            Pitch::df5 => 554.3653, 
            Pitch::d5 => 587.3295, 
            Pitch::ds5 => 622.2540, 
            Pitch::ef5 => 622.2540, 
            Pitch::e5 => 659.2551, 
            Pitch::f5 => 698.4565, 
            Pitch::fs5 => 739.9888, 
            Pitch::gf5 => 739.9888, 
            Pitch::g5 => 783.9909, 
            Pitch::gs5 => 830.6094, 
            Pitch::af5 => 830.6094,
            Pitch::a5 => 880.0000, 
            Pitch::as5 => 932.3275, 
            Pitch::bf5 => 932.3275, 
            Pitch::b5 => 987.7666, 
            Pitch::c6 => 1046.502, 
            Pitch::cs6 => 1108.731, 
            Pitch::df6 => 1108.731, 
            Pitch::d6 => 1174.659, 
            Pitch::ds6 => 1244.508, 
            Pitch::ef6 => 1244.508, 
            Pitch::e6 => 1318.510, 
            Pitch::f6 => 1396.913, 
            Pitch::fs6 => 1479.978, 
            Pitch::gf6 => 1479.978, 
            Pitch::g6 => 1567.982, 
            Pitch::gs6 => 1661.219, 
            Pitch::af6 => 1661.219,
            Pitch::a6 => 1760.000, 
            Pitch::as6 => 1864.655, 
            Pitch::bf6 => 1864.655, 
            Pitch::b6 => 1975.533, 
            Pitch::c7 => 2093.005, 
            Pitch::cs7 => 2217.461, 
            Pitch::df7 => 2217.461, 
            Pitch::d7 => 2349.318, 
            Pitch::ds7 => 2489.016, 
            Pitch::ef7 => 2489.016, 
            Pitch::e7 => 2637.020, 
            Pitch::f7 => 2793.826, 
            Pitch::fs7 => 2959.955, 
            Pitch::gf7 => 2959.955, 
            Pitch::g7 => 3135.963, 
            Pitch::gs7 => 3322.438, 
            Pitch::af7 => 3322.438,
            Pitch::a7 => 3520.000, 
            Pitch::as7 => 3729.310, 
            Pitch::bf7 => 3729.310, 
            Pitch::b7 => 3951.066, 
            Pitch::c8 => 4186.009, 
            Pitch::cs8 => 4434.922, 
            Pitch::df8 => 4434.922, 
            Pitch::d8 => 4698.636, 
            Pitch::ds8 => 4978.032, 
            Pitch::ef8 => 4978.032, 
            Pitch::e8 => 5274.041, 
            Pitch::f8 => 5587.652, 
            Pitch::fs8 => 5919.911, 
            Pitch::gf8 => 5919.911, 
            Pitch::g8 => 6271.927, 
            Pitch::gs8 => 6644.875, 
            Pitch::af8 => 6644.875,
            Pitch::a8 => 7040.000,
            Pitch::as8 => 7458.620,
            Pitch::bf8 => 7458.620,
            Pitch::b8 => 7902.133,
            _ => 440.0,
        }
    }

    fn save_sound(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(b"#Settings for sound\n")?;
        /*for tone in self.sound {
            file.write_all(b"pitch: {}, x: {}, y: {}, rotate: {}", tone.pitch, tone.x, tone.y, tone.rotate)?;
        }*/
        Ok(())
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 0.1 * args.dt;
        self.t += 0.3 * self.pi;
        if self.t > 2.0 * self.pi {
            self.t = self.t % (self.pi * 2.0);
        }
        self.wave = self.t.sin() * 10.0;
        //println!("{}", self.wave.to_string());
    }
}

pub struct Virtualization {

}

impl Virtualization {

    fn vibrating_line(c: &graphics::Context, gl: &mut GlGraphics, transform: [[f64; 3];2], wave: f64, pi: f64, pitch: f64, x: u32, y: u32, rotate: u32) {

        let testline = Line::new(BLACK, 2.0);
        //// Draw a box rotating around the middle of the screen.
        //rectangle(RED, square, transform, gl);
        let mut hline; // x, y, x2, y2
        let length = 7;
        let mut seg = vec![0_f64; length];
        for i in 0..length {
            seg[i] = i as f64; //let seg = [-3_f64, -2_f64, -1_f64, 0_f64, 1_f64, 2_f64, 3_f64]; // normalized segments of line
        }

        let mut amp = vec![0_f64; length];
        let length_norm = pi / ((length - 1) as f64); //length normalized to pi
        for i in 0..length {
            amp[i] = (i as f64 * length_norm).sin(); //let amp = [0_f64, 0.382_f64, 0.707_f64, 1_f64, 0.707_f64, 0.382_f64, 0_f64]; // amplitude of segments
        }

        // Draw line from seg and amp
        for item in 0..length {
            if item < length - 1 {
                hline = [
                    (pitch / ((length as f64) - 1.0)) * seg[item],
                    wave * amp[item].abs(),
                    (pitch / ((length as f64) - 1.0)) * seg[item + 1],
                    wave * amp[item + 1].abs(),
                ];
                testline.draw(hline, &c.draw_state, transform, gl);
            }
        }
    }

}


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("sinus-synth", [400, 400])
        .opengl(opengl)
        .samples(2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        wave: 0.0,
        t: 0.0,
        pi: std::f64::consts::PI,
        sound: [[0.0; 4];64],
        //contents: std::io::Result<()>
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
