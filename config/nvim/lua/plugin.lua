local fern = require('plugins.fern')

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
	'williamboman/mason.nvim',
	'williamboman/mason-lspconfig.nvim',

	-- autocomplete
	'hrsh7th/nvim-cmp',
	'hrsh7th/vim-vsnip',
	'hrsh7th/cmp-nvim-lsp',
	'hrsh7th/cmp-vsnip',
	'hrsh7th/cmp-buffer',
	'hrsh7th/cmp-path',
	'hrsh7th/cmp-cmdline',

	-- Git
	'tpope/vim-fugitive',
	'lewis6991/gitsigns.nvim',
}
