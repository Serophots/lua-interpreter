local argss = { ... }
local outputFile
pcall(function(args)
    local outputName = args[2]
    outputFile = io.open(outputName, 'wb')
end, argss)

if outputFile then
    local s, e = pcall(function(args)
        local inputName = args[1]
        -- local inputFile = io.open(inputName, 'rb')
        -- local inputContent = inputFile:read("*all")
        -- inputFile:close()

        -- local bytecode = string.dump(load(inputContent))
        --
        local bytecode = string.dump(assert(loadfile(inputName)))

        outputFile:write(bytecode)
    end, argss)

    if not s then
        outputFile:write(e)
    end
end
