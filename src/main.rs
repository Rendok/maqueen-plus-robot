#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod board;
mod brain;
mod hc_sr04;

use board::motor_control::{Direction, Motor};
use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::prelude::*;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

fn light_one(x: usize, y: usize) -> [[u8; 5]; 5] {
    let mut zeros = [[0u8; 5]; 5];
    zeros[x][y] = 1;
    zeros
}

const DISPLAY_A_LETTER: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let microbit_board = Board::take().unwrap();

    let mut board = board::Board::new(microbit_board);

    board.print_board_version();
    board.control(Motor::Both, Direction::Stop, 0u8);

    rprintln!("Press button A to start.");
    while board.button_a.is_high().unwrap() {
        board
            .display
            .show(&mut board.timer_0, DISPLAY_A_LETTER, 100);
    }

    loop {
        brain::keep_distance_p(&mut board, 30);
        // let sensors = board.read_light_sensors();
        // brain::follow_line(&mut board, &sensors);
        // rprintln!("{:?}", board.read_light_sensors());
        // rprintln!("{:?}", board.read_light_sensors_grayscale());

        // board.timer_0.delay_ms(500u32);
    }

    let mut x = 0;
    let mut y = 0;
    loop {
        board.display.show(&mut board.timer_0, light_one(x, y), 100);

        if x == 4 && y == 0 {
            y += 1;
        } else if x == 4 && y == 4 {
            x -= 1;
        } else if x == 0 && y == 4 {
            y -= 1;
        } else if x == 0 && y == 0 {
            x += 1;
        } else if x > 0 && y == 0 {
            x += 1;
        } else if x > 0 && y == 4 {
            x -= 1;
        } else if y > 0 && x == 0 {
            y -= 1;
        } else {
            y += 1;
        }
        board.display.clear();
    }
}
