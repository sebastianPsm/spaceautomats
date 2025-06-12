-- Constants
PROPULSION = 1
REACTION_WHEEL = 2
SCANNER = 3
PLASMA_CANNON = 4

-- Global variables
global_t = 0

function read_u16(ship, slot, start_address)
    val_1 = ship:read(slot, start_address+0) <<  0 -- low byte
    val_2 = ship:read(slot, start_address+1) <<  8 -- high byte
    return val_1 + val_2
end
function read_u32(ship, slot, start_address)
    val_1 = ship:read(slot, start_address+0) <<  0 -- low byte
    val_2 = ship:read(slot, start_address+1) <<  8
    val_3 = ship:read(slot, start_address+2) << 16
    val_4 = ship:read(slot, start_address+3) << 24 -- high byte
    return val_1 + val_2 + val_3 + val_4
end
function write_u16(ship, slot, start_address, value)
    value_ = math.floor(value + 0.5)
    ship:write(slot, start_address+0, value_ & 0xFF)
    ship:write(slot, start_address+1, (value_ >> 8) & 0xFF)
end
function normRad(val)
    res = math.fmod(val + math.pi, 2*math.pi)
    if res < 0 then
        res = res + 2 * math.pi
    end
    return res - math.pi
end
function turn(ship, setpoint)
    processval = read_u32(ship, PROPULSION, 12) / 1E6
    err = normRad(setpoint - processval)

    dir = ship:read(REACTION_WHEEL, 2) == 0 and -1 or 1
	curr_angular_velocity = dir*(read_u32(ship, REACTION_WHEEL, 3) / 1E6)

	torque = 5 * err - 500 * curr_angular_velocity

	ship:write(REACTION_WHEEL, 0, torque >= 0 and 3 or 1)
	write_u16(ship, REACTION_WHEEL, 1, math.abs(torque))
end

function accelerate(ship, power, thresh)
    velo = read_u16(ship, PROPULSION, 6) -- current velocity

    ship:write(PROPULSION, 0, 0) -- diable propulsion
    ship:write(PROPULSION, 1, 0) -- propulsion power
    if velo < thresh then
        ship:write(PROPULSION, 0, 3) -- enable propulsion, forward
        ship:write(PROPULSION, 1, power) -- propulsion power
    elseif velo > thresh then
        ship:write(PROPULSION, 0, 1) -- enable propulsion, backward
        ship:write(PROPULSION, 1, power) -- propulsion power
    end    
end

function scan(ship)
    -- Scan
    nDetections = ship:read(SCANNER, 5) -- get number of detections
    minDistance = 100000000
    hasAngle = false
    angle = 0

    for idx=1,nDetections do
        d = ship:read(SCANNER, 5+idx) -- get detection distance
        a = ship:read(SCANNER, 6+idx) -- get detection angle (0..255, right to left))
		if d < minDistance then -- find closest detection
            minDistance = d
            angle = a
            hasAngle = true
        end
    end
    if not hasAngle then
        return
    end

    -- Get absolut position of detection
    aperturAngle = ship:read(SCANNER, 1) / 255 * 360
    detectionAngle = angle / 255 * aperturAngle
    detectionAngle = detectionAngle - aperturAngle / 2
    ship:log(string.format("angle: %.2f, %.2f\n", detectionAngle, aperturAngle))
    ship:log(string.format("minDistance: %.2f\n", minDistance))


    curHead = ship:read(SCANNER, 3, 5) -- get current heading (0..255)


    ship:write(SCANNER, 3, 0) -- scanner heading
--	ship:write(SCANNER, 1, 5) -- angle
--		ship:write(SCANNER, 1, 255-t%255) -- aperture angle (x/255*360)

end

-- The init()-function is called once before every simulation
--  Use init() to configure your space automat.
function init(ship)
    ship:name("myautomat")
    ship:slot(PROPULSION, "propulsion")
    ship:slot(REACTION_WHEEL, "reaction wheel")
    ship:slot(SCANNER, "scanner")
	ship:slot(PLASMA_CANNON, "plasma cannon")

	ship:write(PROPULSION, 0, 1) -- enable propulsion

	ship:write(SCANNER, 0, 1) -- enable scanner
	ship:write(SCANNER, 1, 127) -- aperture angle (x/255*360)
	ship:write(SCANNER, 2, 255) -- max. detection distance
	ship:write(SCANNER, 3, 0) -- heading

	ship:write(PLASMA_CANNON, 0, 0) -- enable plasma cannon
end


-- The run()-function is called in every simulation step
a = 3
function run(ship)
	global_t = global_t + 1
	
	scan(ship)
	
	if global_t % 300 == 0 then
		a = math.random() * 10
	end

	--turn(ship, a)

end