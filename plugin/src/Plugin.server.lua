if not plugin then
	return
end

local HttpService = game:GetService("HttpService")

local function create(rbx, instance)
	for name, child in pairs(instance.children) do
		local details = child.details
		local childRbx = Instance.new(details.type)

		if details.source then
			childRbx.Source = details.source
		end

		childRbx.Name = name
		childRbx.Parent = rbx

		create(childRbx, child)
	end
end

local function main()
	local address = "localhost"
	local port = 8001

	local remote = ("http://%s:%d/fs"):format(address, port)

	local toolbar = plugin:CreateToolbar("rbxfs 2.0")

	toolbar:CreateButton("Connect", "Connect to RBXFS Instance", "")
		.Click:Connect(function()
			local infoUrl = ("%s/info"):format(remote)
			local result = HttpService:JSONDecode(HttpService:GetAsync(infoUrl))

			print(("Connected!\nServer version: %s\nProtocol version: %s"):format(
				result.server_version,
				result.protocol_version
			))
		end)

	toolbar:CreateButton("Download", "Download (Testing)", "")
		.Click:Connect(function()
			print("Downloading...")

			local readAllUrl = ("%s/read"):format(remote)
			local result = HttpService:GetAsync(readAllUrl)

			local value = HttpService:JSONDecode(result)

			create(game.Workspace, value.instance)

			print("Downloaded from rbxfs server!")
		end)
end

main()