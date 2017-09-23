if (not plugin) then
	return
end

local HttpService = game:GetService("HttpService")

local function main()
	local address = "localhost"
	local port = 8001

	local remote = ("http://%s:%d/rbxfs"):format(address, port)

	local toolbar = plugin:CreateToolbar("rbxfs 2.0")

	toolbar:CreateButton("Connect", "Connect to RBXFS Instance", "")
		.Click:Connect(function()
			local result = HttpService:JSONDecode(HttpService:GetAsync(remote))

			print(("Connected!\nServer version: %s\nProtocol version: %s"):format(
				result.server_version,
				result.protocol_version
			))
		end)
end

main()