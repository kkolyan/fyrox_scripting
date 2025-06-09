---@uuid 7c259fd2-fdb9-453b-a7ef-19cdd85428cc
---@class Beacon : NodeScript
Beacon = script_class()

function Beacon:on_update()
    local pos = self.node.global_position
    table.insert(GlobalScript:get("Game").beacons, pos)
    print("beacon registered: " .. tostring(self.node));
    self.node:destroy()
end
