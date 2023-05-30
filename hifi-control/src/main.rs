use std::error::Error;

use embedded_graphics::prelude::*;

use display::Display;

mod display;

fn main() -> Result<(), Box<dyn Error>>  {

    let mut dpy = Display::new();

    dpy.render_text("13:48", Point::new(0, 0));
    dpy.render_text("Cpu 10%", Point::new(72, 0));
    dpy.render_rect(Point::new(0, 40), Size::new(127, 1));
    dpy.target.flush().unwrap();

    Ok(())
}
