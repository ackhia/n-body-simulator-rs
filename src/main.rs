use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use rand::Rng;

const MAX_MASS: f64 = 5.0;
const MIN_MASS: f64 = 1.0;
const PARTICLE_COUNT: i32 = 400;
const TAIL_OFF: f64 = 1.8;
const INITIAL_VOLICITY_MIN: f64 = -0.1;
const INITIAL_VOLICITY_MAX: f64 = 0.1;

pub struct Partical {
  x: f64,
  y: f64,
  velocity_x: f64,
  velocity_y: f64,
  mass: f64
}

impl PartialEq for Partical {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y && 
        self.velocity_x == other.velocity_x &&
        self.velocity_y == other.velocity_y &&
        self.mass == other.mass
    }
}
impl Eq for Partical {}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    particals: Vec<Partical>
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
  (((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)) as f64).sqrt()
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        clear(BLACK, &mut self.gl);

        for p in &self.particals {
          self.gl.draw(args.viewport(), |c, gl| {
              let transform = c.transform.trans(p.x, p.y);
              let size = p.mass / MAX_MASS * 5.0;
              let circle = ellipse::centered([0.0,0.0,size,size]);
              ellipse(GREEN, circle, transform, gl);
          });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
      let drag = 0.999;
      let mut a_x: f64;
      let mut a_y: f64;
      let mut new_particals: Vec<Partical> = Vec::new();

      for sp in self.particals.iter() {
        a_x = 0.0;
        a_y = 0.0;
        for p in self.particals.iter() {
          if sp != p {
            let distance = distance(sp.x, sp.y, p.x, p.y);
            a_x += p.mass * (p.x - sp.x) / distance.powf(TAIL_OFF);
            a_y += p.mass * (p.y - sp.y) / distance.powf(TAIL_OFF);
          }
        }
        let v_x = (sp.velocity_x + (a_x * args.dt)) * drag;
        let v_y = (sp.velocity_y + (a_y * args.dt)) * drag;
        new_particals.push(Partical {
          x: sp.x + v_x,
          y: sp.y + v_y,
          velocity_x: v_x,
          velocity_y: v_y,
          mass: sp.mass
        })
      }

      self.particals = new_particals;
    }
}

fn build_app(gl: GlGraphics) -> App {
  let mut app = App {
    gl,
    particals: Vec::new()
  };

  let mut i = 0;
  while i < PARTICLE_COUNT {
    app.particals.push(Partical {
      x: rand::thread_rng().gen_range(20.0, 780.0),
      y: rand::thread_rng().gen_range(20.0, 780.0),
      velocity_x: rand::thread_rng().gen_range(INITIAL_VOLICITY_MIN, INITIAL_VOLICITY_MAX),
      velocity_y: rand::thread_rng().gen_range(INITIAL_VOLICITY_MIN, INITIAL_VOLICITY_MAX),
      mass: rand::thread_rng().gen_range(MIN_MASS, MAX_MASS)
    });

    i += 1;
  }
  app
}

fn main() {
  println!("{}", distance(10.0, 10.0, 740.0, 10.0));

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