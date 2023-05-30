use std::error::Error;

use rppal::i2c::I2c;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle}, mono_font::MonoTextStyleBuilder, text::{TextStyle, Baseline, Text},
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() -> Result<(), Box<dyn Error>>  {

    let i2c = I2c::new()?;
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();


    let char_style = MonoTextStyleBuilder::new()
        .font(&profont::PROFONT_12_POINT)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();
    let text_style = TextStyle::with_baseline(Baseline::Top);


    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    Text::with_text_style("12:43", Point::new(0, 0), char_style, text_style)
        .draw(&mut display)
        .unwrap();

    Text::with_text_style("Cpu 13%", Point::new(72, 0), char_style, text_style)
        .draw(&mut display)
        .unwrap();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 40), Size::new(127, 1))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    Ok(())
}
