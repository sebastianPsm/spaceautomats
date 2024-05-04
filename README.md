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

| Offset 	| Name  	| r/w 	| Description                                    	|
|--------	|-------	|-----	|------------------------------------------------	|
| 0      	| Ctrl  	| w   	| Bit 0: Enabled<br>Bit 1: Forward               	|
| 1      	| Power 	| w   	| Power level (0..255)                           	|
| 2      	| Fuel  	| r   	| Fuel level (Bit 0..7, least significant octet) 	|
| 3      	| Fuel  	| r   	| Fuel level (Bit 8..15)                         	|
| 4      	| Fuel  	| r   	| Fuel level (Bit 16..23)                        	|
| 5      	| Fuel  	| r   	| Fuel level (Bit 24..31)                        	|

#### Example

*init()*

    ship.slot(1, 'propulsion')
    ship.write(0x1001, 255) -- Set max. to the Power memory address

*run()* 

    ship.write(0x1000, 3) -- Set the Enabled- and Forward-flag from the Ctrl memory addres
    b1 = ship.read(0x1002) -- Read all 4 fuel bytes and put them together
    b2 = ship.read(0x1003)
    b3 = ship.read(0x1004)
    b4 = ship.read(0x1005)
    fuel = b1 | b2<<8 | b3<<16 | b4<<24