--------------------------------------------------------------------------------
-- Vim config (Lua)
--------------------------------------------------------------------------------

local option = require("option")
local theme = require("theme")
local plugin = require("plugin")
local keymap = require("keymap")
local lsp = require("lsp")

-- gitsigns
require("gitsigns").setup()
