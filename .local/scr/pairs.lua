#!/bin/env lua

io.write([[
	def new-command \
	-shell-completion \
	-override \
	%{echo %sh{echo "Sasha"}
]])

-- local inserted = "" .. arg[1]

-- local opening = { '[', '{', '(', '<', '"' }

-- local closing = { ']', '}', ')', '>', '"' }

-- for index, op_pair in ipairs(opening) do
--     if inserted == op_pair then
--         io.write(closing[index])
--     end
-- end

