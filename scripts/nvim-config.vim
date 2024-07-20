set number
syntax enable

lua << EOF
require("nvim-treesitter.configs").setup({
  highlight = {
    enable = true,
    disable = { },
  }
})

local lspconfig = require("lspconfig")
lspconfig.rust_analyzer.setup {
  settings = {
    ["rust-analyzer"] = {},
  },
}

require("trouble").setup {}

require("nvim-tree").setup({
  renderer = {
    icons = {
     show = {
        git = false,
      }
    }
  }
})
EOF
