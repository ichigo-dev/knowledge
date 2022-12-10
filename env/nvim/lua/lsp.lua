--------------------------------------------------------------------------------
-- Build in LSP
--------------------------------------------------------------------------------


-- LSP handler
vim.lsp.handlers["textDocument/publishDiagnostics"] = vim.lsp.with(
	vim.lsp.diagnostic.on_publish_diagnostics, { virtual_text = false }
)


-- complete

vim.opt.completeopt = 'menu,menuone,noselect'