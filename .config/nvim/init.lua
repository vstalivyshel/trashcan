local vim = vim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
	vim.fn.system({
		"git",
		"clone",
		"--filter=blob:none",
		"https://github.com/folke/lazy.nvim.git",
		lazypath,
	})
end

local o = vim.opt
o.rtp:prepend(lazypath)
o.swapfile = false
o.backup = false
o.smartindent = true
o.laststatus = 1
o.ignorecase = false
o.tabstop = 4
o.softtabstop = 4
o.shiftwidth = 4
o.expandtab = true
o.nu = true
o.wrap = false
o.relativenumber = true
o.scrolloff = 15
o.splitbelow = true
o.splitright = true
o.splitkeep = "cursor"
o.hlsearch = false
o.path:append("**")

local map = vim.keymap.set
vim.g.mapleader = " "
map("n", "<leader>f", [[:find ]])
map("n", "<leader>b", [[<cmd>ls<cr>:]])
map("n", "ds", [[mz%"_x`z"_x]])
map("t", "<c-/>", "<c-\\><c-n>")
map("n", "<leader>h", "<cmd>noh<cr>")
map("n", "x", [["_x]])
map("n", "<A-o>", "mzo<esc>`z")
map("n", "<A-O>", "mzO<esc>`z")
map("n", "J", "mzJ`z")
map("n", "<c-u>", "<c-u>zz")
map("n", "<c-d>", "<c-d>zz")
map("n", "n", "nzz")
map("n", "N", "Nzz")
map("n", "<leader>p>", [["*p]])
map("n", "<leader>P>", [["*P]])
map("n", "<leader>y>", [["+y]])
map("n", "<leader>Y>", [["+Y]])
map("n", "<c-w>t", "<cmd>vsplit | terminal<cr>")
map("n", "<c-w>T", "<cmd>split | terminal<cr>")
map("n", "<leader>r", [[:%s/\<<c-r><c-w>\>/<c-r><c-w>/gI<left><left><left>]])
map("n", "ga", "<c-^>")
map("n", "gh", "0")
map("n", "gl", "$")
map("n", "ge", "G")
map("n", "gi", "^")
map("n", "<leader>e", "<cmd>Oil<cr>")
map("v", "<leader>;", [[ "zy:<c-r>z<cr> ]])

vim.keymap.set("n", "<leader><leader>", function()
    local unpack = table.unpack
    local v = vim.api
    local bufs_id = v.nvim_list_bufs()
    local get_file = v.nvim_buf_get_name

    local function basename(path)
      return path:match("^.+/(.+)$")
    end

    local function bufname(id)
        if type(id) == "table" then
            local names = {}
            for i=1, #id do
                names[i] = basename(get_file(id[i]))
            end
            return names
        end
        return basename(get_file(id))
    end

    local function bufexists(name) 
        local bufs = bufname(bufs_id)
        for i=1, #bufs do
            if bufs[i] == name then return true end
        end
        return false
    end

    local cur_b_id = v.nvim_win_get_buf(0)
    local cur_b_name = bufname(cur_b_id)

    local bufs_id = v.nvim_list_bufs()
    local bufs = bufname(bufs_id) 
    print()
end)

require("lazy").setup({
	{
		"stevearc/oil.nvim",
		cmd = "Oil",
		config = function()
			local opts = {
				columns = {
					"icon",
					"permissions",
					"size",
					-- "mtime",
				},
				default_file_explorer = true,
				restore_win_options = true,
				skip_confirm_for_simple_edits = false,
				keymaps = {
					["g?"] = "actions.show_help",
					["<CR>"] = "actions.select",
					["<C-s>"] = "actions.select_vsplit",
					["<C-h>"] = "actions.select_split",
					["<C-t>"] = "actions.select_tab",
					["<C-p>"] = "actions.preview",
					["<C-/>"] = "actions.close",
					["<C-l>"] = "actions.refresh",
					["-"] = "actions.parent",
					["_"] = "actions.open_cwd",
					["`"] = "actions.cd",
					["~"] = "actions.tcd",
					["g."] = "actions.toggle_hidden",
				},
				view_options = {
					show_hidden = true,
				},
			}
			require("oil").setup(opts)
		end,
		lazy = false,
	},
	{ "windwp/nvim-autopairs", config = true, event = "InsertEnter" },
	{ "norcalli/nvim-colorizer.lua", cmd = "ColorizerToggle" },
	{ "rose-pine/neovim", name = "rose-pine", lazy = false },
	{ "rebelot/kanagawa.nvim", lazy = false },
	{ "ellisonleao/gruvbox.nvim", lazy = false },
})

vim.cmd("color kanagawa-lotus")
