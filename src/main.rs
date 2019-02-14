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
    sound: [[f64; 4];64]
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
        let sound = self.sound;

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
            let file_name = "sound.sin";
            App::open_sound("sound_test.sin");
            App::save_sound(file_name);
        });
    }
    
    fn open_sound(file_name: &str) -> sound { 
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        for line in contests.split("\n") {
            println!(line.to_string());
        }
        Ok(())
    }   
    
    fn save_sound(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(b"#Settings for sound\n")?;
        for tone in self.sound {
            file.write_all(b"pitch: {}, x: {}, y: {}, rotate: {}", tone.pitch, tone.x, tone.y, tone.rotate)?;
        }
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
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
            //std::thread::sleep(std::time::Duration::from_millis(10));
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
