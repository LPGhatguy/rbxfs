if (not plugin) then
	return
end

local Net = require(script.Parent.Net)
local VFS = require(script.Parent.VFS)

local function main()
	local toolbar = plugin:CreateToolbar("RBXFS Plugin")

	local button

	button = toolbar:CreateButton("Version", "Show RBXFS server version", "")
	button.Click:connect(function()
		print("Found server, version", Net:version().version)
	end)

	button = toolbar:CreateButton("Sync to Roblox", "Pull files from RBXFS server", "")
	button.Click:connect(function()
		print("Starting sync to Roblox...")
		VFS:syncToRBX()
		print("Sync to Roblox successful!")
	end)

	-- button = toolbar:CreateButton("Sync to Server", "Push files to RBXFS server", "http://www.roblox.com/asset/?id=145723965")
	-- button.Click:connect(function()
	-- 	VFS:syncToServer()
	-- end)

	button = toolbar:CreateButton("Polling", "Set polling the RBXFS server for changes", "")
	button.Click:connect(function()
		if (VFS.polling) then
			VFS:stopPolling()
			button:SetActive(false)
			print("Stopped polling server for changes.")
		else
			VFS:startPolling()
			button:SetActive(true)
			print("Started polling server for changes.")
		end
	end)
end

main()