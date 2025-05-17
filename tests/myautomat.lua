-- Global variables
global_t = 0

function turn(ship, value)
    value = (value < -255) and -255 or value
    value = (value > 255) and 255 or value
	
    ship:write(2, 0, value < 0 and 3 or 1)
	ship:write(2, 1, math.abs(value))
end
function turn_stop(ship)
    counter_clock = ship:read(2, 2)
    ang_velo_1 = ship:read(2, 3) << 0
    ang_velo_2 = ship:read(2, 4) << 8
    ang_velo_3 = ship:read(2, 5) << 16
    ang_velo_4 = ship:read(2, 6) << 24
    ang_velo = (ang_velo_1 + ang_velo_2 + ang_velo_3 + ang_velo_4) / 1000.0 + 0.5

    turn(ship, counter_clock and -ang_velo or ang_velo)
end


function scan(ship)
    -- Scan
    nDetections = ship:read(3, 5) -- get number of detections
    minDistance = 100000000
    hasAngle = false
    angle = 0

    for idx=1,nDetections do
        d = ship:read(3, 5+idx) -- get detection distance
        a = ship:read(3, 6+idx) -- get detection angle (0..255, right to left))
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
    aperturAngle = ship:read(3, 1) / 255 * 360
    detectionAngle = angle / 255 * aperturAngle
    detectionAngle = detectionAngle - aperturAngle / 2
    ship:log(string.format("angle: %.2f, %.2f\n", detectionAngle, aperturAngle))
    ship:log(string.format("minDistance: %.2f\n", minDistance))


    curHead = ship:read(3, 3, 5) -- get current heading (0..255)


    ship:write(3, 3, 0) -- scanner heading
--	ship:write(3, 1, 5) -- angle
--		ship:write(3, 1, 255-t%255) -- aperture angle (x/255*360)
    
end

-- The init()-function is called once before every simulation
--  Use init() to configure your space automat.
function init(ship)
    ship:name("myautomat")
    ship:slot(1, "propulsion")
    ship:slot(2, "reaction wheel")
    ship:slot(3, "scanner")

	ship:write(1, 0, 3)
 -- enable propulsion, forward
	ship:write(1, 1, 0) -- propulsion power

	turn(ship, 1)

	ship:write(3, 0, 1) -- enable scanner
	ship:write(3, 1, 127) -- aperture angle (x/255*360)
	ship:write(3, 2, 255) -- max. detection distance
	ship:write(3, 3, 0) -- heading
end


-- The run()-function is called in every simulation step
function run(ship)
	global_t = global_t + 1
    scan(ship)

    if global_t > 300 then
		turn_stop(ship)
	end	
end