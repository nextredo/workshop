#! /bin/lua


---Basic fn to test pass by value/reference
---@param stuff table
local function do_thing(stuff)
    stuff.a = stuff.a + 2
end

local this = { a = 0 }

--------------------------------------------------------------------------------
print(this.a)
this.a = this.a + 1
print(this.a)
do_thing(this)
print(this.a)
print(tostring(this.a))
