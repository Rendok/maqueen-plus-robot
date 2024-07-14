use microbit::hal::{twim::Instance, Twim};

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

pub struct MotorController {
    i2c_address: u8,
}

impl MotorController {
    pub fn new(i2c_address: u8) -> Self {
        MotorController { i2c_address }
    }

    pub fn control<T>(&self, i2c: &mut Twim<T>, motor: Motor, direction: Direction, speed: u8)
    where
        T: Instance,
    {
        if speed > 240 {
            panic!("The speed must be between 0 and 240");
        }
        match motor {
            Motor::Left => {
                let mut buf = [0u8; 3];
                buf[0] = 0x00;
                buf[1] = direction as u8;
                buf[2] = speed;
                i2c.write(self.i2c_address, &buf).unwrap();
            }
            Motor::Right => {
                let mut buf = [0u8; 3];
                buf[0] = 0x02;
                buf[1] = direction as u8;
                buf[2] = speed;
                i2c.write(self.i2c_address, &buf).unwrap();
            }
            Motor::Both => {
                let mut buf = [0u8; 5];
                buf[0] = 0x00;
                buf[1] = direction as u8;
                buf[2] = speed;
                buf[3] = direction as u8;
                buf[4] = speed;
                i2c.write(self.i2c_address, &buf).unwrap();
            }
        }
    }
}
