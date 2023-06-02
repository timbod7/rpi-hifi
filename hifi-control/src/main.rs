use std::{fs::File, io::{BufReader,BufRead}};

use embedded_graphics::prelude::*;
use chrono::prelude::*;

use display::Display;
use audio::AudioView;

mod display;
mod audio;

fn main() -> ()  {

    let mut cpu_usage = CpuUsage::new();
    let mut dpy = Display::new();
    let mut audio = AudioView::new();

    loop {
        let now = Local::now();
        let (llevel,rlevel) = audio.read_average(4410);

        let lx = (llevel * 128 / 32768) as u32;
        let rx = (rlevel * 128 / 32768) as u32;

        println!("llevel: {}, lx : {}", llevel, lx);

        dpy.render_text(&format!("{:0>2}:{:0>2}", now.hour(), now.minute()), Point::new(0, 0));
        dpy.render_text(&format!("Cpu {: >2}%", cpu_usage.get_percent()), Point::new(72, 0));
        
        dpy.clear_rect(Point::new(0, 28), Size::new(127, 4));
        dpy.render_rect(Point::new(0, 28), Size::new( lx, 4));

        dpy.clear_rect(Point::new(0, 48), Size::new(127, 4));
        dpy.render_rect(Point::new(0, 48), Size::new( rx, 4));

        dpy.target.flush().unwrap();
    }
}


struct CpuUsage {
    last: Option<(u32, u32)>,
    percents: Vec<u32>,
    percent_i: usize,
}

impl CpuUsage {
    pub fn new() -> Self {
        CpuUsage{
            last: None,
            percents: vec![0; AVG_SIZE],
            percent_i: 0,
        }
    }

    pub fn get_percent(&mut self) -> u8 {

        let file = File::open("/proc/stat").unwrap(); 
        let line1 = BufReader::new(file).lines().next()
            .expect("/dev/stat to be readable")
            .expect("a first line in /proc/stat");
        let times: Vec<u32> = line1.split_whitespace().into_iter()
            .skip(1)
            .map(|ts| ts.parse::<u32>().expect("cpu usage should be an int"))
            .collect();
        let total: u32 = times.iter().sum();
        let idle = times[3];

        let percent = if let Some((last_idle,last_total)) = self.last {
            let didle = idle - last_idle;
            let dtotal = total - last_total;
            let percent = (dtotal - didle) * 100 / dtotal;
            self.percents[self.percent_i] = percent;
            self.percent_i = (self.percent_i + 1) % AVG_SIZE;
            self.percents.iter().sum::<u32>() / (AVG_SIZE as u32)
        } else {
            0
        };
        self.last = Some((idle,total));

        percent as u8
    }
}

const AVG_SIZE:usize = 20;
