if (not plugin) then
	return
end

local Net = require(script.Parent.Net)
local VFS = require(script.Parent.VFS)

local function main()
	local toolbar = plugin:CreateToolbar("RBXFS 0.3.0")

	toolbar:CreateButton("Version", "Show RBXFS server version", "")
		.Click:connect(function()
			local ok, response = Net:version()

			if ok then
				print("Found server, version", response.version)
			else
				warn("Couldn't connect to RBXFS server. Make sure that it's running!")
			end
		end)

	toolbar:CreateButton("Sync to Roblox", "Pull files from RBXFS server", "")
		.Click:connect(function()
			print("Starting sync to Roblox...")
			local ok = VFS:syncToRBX()

			if ok then
				print("Sync to Roblox successful!")
			else
				warn("Couldn't connect to RBXFS server. Make sure that it's running.")
			end
		end)

	local pollingButton = toolbar:CreateButton("Automatic Sync", "Set polling the RBXFS server for changes", "")
	pollingButton.Click:connect(function()
		if (VFS.polling) then
			VFS:stopPolling()
			print("Stopped polling server for changes.")
		else
			VFS:startPolling()
			print("Started polling server for changes.")
		end
	end)

	VFS:onPollingChanged(function(value)
		pollingButton:SetActive(value)
	end)
end

main()