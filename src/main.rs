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
            if line.starts_with('#')
            {
                
            }
            else 
            {
                let params: Vec<&str> = line.split(',').collect();
                for param in params {
                    let param_split: Vec<&str> = param[].split(':').collect();
                    match param_split[0].as_ref() {
                        "tone" => { tone[0][0] = param_split[1].trim().parse::<f64>().unwrap(); // number. Trim takes away white spaces, parse parses to f64 and unwrap unwraps from Ok(1) to 1
                                    tone[0][1] = param_split[3].trim().parse::<f64>().unwrap(); // amp
                                    tone[0][2] = param_split[5].trim().parse::<f64>().unwrap(); // freq
                                    tone[0][3] = param_split[7].trim().parse::<f64>().unwrap(); // x
                                    tone[0][4] = param_split[9].trim().parse::<f64>().unwrap(); // y
                                    tone[0][5] = param_split[11].trim().parse::<f64>().unwrap(); // rotate
                                    println!("{:?}", tone[0]);
                                    },
                        _ => { println!("{}","Empty") }
                    }
                }
            }
        }
        Ok(contents)
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
