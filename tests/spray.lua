-- Name: Spray
-- Author: Sebastian
-- Date: 29.05.2025

-- The init()-function is called once before every simulation
--  Use init() to configure your space automat.
function init(ship)
    ship:name("spray") -- set name
    ship:slot(1, "propulsion") -- register propulsion on slot 1
    ship:slot(2, "plasma cannon") -- register plasma cannon on slot 2

    ship:write(1, 0, 1)  -- write to slot 1 (aka propulsion): Enable propulsion, backward
    ship:write(1, 1, 100) -- write to slot 1: Propulsion power
    ship:write(2, 0, 1) -- write to slot 2: Enable plasma cannon
end

-- The run()-function is called in every simulation step
function run(ship)
end