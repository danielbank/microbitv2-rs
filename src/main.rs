//! https://tech.microbit.org/hardware/
//!
//! cargo run --release

#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use nrf_softdevice_defmt_rtt as _; // global logger
use panic_probe as _; // print out panic messages
mod ble;

use ble::{bluetooth_task, embassy_config, softdevice_config, softdevice_task};
use defmt::{info, unwrap};
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::gpio::{self, AnyPin, Pin};
use embassy_nrf::gpiote::{self, Channel};
use embassy_nrf::Peripherals;
use embedded_hal::digital::v2::OutputPin;
use nrf_softdevice::Softdevice;

#[embassy::main(config = "embassy_config()")]
async fn main(spawner: Spawner, dp: Peripherals) {
    // well use these logging macros instead of println to tunnel our logs via the debug chip
    info!("Hello World!");

    // some bluetooth under the covers stuff we need to start up
    let config = softdevice_config();
    let sd = Softdevice::enable(&config);

    // button presses will be delivered on LotoHi or when you release the button
    let button1 = gpiote::InputChannel::new(
        // degrade just a typesystem hack to forget which pin it is so we can
        // call it Anypin and make our function calls more generic
        dp.GPIOTE_CH1.degrade(),
        gpio::Input::new(dp.P0_14.degrade(), gpio::Pull::Up),
        gpiote::InputChannelPolarity::LoToHi,
    );

    // microbit dosent have a single led, it has a matrix where you set the
    // column high AND row low for the led you want to turn on.

    // row1 permenantly powered
    let _row1 = gpio::Output::new(
        dp.P0_21.degrade(),
        gpio::Level::High,
        gpio::OutputDrive::Standard,
    );

    // The column pins are active low, start leds high (off)
    let red = gpio::Output::new(
        dp.P0_28.degrade(),
        gpio::Level::High,
        gpio::OutputDrive::Standard,
    );

    let red5 = gpio::Output::new(
        dp.P0_30.degrade(),
        gpio::Level::High,
        gpio::OutputDrive::Standard,
    );

    // tell the executor to start each of our tasks
    unwrap!(spawner.spawn(softdevice_task(sd)));
    // note this unwrap! macro is just like .unwrap() you're used to, but for
    // various reasons has less size for microcontrollers
    unwrap!(spawner.spawn(bluetooth_task(sd, button1, red5)));
    unwrap!(spawner.spawn(blinky_task(red)));

    // we can sneak another 'task' here as well instead of exiting
    let mut red2 = gpio::Output::new(
        dp.P0_11.degrade(),
        gpio::Level::High,
        gpio::OutputDrive::Standard,
    );

    loop {
        red2.set_low().unwrap();
        Timer::after(Duration::from_millis(1000)).await;
        red2.set_high().unwrap();
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy::task]
async fn blinky_task(mut red: gpio::Output<'static, AnyPin>) {
    loop {
        red.set_high().unwrap();
        Timer::after(Duration::from_millis(1000)).await;
        red.set_low().unwrap();
        Timer::after(Duration::from_millis(1000)).await;
    }
}

// just a bookkeeping function for our logging library
// WARNING may overflow and wrap-around in long lived apps
defmt::timestamp! {"{=usize}", {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static COUNT: AtomicUsize = AtomicUsize::new(0);
        // NOTE(no-CAS) `timestamps` runs with interrupts disabled
        let n = COUNT.load(Ordering::Relaxed);
        COUNT.store(n + 1, Ordering::Relaxed);
        n
    }
}
