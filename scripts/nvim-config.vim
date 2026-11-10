set number
syntax enable

lua << EOF
local function treesitter_start(buf)
  pcall(vim.treesitter.start, buf)
end
vim.api.nvim_create_autocmd("FileType", {
  callback = function(ev) treesitter_start(ev.buf) end,
})
for _, buf in ipairs(vim.api.nvim_list_bufs()) do
  if vim.api.nvim_buf_is_loaded(buf) then
    treesitter_start(buf)
  end
end

vim.lsp.config("rust_analyzer", {
  settings = {
    ["rust-analyzer"] = {},
  },
})

require("trouble").setup {}

require("nvim-tree").setup({
  filters = {
    dotfiles = true,
  },
  renderer = {
    icons = {
     show = {
        git = false,
      }
    }
  }
})
EOF
