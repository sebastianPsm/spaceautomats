-- The init()-function is called once before every simulation
--  Use init() to configure your space automat.
function init(ship)
    ship:name("myautomat")
    ship:slot(0, "propulsion")
    ship:slot(1, "reaction wheels")
end

-- The run()-function is called in every simulation step
function run(ship)
    ship:write(1024, 3);
    ship:write(1025, 255);
end