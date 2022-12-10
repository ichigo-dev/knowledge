vim.cmd([[
	set guifont=Consolas:h12
]])

--------------------------------------------------------------------------------
-- Character set
--------------------------------------------------------------------------------

-- file
vim.o.encoding='utf-8'
vim.o.fileencodings='utf-8,enc-jp,iso-20220-jp'

-- LF(Line Feed), CR(Carriage Return)
vim.o.fileformats='unix,mac,dos'


--------------------------------------------------------------------------------
-- History
--------------------------------------------------------------------------------

vim.o.history=100
vim.o.undofile=true


--------------------------------------------------------------------------------
-- Display
--------------------------------------------------------------------------------

vim.o.t_Co=256
vim.o.signcolumn='yes'
vim.o.updatetime=100
vim.o.number=true
vim.o.title=true
vim.o.list=true
vim.o.listchars='tab:»-,extends:»,precedes:«,nbsp:%'

-- command line
vim.o.showcmd=true
vim.o.wildmenu=true


--------------------------------------------------------------------------------
-- Cursor
--------------------------------------------------------------------------------

vim.o.cursorline=true
vim.o.whichwrap='b,s,<,>,~,[,]'
vim.o.virtualedit='onemore'
vim.o.mouse='a'


--------------------------------------------------------------------------------
-- Edit
--------------------------------------------------------------------------------

vim.o.autoread=true
vim.o.hidden=true
vim.o.confirm=true
vim.o.backup=false
vim.o.swapfile=false


--------------------------------------------------------------------------------
-- Indent
--------------------------------------------------------------------------------

vim.o.autoindent=true
vim.o.expandtab=false
vim.o.tabstop=4
vim.o.shiftwidth=4
vim.o.smarttab=4


--------------------------------------------------------------------------------
-- Search
--------------------------------------------------------------------------------

vim.o.hlsearch=true
vim.o.wrapscan=true
vim.o.ignorecase=true
vim.o.smartcase=true
vim.o.incsearch=true


--------------------------------------------------------------------------------
-- Keymap
--------------------------------------------------------------------------------

vim.o.cp=false
vim.o.backspace='indent,eol,start'
