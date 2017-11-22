--[[
	What the heck do I name this?
]]

local Http = require(script.Parent.Http)
local ServerContext = require(script.Parent.ServerContext)

local function fileToName(filename)
	if filename:find("%.server%.lua$") then
		return filename:match("^(.-)%.server%.lua$"), "Script"
	elseif filename:find("%.client%.lua$") then
		return filename:match("^(.-)%.client%.lua$"), "LocalScript"
	elseif filename:find("%.lua") then
		return filename:match("^(.-)%.lua$"), "ModuleScript"
	else
		return filename, "StringValue"
	end
end

local function nameToInstance(filename, contents)
	local name, className = fileToName(filename)

	local instance = Instance.new(className)
	instance.Name = name

	if className:find("Script$") then
		instance.Source = contents
	else
		instance.Value = contents
	end

	return instance
end

local function make(item, name)
	if item.type == "dir" then
		local instance = Instance.new("Folder")
		instance.Name = name

		for childName, child in pairs(item.children) do
			make(child, childName).Parent = instance
		end

		return instance
	elseif item.type == "file" then
		return nameToInstance(name, item.contents)
	else
		error("not implemented")
	end
end

local function write(parent, route, item)
	local location = parent

	for index = 1, #route - 1 do
		local piece = route[index]
		local newLocation = location:FindFirstChild(piece)

		if not newLocation then
			newLocation = Instance.new("Folder")
			newLocation.Name = piece
			newLocation.Parent = location
		end

		location = newLocation
	end

	local fileName = route[#route]
	local name = fileToName(fileName)

	local existing = location:FindFirstChild(name)

	local new
	if item then
		new = make(item, fileName)
	end

	if existing then
		existing:Destroy()
	end

	if new then
		new.Parent = location
	end
end

local Foop = {}
Foop.__index = Foop

function Foop.new(address, port)
	local remote = ("http://%s:%d"):format(address, port)

	local foop = {
		_http = Http.new(remote),
		_server = nil,
		_polling = false,
	}

	setmetatable(foop, Foop)

	do
		local screenGui = Instance.new("ScreenGui")
		screenGui.Name = "rbxfs ui"
		screenGui.Parent = game.CoreGui
		screenGui.DisplayOrder = -1
		screenGui.Enabled = false

		local label = Instance.new("TextLabel")
		label.Font = Enum.Font.SourceSans
		label.TextSize = 20
		label.Text = "rbxfs polling"
		label.BackgroundColor3 = Color3.new(0, 0, 0)
		label.BackgroundTransparency = 0.6
		label.BorderSizePixel = 0
		label.TextColor3 = Color3.new(1, 1, 1)
		label.Size = UDim2.new(0, 120, 0, 22)
		label.Position = UDim2.new(0, 0, 0, 0)
		label.Parent = screenGui

		foop._label = screenGui
	end

	return foop
end

function Foop:server()
	if not self._server then
		self._server = ServerContext.connect(self._http, function(newServer)
			self._server = newServer
		end)
	end

	return self._server
end

function Foop:connect()
	print("Testing connection...")

	local response = self:server():ping()

	print("Server found!")
	print("Protocol version:", response.protocolVersion)
	print("Server version:", response.serverVersion)
end

function Foop:poll()
	if self._polling then
		return
	end

	print("Starting to poll...")

	self._polling = true
	self._label.Enabled = true

	while true do
		local changes = self:server():getChanges()

		local routes = {}

		for _, change in ipairs(changes) do
			table.insert(routes, change.route)
		end

		local items = self:server():read(routes)

		for index = 1, #routes do
			local route = routes[index]
			local item = items[index]

			local fullRoute = {"ReplicatedStorage"}
			for _, v in ipairs(route) do
				table.insert(fullRoute, v)
			end

			write(game, fullRoute, item)
		end

		wait(0.3)
	end
end

function Foop:syncIn()
	print("Syncing from server...")

	local response = self:server():read({{"src"}})

	write(game, {"ReplicatedStorage", "src"}, response[1])

	print("Synced successfully!")
end

return Foop