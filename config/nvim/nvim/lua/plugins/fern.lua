--------------------------------------------------------------------------------
-- Fern
--------------------------------------------------------------------------------

vim.keymap.set("n", "<C-n>", ":Fern . -reveal=% -drawer -toggle -width=30<CR>")
vim.cmd("let g:fern#default_hidden=1")
vim.cmd("let g:fern#renderer='nerdfont'")

vim.cmd("augroup fern-setting")
vim.cmd("autocmd! *")
vim.cmd("autocmd VimEnter * ++nested Fern . -drawer -width=30")
vim.cmd("autocmd FileType fern call glyph_palette#apply()")
vim.cmd("autocmd FileType nerdtree,startify call glyph_palette#apply()")
vim.cmd("augroup END")
