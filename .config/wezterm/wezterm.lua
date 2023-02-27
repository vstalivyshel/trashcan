local wt = require "wezterm"
local a = wt.action
local l = 'LEADER'
local M = {
  tab_max_width = 25,
  color_scheme = require("color_file"),
  harfbuzz_features = { "calt=0", "clig=0", "liga=0" },
  automatically_reload_config = true,
  hide_mouse_cursor_when_typing = true,
  hide_tab_bar_if_only_one_tab = true,
  tab_bar_at_bottom = true,
  use_fancy_tab_bar = false,
  font_size = 10.2,
  adjust_window_size_when_changing_font_size = false,
  window_decorations = "RESIZE",
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
M.leader = { key = 'q', mods = 'CTRL' }

local function map(key, mod, action)
  table.insert(M.keys, {
      key = key,
      mods = mod,
      action = action
  })
end

-- shortcuts
map('f', l, a.SpawnCommandInNewTab { args = { 'go_to' }, })
map('t', l, a.SpawnCommandInNewTab { args = { 'wetheme' }, })
map('e', l, a.SpawnCommandInNewTab { args = { 'walk', "/home/vstalivyshel" }, })

-- command palette
map('p', l, a.ActivateCommandPalette)

-- copy paste
map('C', 'CTRL', a.CopyTo 'Clipboard')
map('V', 'CTRL', a.PasteFrom 'Clipboard')

-- font size
map('0', 'CTRL', a.ResetFontSize)
map('=', 'CTRL', a.IncreaseFontSize)
map('-', 'CTRL', a.DecreaseFontSize)

-- splits
map('s', l, a.SplitVertical { domain = 'CurrentPaneDomain' })
map('v', l, a.SplitHorizontal { domain = 'CurrentPaneDomain' })
map('r', l, a.ActivateKeyTable { name = 'resize_pane', one_shot = false })
map('j', 'CTRL|SHIFT', a.ActivatePaneDirection 'Down')
map('k', 'CTRL|SHIFT', a.ActivatePaneDirection 'Up')
map('l', 'CTRL|SHIFT', a.ActivatePaneDirection 'Right')
map('h', 'CTRL|SHIFT', a.ActivatePaneDirection 'Left')

-- tabs
map('w', l, a.SpawnTab 'CurrentPaneDomain')
map('0', 'ALT', a.ActivateTabRelative(1))
map('9', 'ALT', a.ActivateTabRelative( -1))
for i = 1, 8 do
  map('F' .. tostring(i), '', a.ActivateTab(i - 1))
  map(tostring(i), 'ALT', a.ActivateTab(i - 1))
end

-- copy mode
local cm = {}
map('Space', l, a.ActivateCopyMode)
local copy_mode = wt.gui.default_key_tables().copy_mode
for _, key in pairs(cm) do
  table.insert(copy_mode, key)
end

-- resize pane mo
local resize_mode = {
  { key = 'h', action = a.AdjustPaneSize { 'Left', 5 } },
  { key = 'l', action = a.AdjustPaneSize { 'Right', 5 } },
  { key = 'k', action = a.AdjustPaneSize { 'Up', 5 } },
  { key = 'j', action = a.AdjustPaneSize { 'Down', 5 } },
  { key = 'c', mods = 'CTRL', action = 'PopKeyTable' },
}

-- modes
M.key_tables = {
  resize_pane = resize_mode,
  copy_mode = copy_mode,
}

return M
