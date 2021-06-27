extern crate piston_window;
use piston_window::*;

pub struct Display {
  window: PistonWindow
}

impl Display {
  pub fn new() -> Self {
    let window_settings = WindowSettings::new("RustBoy", [160 * 3, 144 * 3])
      .exit_on_esc(true);
    let window: PistonWindow = window_settings.build().unwrap();
    Display { window: window, }
  }

  pub fn set_title(&self, title: &str) {
    self.window.window.ctx.window().set_title(title);
  }

  pub fn poll(&mut self) -> Option<Event> {
    self.window.next()
  }

  pub fn refresh(&mut self, e: &Event) {
 		self.window.draw_2d(e, |c, g, _| {
		  clear([0.0, 0.0, 0.0, 0.0], g);
      rectangle([1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0], // rectangle
                 c.transform, g);
    });
  }
}
