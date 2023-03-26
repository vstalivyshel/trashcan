#!/bin/env lua

OPEN_LETTER = { "b", "r", "a", "Q", "B", "q" }
OPEN        = { "(", "[", "<", '"', "{", "'" }
CLOSE       = { ")", "]", ">", '"', "}", "'" }

function pair_item(item)
	for idx = 1, #OPEN do
		if item == OPEN[idx] or item == OPEN_LETTER[idx] then
			local left = OPEN[idx]
			local right = CLOSE[idx]
			return { left, right }
		end
	end
	return { item, item }
end

local opt = {
	both = false,
	only_left = false,
	only_right = false,
}

local inserted_char

for _, ar in ipairs(arg) do
	if ar == "-l" then
		opt.only_left = true
	elseif ar == "-r" then
		opt.only_right = true
	elseif ar == "-b" then
		opt.both = true
	else
		inserted_char = ar
	end
end

local pairs = pair_item(inserted_char)

if not inserted_char then
	return
elseif opt.both then
	print(pairs[1] .. pairs[2])
elseif opt.only_left then
	print(pairs[1])
elseif opt.only_right then
	print(pairs[2])
else
	return
end
