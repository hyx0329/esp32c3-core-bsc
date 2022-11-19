use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{
        Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};
use embedded_hal_0_2::blocking::delay::DelayMs;
use esp32c3_core_bsc::hal::delay::FreeRtos;
use esp32c3_core_bsc::hal::peripherals::Peripherals;
use esp32c3_core_bsc::led::{Led, Led1, Led2};
use esp32c3_core_bsc::{lcd_hat, lcd_pins};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // According to esp-idf-template:
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    println!("Initializing peripherals and led pins");

    let peripherals = Peripherals::take().unwrap();
    let led1_pin = peripherals.pins.gpio12;
    let led2_pin = peripherals.pins.gpio13;

    let mut led1 = Led1::new(led1_pin.into_output().unwrap());
    let mut led2 = Led2::new(led2_pin.into_output().unwrap());

    println!("Switching leds");
    led1.off();
    led2.on();

    let leds: [&mut dyn Led; 2] = [&mut led1, &mut led2];

    println!("Initializing lcd");
    let lcd_pins = lcd_pins!(peripherals);
    let mut lcd = lcd_hat::configure(peripherals.spi2, lcd_pins);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    println!("Clear screen!");
    Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    // code below adapted from embedded_graphics's example
    let thin_stroke = PrimitiveStyle::with_stroke(Rgb565::CYAN, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Rgb565::CYAN, 3);
    let fill = PrimitiveStyle::with_fill(Rgb565::CYAN);
    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb565::CYAN);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::CYAN)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let yoffset = 10; // change this

    println!("Draw a bounding box!");
    lcd.bounding_box()
        .into_styled(border_stroke)
        .draw(&mut lcd)
        .unwrap();

    println!("Draw a triangle!");
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut lcd)
    .unwrap();

    println!("Draw a filled square!");
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut lcd)
        .unwrap();

    println!("Draw a circle with a 3px wide stroke!");
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut lcd)
        .unwrap();

    println!("Draw centered text!");
    let text = "embedded-graphics";
    Text::with_alignment(
        text,
        lcd.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut lcd)
    .unwrap();

    println!("Draw a diagonal line!");
    Line::new(Point::new(0, 0), Point::new(160, 80))
        .into_styled(thin_stroke)
        .draw(&mut lcd)
        .unwrap();

    println!("Start the loop for blinky!");
    loop {
        for _ in 0..3 {
            FreeRtos.delay_ms(1_000 as u32);
            for id in 0..2 {
                leds[id].toggle();
            }
        }
    }
}
