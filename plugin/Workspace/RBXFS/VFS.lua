local Net = require(script.Parent.Net)

local VFS = {
	polling = false,
	pollingRate = 0.25,
	now = 0
}

function VFS:getRBX(scriptObject)
	local current = game

	local leaves = {}

	for leaf in scriptObject.name:gmatch("[^.]+") do
		table.insert(leaves, leaf)
	end

	for i = 1, #leaves do
		local leaf = leaves[i]
		local newCurrent = current:FindFirstChild(leaf)

		if (not newCurrent) then
			if (i < #leaves) then
				newCurrent = Instance.new("Folder")
			else
				newCurrent = Instance.new(scriptObject.type)
			end

			newCurrent.Name = leaf
			newCurrent.Parent = current
		end

		current = newCurrent
	end

	return current
end

function VFS:startPolling()
	self.polling = true

	spawn(function()
		while (self.polling) do
			wait(0.25)

			local changeInfo = Net:getChangedSince(self.now)
			local changed = changeInfo.changed
			self.now = changeInfo.now

			if (#changed > 0) then
				for key, changeEvent in ipairs(changed) do
					local codeObject = changeEvent.object
					local scriptObject = self:getRBX(codeObject)

					if (changeEvent.type == "change") then
						local source = Net:read(scriptObject)

						scriptObject.Source = source
					elseif (changeEvent.type == "delete") then
						scriptObject:Destroy()
					end
				end
			end
		end
	end)
end

function VFS:stopPolling()
	self.polling = false
end

function VFS:syncToRBX()
	local list = Net:list()

	for key, codeObject in ipairs(list.files) do
		local scriptObject = self:getRBX(codeObject)
		local source = Net:read(scriptObject)
		scriptObject.Source = source
	end
end

function VFS:syncToServer()
	print("TODO: sync to server")
end

return VFS