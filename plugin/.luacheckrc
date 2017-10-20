stds.roblox = {
	globals = {
		"game",
	},
	read_globals = {
		"plugin", "script",

		"warn", "spawn", "wait",

		"Instance",
	},
}

ignore = {
	"212", -- unused arguments
	"421", -- shadowing local variable
	"431", -- shadowing upvalue
	"432", -- shadowing upvalue argument
}

std = "lua51+roblox"
