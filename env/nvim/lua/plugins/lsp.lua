-- Mason
local nvim_lsp = require('lspconfig')
require('mason').setup()
require('mason-lspconfig').setup_handlers(
{
	function(server)
		local opts =
		{
			capabilities = require('cmp_nvim_lsp').default_capabilities(
				vim.lsp.protocol.make_client_capabilities()
			)
		}
		nvim_lsp[server].setup(opts)
	end
})


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
	sources =
	{
		{ name = "nvim_lsp" },
		{ name = "buffer" },
		{ name = "path" },
	},
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
