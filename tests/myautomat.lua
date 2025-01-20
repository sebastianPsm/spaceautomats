function turn(ship, value)
    if value < -255 then
        value = -255
    elseif value > 255 then
        value = 255
    end

    if value < 0 then
        ship:write(2, 0, 3)
    else
        ship:write(2, 0, 1)
    end
	ship:write(2, 1, math.abs(value))
end

function scan(ship)
    -- Scan
    nDetections = ship:read(3, 5) -- get number of detections
    minDistance = 100000000
    hasAngle = false
    angle = 0
    ship:log(string.format("nDetections: %d\n", nDetections))

    for idx=1,nDetections do
        d = ship:read(3, 5+idx) -- get detection distance
        a = ship:read(3, 6+idx) -- get detection angle (0..255, right to left))
		if d < minDistance then
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

	ship:write(1, 0, 0) -- enable propulsion, forward
	ship:write(1, 1, 50) -- propulsion power

    turn(ship, 0)

	ship:write(3, 0, 1) -- enable scanner
	ship:write(3, 1, 127) -- aperture angle (x/255*360)
	ship:write(3, 2, 255) -- max. detection distance
	ship:write(3, 3, 0) -- heading
end


-- The run()-function is called in every simulation step
t = 0
function run(ship)
	t = t + 1
    scan(ship)
end