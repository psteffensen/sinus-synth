extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
//extern crate portaudio as pa; // For audio I/O

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    wave: f64,
    t: f64,
    pi: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        //let square = rectangle::square(0.0, 0.0, 50.0);
        let testline = Line::new(BLACK, 2.0);
        let rotation = self.rotation;
        let wave = self.wave;
        let pi = self.pi;
        let (x, y) = (args.width / 2.0, args.height / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);
            
            let line_width = 200.0_f64;
            
            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-line_width/2.0, 0.0);


            //// Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);
            let mut hline; // x, y, x2, y2
            let length = 7;
            let mut seg = vec![0_f64; length];
            for i in 0..length{
                seg[i] = i as f64; //let seg = [-3_f64, -2_f64, -1_f64, 0_f64, 1_f64, 2_f64, 3_f64]; // normalized segments of line
            }
            
            let mut amp = vec![0_f64; length];
            let length_norm = pi / ((length-1) as f64); //length normalized to pi
            for i in 0..length{
                amp[i] = (i as f64 *length_norm).sin(); //let amp = [0_f64, 0.382_f64, 0.707_f64, 1_f64, 0.707_f64, 0.382_f64, 0_f64]; // amplitude of segments
            }

            // Draw line from seg and amp
            for item in 0..length {
                if item < length-1 {
                    hline = [(line_width/((length as f64)-1.0 ))*seg[item], wave*amp[item].abs(), (line_width/((length as f64)-1.0))*seg[item+1], wave*amp[item+1].abs()];
                    testline.draw(hline, &c.draw_state, transform, gl);
                }
            }
            
        });

    }


    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 0.1 * args.dt;
        self.t += 0.3*self.pi;
        if self.t > 2.0*self.pi {
            self.t = self.t % (self.pi*2.0);
        }
        self.wave = self.t.sin()*10.0;
        //println!("{}", self.wave.to_string()); 
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "sinus-synth", [400, 400]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        wave: 0.0,
        t: 0.0,
        pi: std::f64::consts::PI
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            //std::thread::sleep(std::time::Duration::from_millis(10));
            app.update(&u);
        }
    }
}
