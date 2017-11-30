if not plugin then
	return
end

local Plugin = require(script.Parent.Plugin)

local function main()
	local pluginInstance = Plugin.new()

	local toolbar = plugin:CreateToolbar("rbxfs 2.0")

	toolbar:CreateButton("Test Connection", "Connect to RBXFS Server", "")
		.Click:Connect(function()
			pluginInstance:connect()
		end)

	toolbar:CreateButton("Sync In", "Sync into Roblox Studio", "")
		.Click:Connect(function()
			pluginInstance:syncIn()
		end)

	toolbar:CreateButton("Toggle Polling", "Poll server for changes", "")
		.Click:Connect(function()
			spawn(function()
				pluginInstance:togglePolling()
			end)
		end)
end

main()
