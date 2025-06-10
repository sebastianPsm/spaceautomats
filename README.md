## Devices

Each ship has 10 slots (1..10), each of which can be equipped with one device.

For example, during *init()* the following will configure a *propulsion*-device to slot#1 and a *reaction wheel*-device to slot#2.

    ship.slot(1, 'propulsion')
    ship.slot(2, 'reaction wheel')

The devices are mapped to the memory addresses 0x1000 (propulsion) and 0x2000 (reaction wheel). Use `x = ship.read(addr)` and `ship.write(addr, value)` to read or write from the device. The exact `addr` is the device memory addres + the offset from the device description. From the example above the propulsion has the memory address 0x1000 and the Power the offset 1, therefore write to 0x1001 to set a new power level.

The following devices are available:

* Propulsion

### Propulsion ('propulsion')

A propulsion device comes with a fuel tank and drives your ship.

| Offset    | Name     | r/w    | Description                                         |
|--------   |-------   |-----   |------------------------------------------------     |
| 0         | Ctrl     | w      | Bit 0: Enabled<br>Bit 1: Forward                    |
| 1         | Power    | w      | Power level (0..255)                                |
| 2         | Fuel     | r      | Fuel level (Bit 0..7, least significant octet)      |
| 3         | Fuel     | r      | Fuel level (Bit 8..15)                              |
| 4         | Fuel     | r      | Fuel level (Bit 16..23)                             |
| 5         | Fuel     | r      | Fuel level (Bit 24..31)                             |
| 6         | Velocity | r      | Velocity (Bit 0..7, least significant octet)        |
| 7         | Velocity | r      | Velocity (Bit 8..15)  -signed-                      |
| 8         | Velocity (direction) | r      | Direction of the velocity in µrad (Bit 0..7, least significant octet) |
| 9         | Velocity (direction) | r      | Direction of the velocity in µrad (Bit 8..15)  -unsigned-             |
| 10        | Velocity (direction) | r      | Direction of the velocity in µrad (Bit 16..23)                        |
| 11        | Velocity (direction) | r      | Direction of the velocity in µrad (Bit 24..31)                        |
| 12        | Heading  | r      | Heading in µrad (Bit 0..7, least significant octet) |
| 13        | Heading  | r      | Heading in µrad (Bit 8..15)  -unsigned-             |
| 14        | Heading  | r      | Heading in µrad (Bit 16..23)                        |
| 15        | Heading  | r      | Heading in µrad (Bit 24..31)                        |

#### Example

*init()*

    ship.slot(1, 'propulsion')
    ship.write(1, 1, 255) -- Set max. to the Power memory address

*run()*

    ship.write(1, 0, 3) -- Set the Enabled- and Forward-flag from the Ctrl memory addres
    b1 = ship.read(1, 2) -- Read all 4 fuel bytes and put them together
    b2 = ship.read(1, 3)
    b3 = ship.read(1, 4)
    b4 = ship.read(1, 5)
    fuel = b1 | b2<<8 | b3<<16 | b4<<24

### Reaction wheel ('reaction wheel')

A reaction wheel device rotates the ship.

| Offset    | Name     | r/w    | Description                                                       |
|--------   |-------   |-----   |-------------------------------------------------------------------|
| 0         | Ctrl     | w      | Bit 0: Enabled<br>Bit 1: Rotate counter-clockwise if set          |
| 1         | Power    | w      | Power level (0..255)                                              |
| 2         | Dir      | r      | 0 if rotation is clockwise, 1 if counter-clockwise                |
| 3         | Ang velo | r      | Angular velocity in µrad/step (Bit 0..7, least significant octet) |
| 4         | Ang velo | r      | Angular velocity in µrad/step (Bit 8..15)  -unsigned-             |
| 5         | Ang velo | r      | Angular velocity in µrad/step (Bit 16..23)                        |
| 6         | Ang velo | r      | Angular velocity in µrad/step (Bit 24..31)                        |


![Reaction wheel system](/img/reaction_wheel_system.png)

State space for the reaction wheel system

A: [0 -1; 0 0]
B: [0; 1]
C: [1 1]
D: 0

### Scanner ('scanner')

A scanner is able to detect objects in the surrounding area. A scanner provides up to 5 detections. The
closest detection is 'Detection#1'.

Scalings:
- Distance: 0 .. 255 (max. detections distance)
- Angle:
-- 0: In front of the aperture
-- 127: On the left of the aperture
-- -127: On the right of the aperture

| Offset | Name                    | r/w | Description
|--------|-------------------------|-----|-----------------
| 0      | Ctrl                    | w   | Bit 0: Enabled
| 1      | Aperture angle          | w   | Aperture angle of the scanner
| 2      | Max. detection distance | w   | Max. detection distance of the scanner * 1000
| 3      | Heading                 | w   | Heading offset of the scanner
| 4      | Sensitivity             | w   | Sensitivity of the scanner
| 5      | No of detections        | r   | 0 .. 5
| 6      | Detection#1             | r   | Detection#1 distance
| 7      | Detection#1             | r   | Detection#1 angle
| 8      | Detection#2             | r   | Detection#2 distance
| 9      | Detection#2             | r   | Detection#2 angle
| 10     | Detection#3             | r   | Detection#3 distance
| 11     | Detection#3             | r   | Detection#3 angle
| 12     | Detection#4             | r   | Detection#4 distance
| 13     | Detection#4             | r   | Detection#4 angle
| 14     | Detection#5             | r   | Detection#5 distance
| 15     | Detection#5             | r   | Detection#5 angle
| 16     | Boundary                | r   | Boundary distance
| 17     | Boundary                | r   | Boundary angle
#### Example

*init()*

    ship.slot(3, 'scanner')
    ship.write(3, 0, 1) -- Enable scanner
    ship.write(3, 1, 60) -- Set aperture angle
    ship.write(3, 2, 255) -- Set max. detection distance
    ship.write(3, 3, 0) -- Set heading offset
    ship.write(3, 4, 255) -- Set sensitivity

### Plasma cannon ('plasma cannon')

A plasma cannon shoots plasma to destroy enemies.

| Offset    | Name     | r/w    | Description                                         |
|--------   |-------   |-----   |------------------------------------------------     |
| 0         | Ctrl     | w      | Bit 0: Enabled                                      |
