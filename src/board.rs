use crate::hc_sr04::DistanceSensor;
use heapless::Vec;
use microbit::gpio::BTN_A;
use microbit::hal::gpio::Level;
use microbit::hal::{Clocks, Rtc, Twim};
use microbit::pac::{CLOCK, RTC0, TIMER0, TIMER1, TWIM0};
use microbit::{
    board::Board as MicrobitBoard,
    display::blocking::Display,
    hal::{prelude::*, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};
use rtt_target::rprintln;

const BOARD_ADDR: u8 = 0x10;

pub mod motor_control {
    pub enum Motor {
        Left,
        Right,
        Both,
    }

    #[repr(u8)]
    #[derive(Clone, Copy)]
    pub enum Direction {
        Stop = 0,
        Forward = 1,
        Backward = 2,
    }
}

pub struct Board {
    pub button_a: BTN_A,
    pub display: Display,
    i2c_0: Twim<TWIM0>,
    pub timer_0: Timer<TIMER0>,
    timer_1: Timer<TIMER1>,
    rtc: Rtc<RTC0>,
    distance_sensor: DistanceSensor,
}

impl Board {
    pub fn new(microbit_board: MicrobitBoard) -> Self {
        let timer_0 = Timer::new(microbit_board.TIMER0);
        let timer_1 = Timer::new(microbit_board.TIMER1);
        let display = Display::new(microbit_board.display_pins);
        let i2c_0 = {
            Twim::new(
                microbit_board.TWIM0,
                microbit_board.i2c_external.into(),
                FREQUENCY_A::K400,
            )
        };
        let distance_sensor = DistanceSensor::new(
            microbit_board.pins.p0_02.into_push_pull_output(Level::Low),
            microbit_board.pins.p0_03.into_floating_input(),
        );

        let rtc = Rtc::new(microbit_board.RTC0, 1).unwrap();
        // Clocks::new(microbit_board.CLOCK).start_lfclk();

        Board {
            display,
            i2c_0,
            timer_0,
            timer_1,
            distance_sensor,
            rtc,
            button_a: microbit_board.buttons.button_a,
        }
    }

    pub fn control(
        &mut self,
        motor: motor_control::Motor,
        direction: motor_control::Direction,
        speed: u8,
    ) {
        let speed = speed.clamp(0, 240);
        match motor {
            motor_control::Motor::Left => {
                let mut buf = [0u8; 3];
                buf[0] = 0x00;
                buf[1] = direction as u8;
                buf[2] = speed;
                self.i2c_0.write(BOARD_ADDR, &buf).unwrap();
            }
            motor_control::Motor::Right => {
                let mut buf = [0u8; 3];
                buf[0] = 0x02;
                buf[1] = direction as u8;
                buf[2] = speed;
                self.i2c_0.write(BOARD_ADDR, &buf).unwrap();
            }
            motor_control::Motor::Both => {
                let mut buf = [0u8; 5];
                buf[0] = 0x00;
                buf[1] = direction as u8;
                buf[2] = speed;
                buf[3] = direction as u8;
                buf[4] = speed;
                self.i2c_0.write(BOARD_ADDR, &buf).unwrap();
            }
        }
    }

    pub fn measure_distance(&mut self) -> Option<u16> {
        self.distance_sensor
            .measure(&mut self.timer_0, &mut self.timer_1)
    }

    pub fn print_board_version(&mut self) {
        let mut size = [0; 1];
        self.i2c_0
            .write_read(BOARD_ADDR, &[0x32u8], &mut size)
            .unwrap();

        let size: usize = size[0].into();
        let mut version = [0; 20];
        self.i2c_0
            .write_read(BOARD_ADDR, &[0x33u8], &mut version[0..size])
            .unwrap();

        rprintln!(
            "Board version {}",
            core::str::from_utf8(&version[0..size]).unwrap()
        );
    }

    // From left to right: L3, L2, L1, R1, R2, R3. `true` is when a sensor is open (is_light).
    pub fn read_light_sensors(&mut self) -> Vec<bool, 6> {
        let mut buf = [0u8; 1];
        self.i2c_0
            .write_read(BOARD_ADDR, &[0x1D], &mut buf)
            .unwrap();

        let mut sensors = Vec::<bool, 6>::new();
        for _ in 0..6 {
            sensors.push(buf[0] & 1 == 1).unwrap();
            buf[0] = buf[0] >> 1;
        }

        sensors
    }

    pub fn read_light_sensors_grayscale(&mut self) -> Vec<u16, 6> {
        let mut buf = [0u8; 12];
        self.i2c_0
            .write_read(BOARD_ADDR, &[0x1E], &mut buf)
            .unwrap();

        let mut sensors = Vec::<u16, 6>::new();

        for i in (0..12).step_by(2) {
            sensors
                .push((buf[i] as u16) << 8 | buf[i + 1] as u16)
                .unwrap();
        }

        sensors
    }
}
