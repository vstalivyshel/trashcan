local wt = require "wezterm"
local a = wt.action
local l = 'LEADER'

local O = {
  color_scheme = require("selected_color"),
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

O.keys = {}
O.key_tables = {}
O.leader = { key = 'q', mods = 'CTRL' }

local function sk(key, mod, action)
  table.insert(O.keys, {
    key = key,
    mods = mod,
    action = action
  })
end

-- shortcuts
sk('c', l, a.SpawnCommandInNewTab { args = { 'fix_cfg' }, })
sk('f', l, a.SpawnCommandInNewTab { args = { 'go_pers' }, })
sk('t', l, a.SpawnCommandInNewTab { args = { 'wetheme' }, })

-- command palette
sk('p', l, a.ActivateCommandPalette)

-- copy paste
sk('C', 'CTRL', a.CopyTo 'Clipboard')
sk('V', 'CTRL', a.PasteFrom 'Clipboard')

-- font size
sk('0', 'CTRL', a.ResetFontSize)
sk('=', 'CTRL', a.IncreaseFontSize)
sk('-', 'CTRL', a.DecreaseFontSize)

-- splits
sk('s', l, a.SplitVertical { domain = 'CurrentPaneDomain' })
sk('v', l, a.SplitHorizontal { domain = 'CurrentPaneDomain' })
sk('j', 'ALT', a.ActivatePaneDirection 'Down')
sk('k', 'ALT', a.ActivatePaneDirection 'Up')
sk('l', 'ALT', a.ActivatePaneDirection 'Right')
sk('h', 'ALT', a.ActivatePaneDirection 'Left')
sk('r', l, a.ActivateKeyTable { name = 'resize_pane', one_shot = false })

-- tabs
sk('w', l, a.SpawnTab 'CurrentPaneDomain')
sk('0', 'ALT', a.ActivateTabRelative(1))
sk('9', 'ALT', a.ActivateTabRelative( -1))
for i = 1, 8 do
  sk('F' .. tostring(i), '', a.ActivateTab(i - 1))
  sk(tostring(i), 'ALT', a.ActivateTab(i - 1))
end

-- copy mode
local cm = {}
sk('Space', l, a.ActivateCopyMode)
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
  { key = 'c', mods = 'CTRL',                           action = 'PopKeyTable' },
}

-- modes
O.key_tables = {
  resize_pane = resize_mode,
  copy_mode = copy_mode,
}

return O
