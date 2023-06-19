#!/bin/env lua

local io = io
local os = os
local arg = arg
local fmt = string.format
local unpack = table.unpack
local nop = ""

local function write(msg, ...)
	io.write(fmt(msg, ...))
end

local function panic(msg, ...)
	write([[ echo -markup {Error}err making colorscheme, see debug ;]])
	local first = true
	msg = fmt(msg .. "\n", ...)
	for line in msg:gmatch("[^\n:]+") do
		if first then
			write("echo -debug MAKE COLOR ERROR: ; ")
			first = false
		end
		write("echo -debug %%{    %s } ;", line)
	end
	os.exit(1)
end

local debug_on = false
local function dbg(msg, ...)
	if debug_on then
		write("DBG: " .. msg .. "\n", ...)
	end
end

-- local function file_exists(file)
--    local ok, err, code = os.rename(file, file)
--    return ok and true or code == 13 and true  or false, err
-- end

local template = [[
-- { <fg>, [bg], [attr] } or just <fg>
-- Color: "#RRGGBB" or 'MenuInfo = "Default"', exception: func = "function"
-- Attribute: "+..."
--- Palette ---
local default = "default"

return {
    Colorscheme_name       = ,

    --- Builtins ---
    Default                = ,
    InsertCursor           = ,
    PrimarySelection       = ,
    SecondarySelection     = ,
    PrimaryCursor          = ,
    PrimaryCursorEol       = ,
    SecondaryCursor        = ,
    SecondaryCursorEol     = ,
    LineNumbers            = ,
    LineNumberCursor       = ,
    LineNumbersWrapped     = ,
    MenuForeground         = ,
    MenuBackground         = ,
    MenuInfo               = ,
    Information            = ,
    Error                  = ,
    StatusLine             = ,
    StatusLineMode         = ,
    StatusLineInfo         = ,
    StatusLineValue        = ,
    StatusCursor           = ,
    Prompt                 = ,
    MatchingChar           = ,
    Whitespace             = ,
    WrapMarker             = ,
    BufferPadding          = ,

    --- Code ---
    value                  = ,
    type                   = ,
    variable               = ,
    module                 = ,
    func                   = ,
    string                 = ,
    keyword                = ,
    operator               = ,
    attribute              = ,
    comment                = ,
    documentation          = ,
    meta                   = ,
    builtin                = ,

    --- LSP ---
    DiagnosticError        = ,
    DiagnosticWarning      = ,
    DiagnosticHint         = ,
    DiagnosticInfo         = ,
    LineFlagError          = ,
    LineFlagWarning        = ,
    LineFlagHint           = ,
    LineFlagInfo           = ,
    InlayDiagnosticError   = ,
    InlayDiagnosticWarning = ,
    InlayDiagnosticHint    = ,
    InlayDiagnosticInfo    = ,
    InlayCodeLens          = ,
    InlayHint              = ,

    --- Markup ---
    title                  = ,
    header                 = ,
    bold                   = ,
    italic                 = ,
    mono                   = ,
    block                  = ,
    link                   = ,
    bullet                 = ,
    list                   = ,
}
]]

if #arg < 1 then
	write(template)
	return
end
if #arg == 1 then
	debug_on = true
end

local colors_file, color_path = unpack(arg)
local colors, err = loadfile(colors_file)
if not colors then
	panic("fail to load color file:\n%s", err)
end

local ok
ok, colors = pcall(function()
	return colors
end)
if not ok then
	panic("failed to get values from file: %s", colors)
end
colors = colors()

local color_name = colors.Colorscheme_name
if not color_name or color_name == "" then
	panic("not colorscheme name supplied")
end
colors.Colorscheme_name = nil

local function from_rgb(rgb)
	return rgb:find("#") and rgb:gsub("#", "rgb:") or rgb
end

local function extract(table)
	if type(table) ~= "table" then
		return from_rgb(table)
	end
	local fg, bg, attr = unpack(table)
	if not fg then
		return nil
	end
	bg = not bg and nop or bg:find("+") and bg or "," .. from_rgb(bg)
	attr = not attr and nop or attr
	return from_rgb(fg) .. bg .. attr
end

local out = nop
for face, specs in pairs(colors) do
	dbg("face before before: %s", face)
	face = face == "func" and "function" or face
	dbg("face before")
	specs = extract(specs)
	dbg("face and specs after: %s = %s", face, specs)
	if not specs then
		panic("wrong spec count: no fg value in %s", face)
	end
	out = fmt('%sface global %s "%s"\n', out, face, specs)
end

dbg(
	[[
OUT: {{{
%s
}}}]],
	out
)
if not color_path then
	return
end
color_path = fmt("%s/%s.kak", color_path, color_name)

local target_file
target_file, err = io.open(color_path, "w")
if not target_file then
	panic("failed to write colors to file: %s", err)
end
target_file:write(out)
write('echo -markup {Prompt}"%s" stored in %s ; colorscheme %s', color_name, color_path, color_name)
