#!/bin/env luajit

local args = arg
local inserted_char = args[1]
local opts = args[2]
local write = io.write
local str_gsub = string.gsub

local q = "'"
local Q = '"'
local PAIRS = {
    "b", "(", ")",
    "r", "[", "]",
    "a", "<", ">",
    "Q", Q, Q,
    "B", "{", "}",
    "q", q, q,
}

local left = inserted_char
local right = inserted_char
local open_letter
local open
local close
for idx = 1, #PAIRS, 3 do
    open_letter = PAIRS[idx]
    open = PAIRS[idx + 1]
    close = PAIRS[idx + 2]
	if left == open or left == open_letter then
    	left = open
    	right = close
	end
end

if #opts > 1 then
    local spc = " "
	opts = str_gsub(opts, 's', '')
	left = left .. spc
	right = spc .. right
end


local match = {
	b = left .. right,
	l = left,
	r = right,
}

write(match[opts])
