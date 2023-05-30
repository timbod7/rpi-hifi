
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, PrimitiveStyle}, mono_font::{MonoTextStyleBuilder, MonoTextStyle}, text::{TextStyle, Baseline, Text},
};

use rppal::i2c::I2c;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode};

type DisplayType = Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

pub struct Display {

    pub target: DisplayType,
    pub char_style: MonoTextStyle<'static, BinaryColor>,
    pub text_style: TextStyle,
    pub stroke_style: PrimitiveStyle<BinaryColor>,
}

impl Display {
    pub fn new() ->  Self {

    let i2c = I2c::new().unwrap();
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

    let stroke_style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    Display{
        target: display,
        char_style,
        text_style,
        stroke_style,
        }
    }

    pub fn render_text(&mut self, text: &str, position: Point) {
        Text::with_text_style(text, position, self.char_style, self.text_style)
        .draw(&mut self.target)
        .unwrap();
    }

    pub fn render_rect(&mut self,  top_left: Point, size: Size) {
        Rectangle::new(top_left, size)
        .into_styled(self.stroke_style)
        .draw(&mut self.target)
        .unwrap();

    }
}