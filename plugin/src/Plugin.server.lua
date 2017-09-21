if (not plugin) then
	return
end

local Net = require(script.Parent.Net)
local VFS = require(script.Parent.VFS)

local function main()
	local toolbar = plugin:CreateToolbar("rbxfs 2.0")

	local button

	button = toolbar:CreateButton("Show Version", "Show RBXFS server version", "")
	button.Click:connect(function()
		print("Found server, version", Net:version().version)
	end)

	button = toolbar:CreateButton("Sync to RBX", "Pull files from RBXFS server", "")
	button.Click:connect(function()
		VFS:syncToRBX()
	end)

	button = toolbar:CreateButton("Sync to Server", "Push files to RBXFS server", "")
	button.Click:connect(function()
		VFS:syncToServer()
	end)

	button = toolbar:CreateButton("Toggle Polling", "Toggle polling the RBXFS server for changes", "")
	button.Click:connect(function()
		if (VFS.polling) then
			VFS:stopPolling()
			print("Stopped polling server for changes.")
		else
			VFS:startPolling()
			print("Started polling server for changes.")
		end
	end)
end

main()