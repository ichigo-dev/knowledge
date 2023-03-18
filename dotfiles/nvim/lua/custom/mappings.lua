---@type MappingsTable
local M = {}

M.general = {
  n = {
    -- window
    ["<leader>s"] = { ":sp<Cr>" },
    ["<leader>v"] = { ":vs<Cr>" },
    ["<leader>h"] = { "<C-w>h", "move left" },
    ["<leader>l"] = { "<C-w>l", "move right" },
    ["<leader>j"] = { "<C-w>j", "move down" },
    ["<leader>k"] = { "<C-w>k", "move up" },
  },
}

M.nvimtree = {
  n = {
    ["<leader>n"] = { "<cmd> NvimTreeToggle <CR>", "toggle nvimtree" },
  },
}

M.nvterm = {
  n = {
    ["<leader>t"] = {
      function()
        require("nvterm.terminal").new "horizontal"
      end,
      "toggle floating term",
    },
  },
}

M.telescope = {
  n = {
    ["<leader>f"] = { "<cmd> FzfLua files <CR>", "FZF files" },
    ["<leader>g"] = { "<cmd> FzfLua live_grep <CR>", "FZF grep" },
  },
}

M.markdown_preview = {
  n = {
    ["<leader>m"] = { "<cmd> MarkdownPreview <CR>", "Markdown Preview" },
  },
}

return M
