local capabilities = require('cmp_nvim_lsp').default_capabilities()


-- Mason
local lspconfig = require('lspconfig')
require('mason').setup()
require('mason-lspconfig').setup_handlers(
{
	function(server)
		local opt =
		{
			capabilities = capabilities,
			root_dir = lspconfig.util.root_pattern('.git'),
		}
		lspconfig[server].setup(opt)
	end
})

vim.o.completeopt = "menu,menuone,noselect"


-- Auto complete
local cmp = require("cmp")
cmp.setup(
{
	snippet =
	{
		expand = function(args)
			vim.fn["vsnip#anonymous"](args.body)
		end,
	},
	sources = cmp.config.sources(
	{
		{ name = 'nvim_lsp' },
		{ name = "vsnip" },
		{ name = "buffer" },
		{ name = "path" },
		-- { name = "cmdline" },
	}),
	mapping = cmp.mapping.preset.insert(
	{
		["<C-p>"] = cmp.mapping.select_prev_item(),
		["<C-n>"] = cmp.mapping.select_next_item(),
		['<C-l>'] = cmp.mapping.complete(),
		['<C-e>'] = cmp.mapping.abort(),
	}),
	experimental =
	{
		ghost_text = true,
	},
})
