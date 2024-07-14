use embedded_hal::timer::Cancel;
use libm::round;
use microbit::hal::gpio::p0::{P0_02, P0_03};
use microbit::hal::gpio::{Floating, Input, Output, PushPull};
use microbit::hal::prelude::*;
use microbit::hal::{clocks, Timer};
use microbit::hal::{time, Rtc};
use microbit::pac::{RTC0, TIMER0, TIMER1};
use rtt_target::rprintln;

const SPEED_OF_SOUND: f64 = 331.4 + 0.606 * 25.0/*Celsius*/; //m/s
const TIMER_FREQUENCY: f64 = 1_000_000.0;

pub struct DistanceSensor {
    trigger_pin: P0_02<Output<PushPull>>,
    echo_pin: P0_03<Input<Floating>>,
}

impl DistanceSensor {
    pub fn new(trigger_pin: P0_02<Output<PushPull>>, echo_pin: P0_03<Input<Floating>>) -> Self {
        Self {
            trigger_pin,
            echo_pin,
        }
    }

    pub fn measure(&mut self, delay: &mut Timer<TIMER0>, timer: &mut Timer<TIMER1>) -> Option<u16> {
        self.trigger_pin.set_low().unwrap();
        delay.delay_ms(1000u32);
        timer.start(1000_000u32);
        self.trigger_pin.set_high().unwrap();
        delay.delay_us(10u32);
        self.trigger_pin.set_low().unwrap();

        while self.echo_pin.is_low().unwrap() {}

        let star_time = timer.read();

        while self.echo_pin.is_high().unwrap() {}

        let end_time = timer.read();
        let distance_cm =
            (end_time - star_time) as f64 * (SPEED_OF_SOUND / TIMER_FREQUENCY as f64) / 2.0 * 100.0;

        timer.cancel().unwrap();

        // The sensor operating range is 5cm-300cm.
        if distance_cm >= 5.0 && distance_cm <= 300.0 {
            return Some(round(distance_cm) as u16);
        }
        None
    }
}
