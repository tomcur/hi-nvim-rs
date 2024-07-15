set number
syntax enable

lua << EOF
require("nvim-treesitter.configs").setup({
  highlight = {
    enable = true,
    disable = { },
  }
})
EOF
