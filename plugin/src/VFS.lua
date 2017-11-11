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

	local isInit = leaves[#leaves] == "init"
	local leafCount = isInit and #leaves - 1 or #leaves

	for i = 1, leafCount do
		local leaf = leaves[i]
		local newCurrent = current:FindFirstChild(leaf)

		if not newCurrent then
			if i < #leaves then
				newCurrent = Instance.new("Folder")
			else
				newCurrent = Instance.new(scriptObject.type)
			end

			newCurrent.Name = leaf
			newCurrent.Parent = current
		end

		current = newCurrent
	end

	if isInit then
		-- Replace this incorrect object!
		if current.ClassName ~= scriptObject.type then
			local newCurrent = Instance.new(scriptObject.type)
			newCurrent.Parent = current.Parent
			newCurrent.Name = current.Name

			for _, child in ipairs(current:GetChildren()) do
				child.Parent = newCurrent
			end

			current:Destroy()
			current = newCurrent
		end
	end

	return current
end

function VFS:_setPolling(value)
	self.polling = value

	for listener in pairs(self._pollingListeners) do
		listener(value)
	end

	if value then
		print("Started polling server for changes.")
	else
		print("Stopped polling server for changes.")
	end
end

function VFS:startPolling()
	self:_setPolling(true)

	spawn(function()
		while (self.polling) do
			local ok, changeInfo = Net:getChangedSince(self.now)

			if not ok then
				changeInfo:report()
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
						local ok, source = Net:read(codeObject.name, scriptObject)

						if not ok then
							warn("RBXFS server disconnected mid-sync! Data may be in an odd state.")
							source:report()
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
		local ok, source = Net:read(codeObject.name, scriptObject)

		if not ok then
			return false, source
		end

		scriptObject.Source = source
	end

	return true
end

return VFS