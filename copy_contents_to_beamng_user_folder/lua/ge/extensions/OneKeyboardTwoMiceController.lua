local M = {}
local socket = require("socket")
local udp = nil
local deviceInst = nil

local prevState = {} -- so we only emit changes

local function onExtensionUnloaded()
    if deviceInst ~= nil and extensions.core_input_virtualInput then
        extensions.core_input_virtualInput.deleteDevice(deviceInst)
        deviceInst = nil
    end
end

local function onUpdate(dt)
    if not udp then
        udp = socket.udp()
        udp:settimeout(0)
        udp:setsockname("127.0.0.1", 5555)
    end

    if deviceInst == nil and extensions.core_input_virtualInput then
        local axes = 4
        local buttons = 0
        deviceInst = extensions.core_input_virtualInput.createDevice("BeamNG 1 Kb 2 Mice Controller", "bng_1k2m", axes, buttons, 0)
    end

    local latest_data = nil
    while true do
        local data = udp:receive()
        if not data then break end
        latest_data = data
    end

    if latest_data and deviceInst then
        for axis, raw_val in enumerate(latest_data:gmatch("[^|]+")) do
            local val = tonumber(raw_val) or 0
            if prevState[axis] ~= val then
                extensions.core_input_virtualInput.emit(deviceInst, "axis", axis, "change", val)
                prevState[axis] = val
            end
        end
    end
end

M.onUpdate = onUpdate
M.onExtensionUnloaded = onExtensionUnloaded
return M
