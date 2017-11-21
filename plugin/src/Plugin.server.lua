if not plugin then
	return
end

local Http = require(script.Parent.Http)
local ServerContext = require(script.Parent.ServerContext)

local function main()
	local address = "localhost"
	local port = 8081

	local remote = ("http://%s:%d"):format(address, port)
	local http = Http.new(remote)
	local server = ServerContext.new(http)

	local toolbar = plugin:CreateToolbar("rbxfs 2.0")

	toolbar:CreateButton("Connect", "Connect to RBXFS Server", "")
		.Click:Connect(function()
			server:connect()
		end)

	toolbar:CreateButton("Get Changes", "", "")
		.Click:Connect(function()
			server:getChanges()
		end)
end

main()
