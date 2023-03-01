local wt = require "wezterm"
local a = wt.action

local theme = require("color_file")

local M = {
  tab_max_width = 20,
  color_scheme = theme,
  harfbuzz_features = { "calt=0", "clig=0", "liga=0" },
  automatically_reload_config = true,
  hide_mouse_cursor_when_typing = true,
  hide_tab_bar_if_only_one_tab = true,
  use_fancy_tab_bar = false,
  font_size = 10,
  adjust_window_size_when_changing_font_size = false,
  window_decorations = "RESIZE",
  switch_to_last_active_tab_when_closing_tab = true,
  window_padding = {
    left = 0,
    right = 0,
    top = 0,
    bottom = 0,
  },
  disable_default_key_bindings = true,
}

M.keys = {}
M.key_tables = {}

local function map(key, mod, action)
  table.insert(M.keys, {
      key = key,
      mods = mod,
      action = action
  })
end

M.leader = { key = "q", mods = "CTRL" }
local le = "LEADER"

-- shortcuts
map('f', le, a.SpawnCommandInNewTab { args = { 'go_to' }, })
map('t', le, a.SpawnCommandInNewTab { args =  { 'wetheme' }, })
map('e', le, a.SpawnCommandInNewTab { args = { 'walk', "/home/vstalivyshel" }, })

-- command palette
map('p', le, a.ActivateCommandPalette)

-- copy paste
map('C', 'CTRL', a.CopyTo 'Clipboard')
map('V', 'CTRL', a.PasteFrom 'Clipboard')

-- font size
map('0', 'CTRL', a.ResetFontSize)
map('=', 'CTRL', a.IncreaseFontSize)
map('-', 'CTRL', a.DecreaseFontSize)

-- splits
map('-', "ALT", a.SplitVertical { domain = 'CurrentPaneDomain' })
map('\\', "ALT", a.SplitHorizontal { domain = 'CurrentPaneDomain' })
map('r', le, a.ActivateKeyTable { name = 'resize_pane', one_shot = false })
map('0', "ALT", a.ActivatePaneDirection 'Next')
map('9', "ALT", a.ActivatePaneDirection 'Prev')

-- tabs
map('=', "ALT", a.SpawnTab 'CurrentPaneDomain')
for i = 1, 8 do
  map('F' .. tostring(i), '', a.ActivateTab(i - 1))
  map(tostring(i), 'ALT', a.ActivateTab(i - 1))
end

-- copy mode
local cm = {}
map('Space', "ALT", a.ActivateCopyMode)
local copy_mode = wt.gui.default_key_tables().copy_mode
for _, key in pairs(cm) do
  table.insert(copy_mode, key)
end

local resize_mode = {
  { key = 'h', action = a.AdjustPaneSize { 'Left', 5 } },
  { key = 'l', action = a.AdjustPaneSize { 'Right', 5 } },
  { key = 'k', action = a.AdjustPaneSize { 'Up', 5 } },
  { key = 'j', action = a.AdjustPaneSize { 'Down', 5 } },
  { key = 'c', mods = 'CTRL', action = 'PopKeyTable' },
  { key = 'Escape', action = 'PopKeyTable' },
}

-- modes
M.key_tables = {
  resize_pane = resize_mode,
  copy_mode = copy_mode,
}

return M



