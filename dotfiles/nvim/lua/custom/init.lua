local opt = vim.opt
local diagnostic = vim.diagnostic

-- Indenting
opt.expandtab =false
opt.shiftwidth = 4
opt.smartindent = true
opt.tabstop = 4
opt.softtabstop = 4

-- Other
opt.list = true
opt.listchars="tab:»-,extends:»,precedes:«,nbsp:%"
opt.virtualedit="onemore"
opt.whichwrap="b,s,<,>,~,[,]"

-- Diagnostic
diagnostic.config(
{
  virtual_text = false,
})

-- Highlight
vim.api.nvim_set_hl(0, "CmpDoc", { bg="gray20" })

-------------------------------------- autocmds ------------------------------------------
local autocmd = vim.api.nvim_create_autocmd

autocmd("VimEnter", {
  command = "NvimTreeToggle <CR>",
})
