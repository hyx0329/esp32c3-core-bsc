//! LCD HAT designed by LuatOS team
#![allow(unused_imports)]
use embedded_hal::digital::blocking::OutputPin;
use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_hal_0_2::blocking::delay::DelayMs;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{
    Gpio10, Gpio11, Gpio13, Gpio2, Gpio3, Gpio4, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9, Input, Output,
};
use esp_idf_hal::spi::{config::Config, Dma, Master, Pins as SpiPins, Spi, SPI2};
use esp_idf_hal::units::Hertz;
use st7735_lcd::{Orientation, ST7735};

/// Sets up all the needed GPIO pins for the LCD(only)
///
/// ```
/// let peripherals = Peripherals::take().unwrap();
/// let lcd_pins = lcd_pins!(peripherals);
/// ```
#[macro_export]
macro_rules! lcd_pins {
    ($peripherals:ident) => {{
        $crate::lcd_hat::LcdPins {
            miso: None, // no need, and occupied for RST
            mosi: $peripherals.pins.gpio3.into_output().unwrap(),
            sck: $peripherals.pins.gpio2.into_output().unwrap(),
            cs: $peripherals.pins.gpio7.into_output().unwrap(),
            dc: $peripherals.pins.gpio6.into_output().unwrap(),
            rst: $peripherals.pins.gpio10.into_output().unwrap(),
        }
    }};
}

type SckPin = Gpio2<Output>;
type MosiPin = Gpio3<Output>;
type MisoPin = Gpio10<Input>;
type CsPin = Gpio7<Output>;
type DcPin = Gpio6<Output>;
type RstPin = Gpio10<Output>;
type SpiType = Master<SPI2, SckPin, MosiPin, MisoPin, CsPin>;
pub type Lcd = ST7735<SpiType, DcPin, RstPin>;

/// Pins consumed by LCD driver
pub struct LcdPins {
    pub miso: Option<MisoPin>,
    pub mosi: MosiPin,
    pub sck: SckPin,
    pub cs: CsPin,
    pub dc: DcPin,
    pub rst: RstPin,
}

pub fn configure(spi: SPI2, pins: LcdPins) -> Lcd {
    let (width, height) = (160, 80);
    let spi_pins = SpiPins {
        sclk: pins.sck,
        sdo: pins.mosi,
        sdi: None,
        cs: Some(pins.cs),
    };

    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let spi_config = Config::new()
        .baudrate(Hertz::from(4000000))
        .data_mode(spi_mode)
        .write_only(true);

    // spi numbered from 1, we use the second one
    let spi2 = SpiType::new(spi, spi_pins, spi_config).unwrap();

    // bgr no-invert 80x160
    let mut lcd = ST7735::new(spi2, pins.dc, pins.rst, false, false, width, height);

    let mut delay = FreeRtos {};

    lcd.init(&mut delay).unwrap();
    lcd.set_orientation(&Orientation::Landscape).unwrap();
    lcd.set_offset(0, 24); // this might differ

    lcd
}
