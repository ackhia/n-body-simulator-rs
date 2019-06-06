use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Partical {
  x: f64,
  y: f64,
  velocity_x: f64,
  velocity_y: f64,
  mass: f64
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    particals: Vec<Partical>
}

fn same_object<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        clear(GREEN, &mut self.gl);

        for p in &self.particals {
          self.gl.draw(args.viewport(), |c, gl| {
              let transform = c.transform.trans(p.x, p.y);

              // Draw a box rotating around the middle of the screen.
              rectangle(RED, square, transform, gl);
          });
        }
    }

    fn distance(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
      (((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)) as f64).sqrt()
    }

    fn update(&mut self, args: &UpdateArgs) {
        /*let mut a_x: f64;
        let mut a_y: f64;
        for mut sp in &mut self.particals {
          a_x = 0.0;
          a_y = 0.0;
          for p in &self.particals {
            if same_object(sp, p) == false {
              a_x += p.mass * (sp.x - p.x) / self.distance(sp.x, p.x, sp.y, p.y);
              a_y += p.mass * (sp.y - p.y) / self.distance(sp.x, p.x, sp.y, p.y);
            }
          }
          sp.x += a_x * args.dt;
          sp.y += a_y * args.dt;
        }*/
    }
}

fn build_app(gl: GlGraphics) -> App {
  let mut app = App {
    gl,
    rotation: 0.0,
    particals: Vec::new()
  };

  app.particals.push(Partical {
    x: 10.0,
    y: 10.0,
    velocity_x: 0.0,
    velocity_y: 0.0,
    mass: 4.0
  });

  app.particals.push(Partical {
    x: 740.0,
    y: 740.0,
    velocity_x: 0.0,
    velocity_y: 0.0,
    mass: 4.0
  });
  app
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "n-body-simulator-rs",
            [800, 800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = build_app(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}