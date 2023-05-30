use embedded_graphics::prelude::*;
use chrono::prelude::*;

use display::Display;

mod display;

fn main() -> ()  {

    let mut dpy = Display::new();

    dpy.render_text("Cpu 10%", Point::new(72, 0));
    dpy.render_rect(Point::new(0, 40), Size::new(127, 1));
    dpy.target.flush().unwrap();

    loop {
        let now = Local::now();
        dpy.render_text(&format!("{:0>2}:{:0>2}", now.hour(), now.minute()), Point::new(0, 0));
        dpy.target.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
