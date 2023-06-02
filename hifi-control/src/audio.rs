extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use std::time::Instant;
use psimple::Simple;
use pulse::stream::Direction;
use pulse::sample::{Spec, Format};

pub struct AudioView {
    pa_simple: Simple,
    start: Instant,
}

impl AudioView {
    pub fn new() -> Self {
        let spec = Spec {
            format: Format::S16NE,
            channels: 2,
            rate: 44100,
        };
        assert!(spec.is_valid());
    
        let pa_simple = Simple::new(
                None,                   // Use the default server
                "HifiControl",            // Our application’s name
                Direction::Record,         // We want a playback stream
                Some("alsa_output.platform-soc_sound.stereo-fallback.monitor"),
                "HifiControl",     // Description of our stream
                &spec,                      // Our sample format
                None,                      // Use default channel map
                None                      // Use default buffering attributes
        ).map_err(|e| e.to_string()).unwrap();

        let start = Instant::now();

        AudioView{
            pa_simple,
            start
        }
    }

    // Collect some audio and measure it's average level
    pub fn read_average(&mut self, n_samples: usize) -> (i32,i32) {


        let mut buf : Vec<i16> = vec![0; n_samples*2];

        unsafe {
            let buf_u8 = buf.align_to_mut::<u8>().1;
            self.pa_simple.read(buf_u8).unwrap();
        }
        println!("{}: {}", self.start.elapsed().as_millis(), n_samples);

        let latency = self.pa_simple.get_latency().unwrap();
        println!( "latency: {} ms", latency.as_millis());
        

        let mut ltotal: i32 = 0;
        let mut rtotal: i32 = 0;

        for i in 0..buf.len()/2 {
            let left =  buf[i*2] as i32;
            let right =  buf[i*2 + 1] as i32;
            ltotal += left.abs();
            rtotal += right.abs();
        }

        ltotal = ltotal / (buf.len() as i32 * 2);
        rtotal = rtotal / (buf.len() as i32 * 2);

        (ltotal, rtotal)
    }
}

