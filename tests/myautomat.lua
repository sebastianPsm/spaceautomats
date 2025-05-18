-- Global variables
global_t = 0

function accelerate(ship, power, thresh)
	velo_1 = ship:read(1, 6) << 0
	velo_2 = ship:read(1, 7) << 8
	velo = velo_1 + velo_2 -- current velocity

	ship:write(1, 0, 0) -- diable propulsion
	ship:write(1, 1, 0) -- propulsion power
	if velo < thresh then
		ship:write(1, 0, 3) -- enable propulsion, forward
		ship:write(1, 1, power) -- propulsion power
	elseif velo > thresh then
		ship:write(1, 0, 1) -- enable propulsion, backward
		ship:write(1, 1, power) -- propulsion power
	end    
end

function turn(ship, power, heading)
	head_1 = ship:read(1, 8) << 0
	head_2 = ship:read(1, 9) << 8
	head_3 = ship:read(1, 10) << 16
	head_4 = ship:read(1, 11) << 24
	curr_heading = (head_1 + head_2 + head_3 + head_4) / 1E6 -- current heading
	d_head = ((curr_heading - heading) + 2*math.pi)%math.pi -- heading delta

	

    ship:write(2, 0, 0) -- disable reaction wheel
	ship:write(2, 1, 0) -- turn power
	if d_head > math.pi then
	    ship:write(2, 0, 1)
		ship:write(2, 1, power)
	elseif d_head < math.pi then
	    ship:write(2, 0, 3)
		ship:write(2, 1, power)
	end
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

	ship:write(3, 0, 1) -- enable scanner
	ship:write(3, 1, 127) -- aperture angle (x/255*360)
	ship:write(3, 2, 255) -- max. detection distance
	ship:write(3, 3, 0) -- heading
end


-- The run()-function is called in every simulation step
function run(ship)
	global_t = global_t + 1
    scan(ship)

	accelerate(ship, 255, 1000)
	turn(ship, 2, 5)

    if global_t > 300 then
		accelerate(ship, 255, 0)
	end	
end