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
vim.opt.rtp:prepend(lazypath)

vim.o.swapfile = false
vim.o.backup = false
vim.o.smartindent = true
vim.o.laststatus = true
vim.o.ignorecase = true
vim.o.tabstop = 4
vim.o.softtabstop = 4
vim.o.shiftwidth = 4
vim.o.expandtab = true
vim.o.nu = true
vim.o.wrap = false
vim.o.relativenumber = true
vim.o.scrolloff = 15
vim.o.splitbelow = true
vim.o.splitright = true
vim.o.splitkeep = true
vim.o.hlsearch = false
vim.opt.path:append("**")

vim.g.mapleader = " "
vim.keymap.set("n", "<leader>f", [[:find ]])
vim.keymap.set("n", "<leader>b", [[<cmd>ls<cr>:]])
vim.keymap.set("n", "ds", [[mz%"_x`z"_x]])
vim.keymap.set("t", "<c-/>", "<c-\\><c-n>")
vim.keymap.set("n", "<leader>h", "<cmd>noh<cr>")
vim.keymap.set("n", "x", [["_x]])
vim.keymap.set("n", "<A-o>", "mzo<esc>`z")
vim.keymap.set("n", "<A-O>", "mzO<esc>`z")
vim.keymap.set("n", "J", "mzJ`z")
vim.keymap.set("n", "<c-u>", "<c-u>zz")
vim.keymap.set("n", "<c-d>", "<c-d>zz")
vim.keymap.set("n", "n", "nzz")
vim.keymap.set("n", "N", "Nzz")
vim.keymap.set("n", "<leader>p>", [["*p]])
vim.keymap.set("n", "<leader>P>", [["*P]])
vim.keymap.set("n", "<leader>y>", [["+y]])
vim.keymap.set("n", "<leader>Y>", [["+Y]])
vim.keymap.set("n", "<c-w>t", "<cmd>vsplit | terminal<cr>")
vim.keymap.set("n", "<c-w>T", "<cmd>split | terminal<cr>")
vim.keymap.set("n", "<leader>r", [[:%s/\<<c-r><c-w>\>/<c-r><c-w>/gI<left><left><left>]])
vim.keymap.set("n", "ga", "<c-^>")
vim.keymap.set("n", "<leader>e", "Oil")

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
	{ "rose-pine/neovim", name = "rose-pine", lazy = false },
	{ "rebelot/kanagawa.nvim", lazy = false },
	{ "norcalli/nvim-colorizer.lua", cmd = "ColorizerToggle" },
})

vim.cmd("color kanagawa-lotus")
-- vim.cmd("color rose-pine-main")
