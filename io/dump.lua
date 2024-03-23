local args = {...}
local outputFile
pcall(function(args)
    local outputName = args[2]
    outputFile = io.open(outputName, 'wb')
end, args)

if outputFile then
    local s, e = pcall(function(args)
        local inputName = args[1]
        local inputFile = io.open(inputName, 'rb')
        local inputContent = inputFile:read("*all")
        inputFile:close()

        local bytecode = string.dump(loadstring(inputContent))

        outputFile:write(bytecode)
    end, args)

    if not s then
        outputFile:write(e)
    end
end
