if (not plugin) then
	return
end

local HttpService = game:GetService("HttpService")

local function create(rbx, domNode)
	for name, child in pairs(domNode.children) do
		local instance = child.instance
		local childRbx = Instance.new(instance.type)

		if instance.source then
			childRbx.Source = instance.source
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
			local readAllUrl = ("%s/read-all"):format(remote)
			local result = HttpService:GetAsync(readAllUrl)

			print("download:", result)

			local value = HttpService:JSONDecode(result)

			create(game.Workspace, value.root)
		end)

	toolbar:CreateButton("Write Test", "Write (Testing)", "")
		.Click:Connect(function()
			local writeUrl = ("%s/write"):format(remote)
			local body = HttpService:JSONEncode({
				{
					path = {"test"},
					instance = {
						type = "ModuleScript",
						source = "-- hi mom"
					}
				}
			})

			local result = HttpService:PostAsync(writeUrl, body)

			print("write result:", result)
		end)
end

main()