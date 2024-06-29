-- The init()-function is called once before every simulation
--  Use init() to configure your space automat.
function init(ship)
    ship:name("myautomat")
    ship:slot(1, "propulsion")
    ship:slot(2, "reaction wheel")
end

-- The run()-function is called in every simulation step
t = 0
function run(ship)
    ship:write(1, 0, 3)
	ship:write(1, 1, 255)
	ship:write(2, 0, 1)
	ship:write(2, 1, 1)
	t = t + 1
	if(t > 100)
	then
		ship:write(1, 1, 100)
		ship:write(2, 1, 0)
	end

end