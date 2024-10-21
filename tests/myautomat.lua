-- The init()-function is called once before every simulation
--  Use init() to configure your space automat.
function init(ship)
    ship:name("myautomat")
    ship:slot(1, "propulsion")
    ship:slot(2, "reaction wheel")
    ship:slot(3, "scanner")

	ship:write(1, 0, 0) -- enable propulsion, forward
	ship:write(1, 1, 50) -- propulsion power

	ship:write(2, 0, 1) -- enable reaction wheel
	ship:write(2, 1, 1) -- turn

	ship:write(3, 0, 1) -- enable scanner
	ship:write(3, 1, 10) -- aperture angle (x/255*360)
	ship:write(3, 2, 255) -- max. detection distance
	ship:write(3, 3, 0) -- heading
	ship:write(3, 4, 255) --sensitivity
end

-- The run()-function is called in every simulation step
t = 0
function run(ship)
	t = t + 1
	if(t > 200)
	then
--		ship:write(1, 1, 0) -- propulsion power off
		ship:write(2, 1, 0) -- turn off
--		ship:write(3, 1, 255-t%255) -- aperture angle (x/255*360)
--		ship:write(3, 3, t%255) -- scanner heading
	end
end