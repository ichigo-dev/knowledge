local fern = require('plugins.fern')
local fzf = require('plugins.fzf')

vim.cmd('let g:previm_open_cmd = "firefox"');


--------------------------------------------------------------------------------
-- Plugin
--------------------------------------------------------------------------------

vim.cmd('packadd vim-jetpack') require('jetpack.paq')
{
	{'tani/vim-jetpack', opt = 1},

	-- Filer(Fern)
	'lambdalisue/fern.vim',
	'lambdalisue/nerdfont.vim',
	'lambdalisue/fern-renderer-nerdfont.vim',
	'lambdalisue/glyph-palette.vim',

	-- Status line
	'nvim-lualine/lualine.nvim',
	'kyazdani42/nvim-web-devicons',

	-- FZF
	'junegunn/fzf',
	'ibhagwan/fzf-lua',

	-- LSP
	'neovim/nvim-lspconfig',
	'williamboman/nvim-lsp-installer',

	-- autocomplete
	'hrsh7th/nvim-cmp',
	'hrsh7th/cmp-vsnip',
	'hrsh7th/cmp-buffer',
	'hrsh7th/cmp-path',
	'hrsh7th/cmp-cmdline',
	'onsails/lspkind-nvim',

	-- Git
	'tpope/vim-fugitive',
	'lewis6991/gitsigns.nvim',

	-- Markdown
	'previm/previm',
}
