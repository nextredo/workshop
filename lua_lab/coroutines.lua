#! /bin/lua

--[[
Main loop
- prints something every so often

Aux loop
- does something more often

NOTE
- Lua's os.clock < std::chrono::steady_clock
- This doesn't really need to be a coroutine
  - See <https://www.lua.org/pil/9.1.html> for more info
    - Producer - consumer
    - Coroutines as iterators
--]]


local function aux_fn()
    local last_time = os.time()

    while true do
        if os.difftime(os.time(), last_time) >= 2 then
            print("aux")
            last_time = os.time()
        end
        coroutine.yield()
    end
end


-- local function main_fn()
-- end
-- local main_cor = coroutine.create(main_fn)

--------------------------------------------------------------------------------
local aux_cor = coroutine.create(aux_fn)
for _=1,40 do
    coroutine.resume(aux_cor)
    print("main")
    os.execute("sleep 0.2")
end
