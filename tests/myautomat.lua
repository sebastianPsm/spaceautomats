-- Constants
PROPULSION = 1
REACTION_WHEEL = 2
SCANNER = 3
PLASMA_CANNON = 4

-- Global variables
global_t = 0
global_dir0 = 0
global_dist0 = 0
-- Common helper functions
function read_u16(ship, slot, start_address)
    val_1 = ship:read(slot, start_address+0) <<  0 -- low byte
    val_2 = ship:read(slot, start_address+1) <<  8 -- high byte
    return val_1 + val_2
end
function write_u16(ship, slot, start_address, value)
    value_ = math.floor(value + 0.5)
    ship:write(slot, start_address+0, value_ & 0xFF)
    ship:write(slot, start_address+1, (value_ >> 8) & 0xFF)
end
function read_u32(ship, slot, start_address)
    val_1 = ship:read(slot, start_address+0) <<  0 -- low byte
    val_2 = ship:read(slot, start_address+1) <<  8
    val_3 = ship:read(slot, start_address+2) << 16
    val_4 = ship:read(slot, start_address+3) << 24 -- high byte
    return val_1 + val_2 + val_3 + val_4
end
function normRad(val)
    res = math.fmod(val + math.pi, 2*math.pi)
    if res < 0 then
        res = res + 2 * math.pi
    end
    return res - math.pi
end

-- Navigation helper
__distance = 0
__velocity = 0
function forward(ship, setpoint)
    K = {0.04, 1}
    
    err = __distance-setpoint
   
	thrust = -K[1] * err -K[2] * __velocity
    thrust = math.min(math.max(thrust, -255), 255) -- saturate
    __velocity = __velocity + thrust -- * t (* 1)
    __distance = __distance + __velocity -- * t (* 1)

	ship:write(PROPULSION, 0, thrust >= 0 and 3 or 1)
	ship:write(PROPULSION, 1, math.abs(thrust))
   
    return -err
end
function turn(ship, setpoint)
    K = {5, 500}
    processval = read_u32(ship, PROPULSION, 12) / 1E6
    err = normRad(processval - setpoint)

    dir = ship:read(REACTION_WHEEL, 2) == 0 and -1 or 1
	angular_velocity = dir*(read_u32(ship, REACTION_WHEEL, 3) / 1E6)

    torque = -K[1] * err -K[2] * angular_velocity
    torque = math.min(math.max(torque, -255), 255) -- saturate

	ship:write(REACTION_WHEEL, 0, torque >= 0 and 3 or 1)
	write_u16(ship, REACTION_WHEEL, 1, math.abs(torque))

	return -err
end
-- Other helper
function scan(ship) -- provides the relative angle to the closest detection or nil
    nDetections = ship:read(SCANNER, 5) -- get number of detections
    minDistance = 100000000
    angle = nil

    for idx=1,nDetections do
        d = ship:read(SCANNER, 5+idx) -- get detection distance
        angle = ship:read(SCANNER, 6+idx) -- get detection angle (0..255, right to left))
		if d < minDistance then -- find closest detection
            minDistance = d
        end
    end
    if not angle then
        return nill
    end

    -- Get absolut position of detection
    aperturAngle = ship:read(SCANNER, 1) / 255 * 2 * math.pi
    detectionAngle = (angle / 255 - 0.5) * aperturAngle
    return detectionAngle
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

    global_dir0 = read_u32(ship, PROPULSION, 12) / 1E6
    global_dist0 = 0
end

loiter_state = 0
function loiter(ship, forward_err, turn_error)
	if loiter_state == 0 then -- go forward (set)
		ship:log("loiter_state == 0\n")
        global_dist0 = global_dist0 + 500000
        loiter_state = 1
		return
	end
	if loiter_state == 1 and forward_err < 100 then -- go forward (wait)
		ship:log("loiter_state == 1\n")
        loiter_state = 2
		return
	end
	if loiter_state == 2 then -- turn (set)
		ship:log("loiter_state == 2\n")
		global_dir0 = global_dir0 + math.pi/2
		loiter_state = 3
		return
	end
	if loiter_state == 3 and turn_error < 0.01 then -- turn (wait)
		ship:log("loiter_state == 3\n")
		loiter_state = 0
		return
	end
end


-- The run()-function is called in every simulation step
function run(ship)
	global_t = global_t + 1
	
	--d = scan(ship)
	--ship:write(PLASMA_CANNON, 0, 0) -- reset plasma cannon
    --if d then
    --    heading = read_u32(ship, PROPULSION, 12) / 1E6
    --    a = heading-d
	--	ship:write(PLASMA_CANNON, 0, 1) -- enable plasma cannon
    --end
	--turn(ship, a)

    -- navigation
    forward_err = forward(ship, global_dist0)
	turn_error = turn(ship, global_dir0)

    loiter(ship, forward_err, turn_error)
end