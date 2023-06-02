extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::sample::{Spec, Format};

pub struct AudioView {
    pa_simple: Simple,
}

impl AudioView {
    pub fn new() -> Self {
        let spec = Spec {
            format: Format::FLOAT32NE,
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

        AudioView{
            pa_simple,
        }
    }

    // Collect some audio and measure it's average level
    pub fn read_average(&mut self, n_samples: usize) -> (f32,f32) {


        let mut buf : Vec<f32> = vec![0.0; n_samples*2];

        // println!( "[{}] reading...",  self.start.elapsed().as_millis());
        unsafe {
            let buf_u8 = buf.align_to_mut::<u8>().1;
            self.pa_simple.read(buf_u8).unwrap();
        }

        // let latency = self.pa_simple.get_latency().unwrap();
        // println!( "[{}] nsamples: {}, latency: {} ms", self.start.elapsed().as_millis(), n_samples, latency.as_millis());
        
        let mut llevel: f32 = 0.;
        let mut rlevel: f32 = 0.;

        for i in 0..buf.len()/2 {
            let left =  buf[i*2];
            let right =  buf[i*2 + 1];
            llevel += left.abs();
            rlevel += right.abs();
        }

        llevel = llevel / (buf.len() as f32 * 2.);
        rlevel = rlevel / (buf.len() as f32 * 2.);

        // println!( "[{}] llevel: {}, rlevel: {}", self.start.elapsed().as_millis(), llevel, rlevel);

        (llevel, rlevel)
    }
}

