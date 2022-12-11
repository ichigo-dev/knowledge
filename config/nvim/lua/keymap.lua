--------------------------------------------------------------------------------
-- Keymap
--------------------------------------------------------------------------------

vim.cmd('let mapleader="<Space>"')

-- util
vim.keymap.set('n', 'n', 'nzz')
vim.keymap.set('n', 'N', 'Nzz')
vim.keymap.set('n', '<Esc><Esc>', ':noh<CR>')

-- terminal
vim.keymap.set('n', '<Space>t', ':terminal ')

-- copy and paste
vim.keymap.set('n', '<Space>p', '"+p')
vim.keymap.set('n', '<Space>y', '"+y')

-- window
vim.keymap.set('n', '<Space>s', ':sp<CR>')
vim.keymap.set('n', '<Space>v', ':vs<CR>')
vim.keymap.set('n', '<Space>j', '<C-w>j')
vim.keymap.set('n', '<Space>k', '<C-w>k')
vim.keymap.set('n', '<Space>h', '<C-w>h')
vim.keymap.set('n', '<Space>l', '<C-w>l')
vim.keymap.set('n', '<Space>w', '<C-w>w')
vim.keymap.set('n', '<Space>W', '<C-w>W')
vim.keymap.set('n', '<Space>r', '<C-w>r')
vim.keymap.set('n', '<Space>R', '<C-w>R')
vim.keymap.set('n', '<Space>J', '<C-w>J')
vim.keymap.set('n', '<Space>K', '<C-w>K')
vim.keymap.set('n', '<Space>H', '<C-w>H')
vim.keymap.set('n', '<Space>L', '<C-w>L')
vim.keymap.set('n', '<Space><', '<C-w><')
vim.keymap.set('n', '<Space>>', '<C-w>>')
vim.keymap.set('n', '<Space>-', '<C-w>-')
vim.keymap.set('n', '<Space>+', '<C-w>+')

-- plugin
vim.keymap.set('n', '<Space>n', ':Fern . -drawer -toggle<CR>')
vim.keymap.set('n', '<Space>f', ':FzfLua files<CR>')
