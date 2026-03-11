
local start_time = os.clock()

local i = 0

while i < 1000000000 do
    i = i + 1
end

local end_time = os.clock()

print("Result:", i)
print(string.format("Executed in: %.0f ms", (end_time - start_time) * 1000))
