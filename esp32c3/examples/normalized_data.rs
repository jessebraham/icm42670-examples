#![no_std]
#![no_main]

use core::fmt::Write;

use esp32c3_hal::{
    gpio::IO,
    i2c::I2C,
    pac::Peripherals,
    prelude::*,
    Delay,
    RtcCntl,
    Timer,
    UsbSerialJtag,
};
use icm42670::{prelude::*, Address, Icm42670};
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let mut delay = Delay::new(peripherals.SYSTIMER);
    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
    let mut timer0 = Timer::new(peripherals.TIMG0);
    let mut timer1 = Timer::new(peripherals.TIMG1);

    rtc_cntl.set_super_wdt_enable(false);
    rtc_cntl.set_wdt_enable(false);
    timer0.disable();
    timer1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio10,
        io.pins.gpio8,
        400_000,
        &mut peripherals.SYSTEM,
    )
    .unwrap();

    let mut icm = Icm42670::new(i2c, Address::Primary).unwrap();

    loop {
        let accel_norm = icm.accel_norm().unwrap();
        let gyro_norm = icm.gyro_norm().unwrap();

        writeln!(
            UsbSerialJtag,
            "ACCEL  =  X: {:+.04} Y: {:+.04} Z: {:+.04}\t\tGYRO  =  X: {:+.04} Y: {:+.04} Z: {:+.04}",
            accel_norm.x, accel_norm.y, accel_norm.z, gyro_norm.x, gyro_norm.y, gyro_norm.z
        )
        .ok();

        delay.delay_ms(100u32);
    }
}
