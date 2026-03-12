local function fib(n)
    if n < 2 then
        return n
    else
        return fib(n - 1) + fib(n - 2)
    end
end

local function generate_array(arr, s)
    local seed = 12345
    local i = 0
    
    while i < s do
        seed = (seed * 1103515245 + 12345)
        
        seed = seed % 4294967296 
        if seed >= 2147483648 then seed = seed - 4294967296 end
        
        local val = seed
        if val < 0 then
            val = 0 - val
        end
        
        local mod_val = val - (val // 10000) * 10000
        
        arr[i] = mod_val
        i = i + 1
    end
end

local function partition(arr, low, high)
    local pivot = arr[high]
    local i = low - 1
    local j = low

    while j < high do
        if arr[j] < pivot then
            i = i + 1
            
            local temp = arr[i]
            arr[i] = arr[j]
            arr[j] = temp
        end
        j = j + 1
    end
    
    local temp2 = arr[i + 1]
    arr[i + 1] = arr[high]
    arr[high] = temp2

    return i + 1
end

local function quicksort(arr, low, high)
    if low < high then
        local pi = partition(arr, low, high)
        
        quicksort(arr, low, pi - 1)
        quicksort(arr, pi + 1, high)
    end
end

local start_time = os.clock()

print("Starting Fib(35)...")
local fib_result = fib(35)
print("Fib result: " .. fib_result)
local fib_time = os.clock()
print("Fib Time: " .. (fib_time - start_time) .. " seconds")

print("Starting 100x Quicksort on 10k array...")
local s = 10000
local arr = {}
local iter = 0

while iter < 10000 do
    generate_array(arr, s)
    quicksort(arr, 0, s - 1)
    iter = iter + 1
end

print("Array median index 5000: " .. arr[5000])

local end_time = os.clock()
print("Sort Time: " .. (end_time - fib_time) .. " seconds")
print("Total Execution Time: " .. (end_time - start_time) .. " seconds")
