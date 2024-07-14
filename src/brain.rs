use crate::board::motor_control::{Direction, Motor};
use crate::board::Board;
use heapless::Vec;
use rtt_target::rprintln;

pub fn keep_distance(board: &mut Board, goal_distance_cm: u16) {
    let distance_cm = board.measure_distance();
    rprintln!("{:?}", distance_cm);

    if distance_cm.is_none() {
        board.control(Motor::Both, Direction::Stop, 0u8);
        return;
    }

    let measured_distance_cm = distance_cm.unwrap();

    if measured_distance_cm <= goal_distance_cm - 30 {
        board.control(Motor::Both, Direction::Backward, 100u8);
    } else if measured_distance_cm <= goal_distance_cm - 10 {
        board.control(Motor::Both, Direction::Backward, 40u8);
    } else if measured_distance_cm >= goal_distance_cm + 30 {
        board.control(Motor::Both, Direction::Forward, 100u8);
    } else if measured_distance_cm >= goal_distance_cm + 10 {
        board.control(Motor::Both, Direction::Forward, 40u8);
    } else {
        board.control(Motor::Both, Direction::Stop, 0u8);
    }
}

pub fn keep_distance_p(board: &mut Board, goal_distance_cm: u16) {
    let distance_cm = board.measure_distance();
    rprintln!("{:?}", distance_cm);

    if distance_cm.is_none() {
        board.control(Motor::Both, Direction::Stop, 0u8);
        return;
    }

    let measured_distance_cm = distance_cm.unwrap();

    if measured_distance_cm >= goal_distance_cm {
        let velocity = 3 * (measured_distance_cm - goal_distance_cm);
        board.control(
            Motor::Both,
            Direction::Forward,
            velocity.clamp(0, 240) as u8,
        );
    } else {
        let velocity = 3 * (goal_distance_cm - measured_distance_cm);
        board.control(
            Motor::Both,
            Direction::Backward,
            velocity.clamp(0, 240) as u8,
        );
    }
}

pub fn follow_line(board: &mut Board, sensors: &Vec<bool, 6>) {
    if !sensors[1] && !sensors[2] && !sensors[3] && !sensors[4] {
        rprintln!("Forward");
        board.control(Motor::Both, Direction::Forward, 0u8);
    } else if !sensors[1] && sensors[2] && sensors[3] && !sensors[4] {
        rprintln!("Forward");
        board.control(Motor::Both, Direction::Forward, 40u8);
    } else if sensors[2] && !sensors[3] && !sensors[4] {
        rprintln!("Left");
        board.control(Motor::Left, Direction::Forward, 00u8);
        board.control(Motor::Right, Direction::Forward, 40u8);
    } else if !sensors[1] && !sensors[2] && sensors[3] {
        rprintln!("Right");
        board.control(Motor::Left, Direction::Forward, 40u8);
        board.control(Motor::Right, Direction::Forward, 00u8);
    } else if sensors[0] || sensors[1] {
        rprintln!("Left");
        board.control(Motor::Left, Direction::Forward, 0u8);
        board.control(Motor::Right, Direction::Forward, 40u8);
    } else if sensors[4] || sensors[5] {
        rprintln!("Right");
        board.control(Motor::Left, Direction::Forward, 40u8);
        board.control(Motor::Right, Direction::Forward, 0u8);
    } else {
        rprintln!("Stop");
        board.control(Motor::Both, Direction::Stop, 0u8);
    }
}
