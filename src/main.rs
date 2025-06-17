#![no_std]
#![no_main]

use hal::println;
use hal::delay::Delay;
use hal::gpio::{Level, Output};
use ch32_hal as hal;

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let config = hal::Config::default();
    let p = hal::init(config);

    let mut delay = Delay;

    let mut led = Output::new(p.PD6, Level::Low, Default::default());

    delay.delay_ms(10);

    println!("hello world!");

    println!("Flash size: {}kb", hal::signature::flash_size_kb());
    println!("Chip UID: {:x?}", hal::signature::unique_id());
    let chip_id = hal::signature::chip_id();
    println!("Chip {}, DevID: 0x{:x}", chip_id.name(), chip_id.dev_id());
    println!("Clocks: {:?}", hal::rcc::clocks());

    loop {
        led.toggle();
        match led.get_output_level() {
            Level::Low => println!("Off!"),
            Level::High => println!("On!"),
        }
        delay.delay_ms(1000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
