local Net = require(script.Parent.Net)

local VFS = {
	polling = false,
	pollingRate = 0.1,
	now = 0,
	_pollingListeners = {}
}

function VFS:onPollingChanged(callback)
	self._pollingListeners[callback] = true
end

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

function VFS:_setPolling(value)
	self.polling = value

	for listener in pairs(self._pollingListeners) do
		listener(value)
	end
end

function VFS:startPolling()
	self:_setPolling(true)

	spawn(function()
		while (self.polling) do
			local ok, changeInfo = Net:getChangedSince(self.now)

			if not ok then
				warn("Couldn't connect to RBXFS server. Make sure that it's running.")
				self:_setPolling(false)
				return
			end

			local changed = changeInfo.changed
			self.now = changeInfo.now

			if (#changed > 0) then
				for _, changeEvent in ipairs(changed) do
					local codeObject = changeEvent.object
					local scriptObject = self:getRBX(codeObject)

					if (changeEvent.type == "change") then
						local ok, source = Net:read(scriptObject)

						if not ok then
							warn("RBXFS server disconnected mid-change.")
							self:_setPolling(false)
							return
						end

						scriptObject.Source = source
					elseif (changeEvent.type == "delete") then
						scriptObject:Destroy()
					end
				end
			end

			wait(self.pollingRate)
		end
	end)
end

function VFS:stopPolling()
	self:_setPolling(false)
end

function VFS:syncToRBX()
	local ok, list = Net:list()

	if not ok then
		return ok, list
	end

	for _, codeObject in ipairs(list.files) do
		local scriptObject = self:getRBX(codeObject)
		local ok, source = Net:read(scriptObject)

		if not ok then
			warn("Sync to Roblox failed mid-read!")
			return false, source
		end

		scriptObject.Source = source
	end

	return true
end

return VFS