require("noice").setup(
{
	lsp =
	{

		override =
		{
			["vim.lsp.util.convert_input_to_markdown_lines"] = true,
			["vim.lsp.util.stylize_markdown"] = true,
			["cmp.entry.get_documentation"] = true,
		},
	},

	routes =
	{
		view = "notify",
		filter = { event = "msg_showmode" },
	},

	messages =
	{
		enabled = true,
		view = "mini",
		view_error = "mini",
		view_warn = "mini",
		view_search = false,
	},

	cmdline =
	{
		enabled = true,
		view = "cmdline"
	},

	views =
	{
		cmdline_popup =
		{
			position =
			{
				row = "35%",
				col = "50%",
			},
			size =
			{
				width = 60,
				height = "auto",
			},
		},

		popupmenu =
		{
			relative = "editor",
			position =
			{
				row = "50%",
				col = "50%",
			},
			size =
			{
				width = 60,
				height = 10,
			},
			border =
			{
				style = "rounded",
				padding = { 0, 1 },
			},
			win_options =
			{
				winhighlight = { Normal = "Normal", FloatBorder = "DiagnosticInfo" },
			},
		},
	},
})