hi clear
set termguicolors
let g:colors_name = "highlow"

lua << EOF
    local function setupLightColorscheme()
        -- Colors
        local diagnostics = {
            ["error"] = "#7c313a",
            ["hint"] = "#4d4d4d",
            ["info"] = "#2c4b86",
            ["ok"] = "#325916",
            ["warning"] = "#743d00",
        }
        local diff = {
            ["added"] = "#b9d9a8",
            ["changed"] = "#dacf96",
            ["removed"] = "#fbbabe",
            ["text"] = "#f5ede3",
        }
        local syn = {
            ["comment"] = "#555555",
            ["constant"] = "#364d76",
            ["constructor"] = "#6a431d",
            ["deprecated"] = "#4d4d4d",
            ["function"] = "#364d76",
            ["identifier"] = "#39405a",
            ["keyword"] = "#5e3e6d",
            ["number"] = "#6a431d",
            ["operator"] = "#464c61",
            ["parameter"] = "#334c7b",
            ["preproc"] = "#584d15",
            ["punctuation"] = "#4d4d4d",
            ["regex"] = "#584d15",
            ["special"] = "#584d15",
            ["statement"] = "#5c406a",
            ["string"] = "#364d76",
            ["type"] = "#6a3b56",
            ["variable"] = "#39405a",
        }
        local ui = {
            ["bg"] = "#f5ede3",
            ["bg_border"] = "#464c61",
            ["bg_dim"] = "#e3ded7",
            ["bg_gutter"] = "#eee0cc",
            ["bg_highlight"] = "#d1bda0",
            ["bg_highlight_dim"] = "#e6d6bf",
            ["bg_selected"] = "#d5cec4",
            ["bg_visual"] = "#d5bc98",
            ["fg"] = "#464c61",
            ["fg_dim"] = "#565b6d",
            ["fg_very_dim"] = "#767986",
            ["float_bg"] = "#eddcc5",
            ["float_border"] = "#565c71",
            ["float_fg"] = "#41475b",
            ["nontext"] = "#646361",
            ["pmenu_bg"] = "#e1ccae",
            ["pmenu_bg_selected"] = "#c49a5a",
            ["pmenu_fg"] = "#3b4155",
            ["pmenu_fg_selected"] = "#363f64",
            ["pmenu_scrollbar"] = "#b7b0a6",
            ["pmenu_thumb"] = "#464c61",
            ["special"] = "#5a366b",
        }
        local vcs = {
            ["added"] = "#3b562b",
            ["changed"] = "#584d15",
            ["removed"] = "#703b40",
        }

        -- Highlights
        vim.api.nvim_set_hl(0, "@constructor", { fg = syn["constructor"], bold = true, })
        vim.api.nvim_set_hl(0, "@lsp.type.comment", { link = "Comment", })
        vim.api.nvim_set_hl(0, "@lsp.type.string", { link = "@string", })
        vim.api.nvim_set_hl(0, "@parameter", { fg = syn["parameter"], })
        vim.api.nvim_set_hl(0, "@string", { fg = syn["string"], })
        vim.api.nvim_set_hl(0, "@tag", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "@tag.attribute", { fg = syn["parameter"], })
        vim.api.nvim_set_hl(0, "@tag.delimiter", { link = "Delimiter", })
        vim.api.nvim_set_hl(0, "Boolean", { link = "Constant", })
        vim.api.nvim_set_hl(0, "ColorColumn", { bg = ui.bg_selected, })
        vim.api.nvim_set_hl(0, "Comment", { fg = syn["comment"], italic = true, })
        vim.api.nvim_set_hl(0, "Conceal", { fg = ui["special"], bold = true, })
        vim.api.nvim_set_hl(0, "Constant", { fg = syn["constant"], })
        vim.api.nvim_set_hl(0, "CurSearch", { fg = ui["fg"], bg = ui.bg_highlight, bold = true, })
        vim.api.nvim_set_hl(0, "Cursor", { fg = ui["bg"], bg = ui.fg, })
        vim.api.nvim_set_hl(0, "CursorColumn", { link = "CursorLine", })
        vim.api.nvim_set_hl(0, "CursorIM", { link = "Cursor", })
        vim.api.nvim_set_hl(0, "CursorLine", { bg = ui.bg_highlight, })
        vim.api.nvim_set_hl(0, "CursorLineNr", { fg = ui["nontext"], bg = ui.bg_gutter, bold = true, })
        vim.api.nvim_set_hl(0, "Define", { link = "Type", })
        vim.api.nvim_set_hl(0, "Delimiter", { fg = syn["punctuation"], })
        vim.api.nvim_set_hl(0, "DiagnosticError", { fg = diagnostics["error"], })
        vim.api.nvim_set_hl(0, "DiagnosticHint", { fg = diagnostics["hint"], })
        vim.api.nvim_set_hl(0, "DiagnosticInfo", { fg = diagnostics["info"], })
        vim.api.nvim_set_hl(0, "DiagnosticOk", { fg = diagnostics["ok"], })
        vim.api.nvim_set_hl(0, "DiagnosticSignError", { fg = diagnostics["error"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticSignHint", { fg = diagnostics["hint"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticSignInfo", { fg = diagnostics["info"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticSignWarn", { fg = diagnostics["warning"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineError", { undercurl = true, })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineHint", { })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineInfo", { })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineWarn", { })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextError", { link = "DiagnosticError", })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextHint", { link = "DiagnosticHint", })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextInfo", { link = "DiagnosticInfo", })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextWarn", { link = "DiagnosticWarn", })
        vim.api.nvim_set_hl(0, "DiagnosticWarn", { fg = diagnostics["warning"], })
        vim.api.nvim_set_hl(0, "DiffAdd", { bg = diff.added, })
        vim.api.nvim_set_hl(0, "DiffChange", { bg = diff.changed, })
        vim.api.nvim_set_hl(0, "DiffDelete", { bg = diff.removed, })
        vim.api.nvim_set_hl(0, "DiffText", { bg = diff.text, })
        vim.api.nvim_set_hl(0, "Directory", { fg = syn["type"], })
        vim.api.nvim_set_hl(0, "EndOfBuffer", { })
        vim.api.nvim_set_hl(0, "ErrorMsg", { fg = diagnostics["error"], })
        vim.api.nvim_set_hl(0, "Float", { link = "Number", })
        vim.api.nvim_set_hl(0, "FloatBorder", { fg = ui["float_border"], bg = ui.float_bg, })
        vim.api.nvim_set_hl(0, "FloatTitle", { fg = ui["special"], bg = ui.float_bg, bold = true, })
        vim.api.nvim_set_hl(0, "FoldColumn", { fg = ui["nontext"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "Folded", { fg = ui["special"], bg = ui.bg_dim, })
        vim.api.nvim_set_hl(0, "Function", { fg = syn["function"], })
        vim.api.nvim_set_hl(0, "GitSignsAdd", { fg = vcs["added"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "GitSignsChange", { fg = vcs["changed"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "GitSignsDelete", { fg = vcs["removed"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "IblIndent", { fg = ui["bg_selected"], })
        vim.api.nvim_set_hl(0, "IblScope", { fg = ui["bg_selected"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "Identifier", { fg = syn["identifier"], })
        vim.api.nvim_set_hl(0, "Ignore", { fg = ui["nontext"], })
        vim.api.nvim_set_hl(0, "IncSearch", { fg = ui["bg"], bg = ui.fg, })
        vim.api.nvim_set_hl(0, "Include", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "IndentBlanklineChar", { fg = ui["bg_selected"], })
        vim.api.nvim_set_hl(0, "Keyword", { fg = syn["keyword"], })
        vim.api.nvim_set_hl(0, "LineNr", { fg = ui["nontext"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "MatchParen", { bg = ui.bg_highlight, bold = true, })
        vim.api.nvim_set_hl(0, "ModeMsg", { fg = diagnostics["warning"], bold = true, })
        vim.api.nvim_set_hl(0, "MoreMsg", { fg = diagnostics["info"], })
        vim.api.nvim_set_hl(0, "MsgArea", { fg = ui["fg_dim"], })
        vim.api.nvim_set_hl(0, "MsgSeparator", { bg = ui.bg_border, })
        vim.api.nvim_set_hl(0, "NonText", { fg = ui["nontext"], })
        vim.api.nvim_set_hl(0, "Normal", { fg = ui["fg"], bg = ui.bg, })
        vim.api.nvim_set_hl(0, "NormalFloat", { fg = ui["float_fg"], bg = ui.float_bg, })
        vim.api.nvim_set_hl(0, "Number", { fg = syn["number"], })
        vim.api.nvim_set_hl(0, "Operator", { fg = syn["operator"], })
        vim.api.nvim_set_hl(0, "Pmenu", { fg = ui["pmenu_fg"], bg = ui.pmenu_bg, })
        vim.api.nvim_set_hl(0, "PmenuSbar", { bg = ui.pmenu_scrollbar, })
        vim.api.nvim_set_hl(0, "PmenuSel", { fg = ui["pmenu_fg_selected"], bg = ui.pmenu_bg_selected, })
        vim.api.nvim_set_hl(0, "PmenuThumb", { bg = ui.pmenu_thumb, })
        vim.api.nvim_set_hl(0, "PreProc", { fg = syn["preproc"], })
        vim.api.nvim_set_hl(0, "Question", { link = "MoreMsg", })
        vim.api.nvim_set_hl(0, "QuickFixLine", { bg = ui.bg_selected, })
        vim.api.nvim_set_hl(0, "Search", { fg = ui["fg"], bg = ui.bg_highlight_dim, })
        vim.api.nvim_set_hl(0, "SignColumn", { fg = ui["special"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "Special", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "SpecialKey", { fg = ui["special"], })
        vim.api.nvim_set_hl(0, "Statement", { fg = syn["statement"], bold = true, })
        vim.api.nvim_set_hl(0, "StatusLine", { fg = ui["bg"], bg = ui.fg, })
        vim.api.nvim_set_hl(0, "StatusLineNC", { fg = ui["bg"], bg = ui.fg_very_dim, })
        vim.api.nvim_set_hl(0, "Substitute", { fg = ui["fg"], bg = diff.removed, })
        vim.api.nvim_set_hl(0, "Title", { fg = syn["function"], bold = true, })
        vim.api.nvim_set_hl(0, "Type", { fg = syn["type"], bold = true, })
        vim.api.nvim_set_hl(0, "Underlined", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "VertSplit", { link = "WinSeparator", })
        vim.api.nvim_set_hl(0, "Visual", { bg = ui.bg_visual, })
        vim.api.nvim_set_hl(0, "VisualNOS", { link = "Visual", })
        vim.api.nvim_set_hl(0, "Whitespace", { fg = ui["nontext"], })
        vim.api.nvim_set_hl(0, "WinSeparator", { fg = ui["bg_border"], bg = ui.bg_border, })
        vim.api.nvim_set_hl(0, "diffAdded", { fg = vcs["added"], })
        vim.api.nvim_set_hl(0, "diffChanged", { fg = vcs["changed"], })
        vim.api.nvim_set_hl(0, "diffDeleted", { fg = vcs["removed"], })
        vim.api.nvim_set_hl(0, "diffNewFile", { fg = vcs["added"], })
        vim.api.nvim_set_hl(0, "diffOldFile", { fg = vcs["removed"], })
        vim.api.nvim_set_hl(0, "diffRemoved", { fg = vcs["removed"], })
        vim.api.nvim_set_hl(0, "gitcommitOverflow", { fg = diagnostics["error"], italic = true, })
        vim.api.nvim_set_hl(0, "lCursor", { link = "Cursor", })
    end

    local function setupDarkColorscheme()
        -- Colors
        local diagnostics = {
            ["error"] = "#df8d93",
            ["hint"] = "#a7a7a7",
            ["info"] = "#85a8e7",
            ["ok"] = "#8cb675",
            ["warning"] = "#d59964",
        }
        local diff = {
            ["added"] = "#1d350d",
            ["changed"] = "#372d00",
            ["removed"] = "#4b1d22",
            ["text"] = "#14110c",
        }
        local syn = {
            ["comment"] = "#9f9f9f",
            ["constant"] = "#90a8d4",
            ["constructor"] = "#c89e7a",
            ["deprecated"] = "#a7a7a7",
            ["function"] = "#90a8d4",
            ["identifier"] = "#aab3ce",
            ["keyword"] = "#ba99cb",
            ["number"] = "#c89e7a",
            ["operator"] = "#a1a7bb",
            ["parameter"] = "#8ca8da",
            ["preproc"] = "#b3a875",
            ["punctuation"] = "#a7a7a7",
            ["regex"] = "#b3a875",
            ["special"] = "#b3a875",
            ["statement"] = "#b89bc7",
            ["string"] = "#90a8d4",
            ["type"] = "#c996b1",
            ["variable"] = "#aab3ce",
        }
        local ui = {
            ["bg"] = "#14110c",
            ["bg_border"] = "#a1a7bb",
            ["bg_dim"] = "#201e1c",
            ["bg_gutter"] = "#231b0e",
            ["bg_highlight"] = "#463721",
            ["bg_highlight_dim"] = "#2d2213",
            ["bg_selected"] = "#2f2c26",
            ["bg_visual"] = "#483619",
            ["fg"] = "#a1a7bb",
            ["fg_dim"] = "#9398a8",
            ["fg_very_dim"] = "#777a83",
            ["float_bg"] = "#271d0d",
            ["float_border"] = "#9096aa",
            ["float_fg"] = "#a7adc1",
            ["nontext"] = "#919191",
            ["pmenu_bg"] = "#382913",
            ["pmenu_bg_selected"] = "#704c05",
            ["pmenu_fg"] = "#adb3c7",
            ["pmenu_fg_selected"] = "#a6b2db",
            ["pmenu_scrollbar"] = "#4a4640",
            ["pmenu_thumb"] = "#a1a7bb",
            ["special"] = "#c29dd6",
        }
        local vcs = {
            ["added"] = "#95b286",
            ["changed"] = "#b3a875",
            ["removed"] = "#d09699",
        }

        -- Highlights
        vim.api.nvim_set_hl(0, "@constructor", { fg = syn["constructor"], bold = true, })
        vim.api.nvim_set_hl(0, "@lsp.type.comment", { link = "Comment", })
        vim.api.nvim_set_hl(0, "@lsp.type.string", { link = "@string", })
        vim.api.nvim_set_hl(0, "@parameter", { fg = syn["parameter"], })
        vim.api.nvim_set_hl(0, "@string", { fg = syn["string"], })
        vim.api.nvim_set_hl(0, "@tag", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "@tag.attribute", { fg = syn["parameter"], })
        vim.api.nvim_set_hl(0, "@tag.delimiter", { link = "Delimiter", })
        vim.api.nvim_set_hl(0, "Boolean", { link = "Constant", })
        vim.api.nvim_set_hl(0, "ColorColumn", { bg = ui.bg_selected, })
        vim.api.nvim_set_hl(0, "Comment", { fg = syn["comment"], italic = true, })
        vim.api.nvim_set_hl(0, "Conceal", { fg = ui["special"], bold = true, })
        vim.api.nvim_set_hl(0, "Constant", { fg = syn["constant"], })
        vim.api.nvim_set_hl(0, "CurSearch", { fg = ui["fg"], bg = ui.bg_highlight, bold = true, })
        vim.api.nvim_set_hl(0, "Cursor", { fg = ui["bg"], bg = ui.fg, })
        vim.api.nvim_set_hl(0, "CursorColumn", { link = "CursorLine", })
        vim.api.nvim_set_hl(0, "CursorIM", { link = "Cursor", })
        vim.api.nvim_set_hl(0, "CursorLine", { bg = ui.bg_highlight, })
        vim.api.nvim_set_hl(0, "CursorLineNr", { fg = ui["nontext"], bg = ui.bg_gutter, bold = true, })
        vim.api.nvim_set_hl(0, "Define", { link = "Type", })
        vim.api.nvim_set_hl(0, "Delimiter", { fg = syn["punctuation"], })
        vim.api.nvim_set_hl(0, "DiagnosticError", { fg = diagnostics["error"], })
        vim.api.nvim_set_hl(0, "DiagnosticHint", { fg = diagnostics["hint"], })
        vim.api.nvim_set_hl(0, "DiagnosticInfo", { fg = diagnostics["info"], })
        vim.api.nvim_set_hl(0, "DiagnosticOk", { fg = diagnostics["ok"], })
        vim.api.nvim_set_hl(0, "DiagnosticSignError", { fg = diagnostics["error"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticSignHint", { fg = diagnostics["hint"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticSignInfo", { fg = diagnostics["info"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticSignWarn", { fg = diagnostics["warning"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineError", { undercurl = true, })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineHint", { })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineInfo", { })
        vim.api.nvim_set_hl(0, "DiagnosticUnderlineWarn", { })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextError", { link = "DiagnosticError", })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextHint", { link = "DiagnosticHint", })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextInfo", { link = "DiagnosticInfo", })
        vim.api.nvim_set_hl(0, "DiagnosticVirtualTextWarn", { link = "DiagnosticWarn", })
        vim.api.nvim_set_hl(0, "DiagnosticWarn", { fg = diagnostics["warning"], })
        vim.api.nvim_set_hl(0, "DiffAdd", { bg = diff.added, })
        vim.api.nvim_set_hl(0, "DiffChange", { bg = diff.changed, })
        vim.api.nvim_set_hl(0, "DiffDelete", { bg = diff.removed, })
        vim.api.nvim_set_hl(0, "DiffText", { bg = diff.text, })
        vim.api.nvim_set_hl(0, "Directory", { fg = syn["type"], })
        vim.api.nvim_set_hl(0, "EndOfBuffer", { })
        vim.api.nvim_set_hl(0, "ErrorMsg", { fg = diagnostics["error"], })
        vim.api.nvim_set_hl(0, "Float", { link = "Number", })
        vim.api.nvim_set_hl(0, "FloatBorder", { fg = ui["float_border"], bg = ui.float_bg, })
        vim.api.nvim_set_hl(0, "FloatTitle", { fg = ui["special"], bg = ui.float_bg, bold = true, })
        vim.api.nvim_set_hl(0, "FoldColumn", { fg = ui["nontext"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "Folded", { fg = ui["special"], bg = ui.bg_dim, })
        vim.api.nvim_set_hl(0, "Function", { fg = syn["function"], })
        vim.api.nvim_set_hl(0, "GitSignsAdd", { fg = vcs["added"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "GitSignsChange", { fg = vcs["changed"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "GitSignsDelete", { fg = vcs["removed"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "IblIndent", { fg = ui["bg_selected"], })
        vim.api.nvim_set_hl(0, "IblScope", { fg = ui["bg_selected"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "Identifier", { fg = syn["identifier"], })
        vim.api.nvim_set_hl(0, "Ignore", { fg = ui["nontext"], })
        vim.api.nvim_set_hl(0, "IncSearch", { fg = ui["bg"], bg = ui.fg, })
        vim.api.nvim_set_hl(0, "Include", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "IndentBlanklineChar", { fg = ui["bg_selected"], })
        vim.api.nvim_set_hl(0, "Keyword", { fg = syn["keyword"], })
        vim.api.nvim_set_hl(0, "LineNr", { fg = ui["nontext"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "MatchParen", { bg = ui.bg_highlight, bold = true, })
        vim.api.nvim_set_hl(0, "ModeMsg", { fg = diagnostics["warning"], bold = true, })
        vim.api.nvim_set_hl(0, "MoreMsg", { fg = diagnostics["info"], })
        vim.api.nvim_set_hl(0, "MsgArea", { fg = ui["fg_dim"], })
        vim.api.nvim_set_hl(0, "MsgSeparator", { bg = ui.bg_border, })
        vim.api.nvim_set_hl(0, "NonText", { fg = ui["nontext"], })
        vim.api.nvim_set_hl(0, "Normal", { fg = ui["fg"], bg = ui.bg, })
        vim.api.nvim_set_hl(0, "NormalFloat", { fg = ui["float_fg"], bg = ui.float_bg, })
        vim.api.nvim_set_hl(0, "Number", { fg = syn["number"], })
        vim.api.nvim_set_hl(0, "Operator", { fg = syn["operator"], })
        vim.api.nvim_set_hl(0, "Pmenu", { fg = ui["pmenu_fg"], bg = ui.pmenu_bg, })
        vim.api.nvim_set_hl(0, "PmenuSbar", { bg = ui.pmenu_scrollbar, })
        vim.api.nvim_set_hl(0, "PmenuSel", { fg = ui["pmenu_fg_selected"], bg = ui.pmenu_bg_selected, })
        vim.api.nvim_set_hl(0, "PmenuThumb", { bg = ui.pmenu_thumb, })
        vim.api.nvim_set_hl(0, "PreProc", { fg = syn["preproc"], })
        vim.api.nvim_set_hl(0, "Question", { link = "MoreMsg", })
        vim.api.nvim_set_hl(0, "QuickFixLine", { bg = ui.bg_selected, })
        vim.api.nvim_set_hl(0, "Search", { fg = ui["fg"], bg = ui.bg_highlight_dim, })
        vim.api.nvim_set_hl(0, "SignColumn", { fg = ui["special"], bg = ui.bg_gutter, })
        vim.api.nvim_set_hl(0, "Special", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "SpecialKey", { fg = ui["special"], })
        vim.api.nvim_set_hl(0, "Statement", { fg = syn["statement"], bold = true, })
        vim.api.nvim_set_hl(0, "StatusLine", { fg = ui["bg"], bg = ui.fg, })
        vim.api.nvim_set_hl(0, "StatusLineNC", { fg = ui["bg"], bg = ui.fg_very_dim, })
        vim.api.nvim_set_hl(0, "Substitute", { fg = ui["fg"], bg = diff.removed, })
        vim.api.nvim_set_hl(0, "Title", { fg = syn["function"], bold = true, })
        vim.api.nvim_set_hl(0, "Type", { fg = syn["type"], bold = true, })
        vim.api.nvim_set_hl(0, "Underlined", { fg = syn["special"], })
        vim.api.nvim_set_hl(0, "VertSplit", { link = "WinSeparator", })
        vim.api.nvim_set_hl(0, "Visual", { bg = ui.bg_visual, })
        vim.api.nvim_set_hl(0, "VisualNOS", { link = "Visual", })
        vim.api.nvim_set_hl(0, "Whitespace", { fg = ui["nontext"], })
        vim.api.nvim_set_hl(0, "WinSeparator", { fg = ui["bg_border"], bg = ui.bg_border, })
        vim.api.nvim_set_hl(0, "diffAdded", { fg = vcs["added"], })
        vim.api.nvim_set_hl(0, "diffChanged", { fg = vcs["changed"], })
        vim.api.nvim_set_hl(0, "diffDeleted", { fg = vcs["removed"], })
        vim.api.nvim_set_hl(0, "diffNewFile", { fg = vcs["added"], })
        vim.api.nvim_set_hl(0, "diffOldFile", { fg = vcs["removed"], })
        vim.api.nvim_set_hl(0, "diffRemoved", { fg = vcs["removed"], })
        vim.api.nvim_set_hl(0, "gitcommitOverflow", { fg = diagnostics["error"], italic = true, })
        vim.api.nvim_set_hl(0, "lCursor", { link = "Cursor", })
    end

    if vim.o.background == "dark" then
        setupDarkColorscheme()
    else
        setupLightColorscheme()
    end
EOF
