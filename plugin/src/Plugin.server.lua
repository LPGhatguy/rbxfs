if (not plugin) then
	return
end

local Net = require(script.Parent.Net)
local VFS = require(script.Parent.VFS)

local function main()
	local toolbar = plugin:CreateToolbar("RBXFS 0.3.2")

	toolbar:CreateButton("Version", "Show RBXFS server version", "")
		.Click:connect(function()
			print("Asking server for version...")
			local ok, response = Net:version()

			if ok then
				print("Found server, version", response.version)
			else
				response:report()
			end
		end)

	toolbar:CreateButton("Sync to Roblox", "Pull files from RBXFS server", "")
		.Click:connect(function()
			print("Starting sync to Roblox...")
			local ok, response = VFS:syncToRBX()

			if ok then
				print("Sync to Roblox successful!")
			else
				response:report()
			end
		end)

	local pollingButton = toolbar:CreateButton("Automatic Sync", "Set polling the RBXFS server for changes", "")
	pollingButton.Click:connect(function()
		if (VFS.polling) then
			VFS:stopPolling()
		else
			VFS:startPolling()
		end
	end)

	VFS:onPollingChanged(function(value)
		pollingButton:SetActive(value)
	end)
end

main()