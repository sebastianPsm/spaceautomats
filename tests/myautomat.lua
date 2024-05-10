-- The init()-function is called once before every simulation
--  Use init() to configure your space automat.
function init(training)
    ship:slot(0, "propulsion")
    ship:slot(1, "reaction wheels")
end

-- The run()-function is called in every simulation step
function run(step)
    ship:write(1024, 3);
    ship:write(1025, 255);
end