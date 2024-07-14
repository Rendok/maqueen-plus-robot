# Rust driver for the Maqueen Plus V1.0

https://wiki.dfrobot.com/SKU_MBT0021-EN_Maqueen_Plus_STEAM_Programming_Educational_Robot

The robot is controlled by Microbit:v2.

This is by no means a complete implementation. I used this code in my experiments.
Because there is no official support for Rust in Maqueen or a datasheet of the board, the implementation was
established by reverse engineering and may be useful for other developers. 

# Pins
microbit:v2 `board.pins` to Maqueen plus GPIO labels:
* p0_02 -> P0
* p0_03 -> P1
* p0_04 -> P2
* p0_10 -> P8
* p0_12 -> P12
* p0_17 -> P13