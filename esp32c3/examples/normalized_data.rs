#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    prelude::*,
    clock::{CpuClock},
    i2c::master::I2c,
};

use esp_backtrace as _;
use esp_println::println;
use icm42670::{prelude::*, Address, Icm42670};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::Clock160MHz;
        config
    });

    let delay = Delay::new();

    let i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
        .with_sda(peripherals.GPIO10)
        .with_scl(peripherals.GPIO8);

    let mut icm = Icm42670::new(i2c, Address::Primary).unwrap();

    loop {
        let accel_norm = icm.accel_norm().unwrap();
        let gyro_norm = icm.gyro_norm().unwrap();

        println!(
            "ACCEL  =  X: {:+.04} Y: {:+.04} Z: {:+.04}\t\tGYRO  =  X: {:+.04} Y: {:+.04} Z: {:+.04}",
            accel_norm.x, accel_norm.y, accel_norm.z, gyro_norm.x, gyro_norm.y, gyro_norm.z
        );

        delay.delay_millis(1_000u32);
    }
}
