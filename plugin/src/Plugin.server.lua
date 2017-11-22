if not plugin then
	return
end

local Foop = require(script.Parent.Foop)

local function main()
	local address = "localhost"
	local port = 8081

	local foop = Foop.new(address, port)

	local toolbar = plugin:CreateToolbar("rbxfs 2.0")

	toolbar:CreateButton("Connect", "Connect to RBXFS Server", "")
		.Click:Connect(function()
			foop:connect()
		end)

	toolbar:CreateButton("Sync In", "Sync into Roblox Studio", "")
		.Click:Connect(function()
			foop:syncIn()
		end)

	toolbar:CreateButton("Poll", "Poll server for changes", "")
		.Click:Connect(function()
			spawn(function()
				foop:poll()
			end)
		end)
end

main()
