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
    nDetections = ship:read(3, 5)
	minDistance = #INF
	hasAngle = false
	angle = 0

    for idx=1,nDetections do
        d = ship:read(3, 5+idx)
        a = ship:read(3, 6+idx)
        
    end


	-- ship:write(3, 1, 500) -- angle
--		ship:write(3, 1, 255-t%255) -- aperture angle (x/255*360)
--		ship:write(3, 3, t%255) -- scanner heading
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
	ship:write(3, 1, 100) -- aperture angle (x/255*360)
	ship:write(3, 2, 255) -- max. detection distance
	ship:write(3, 3, 0) -- heading
end


-- The run()-function is called in every simulation step
t = 0
function run(ship)
	t = t + 1

end