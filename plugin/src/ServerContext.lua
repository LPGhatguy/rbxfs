local HttpService = game:GetService("HttpService")

local ServerContext = {}
ServerContext.__index = ServerContext

ServerContext.Replaced = newproxy(true)

--[[
	Create a new ServerContext using the given HTTP implementation and replacer.

	If the context becomes invalid, `replacer` will be invoked with a new
	context that should be suitable to replace this one.

	Attempting to invoke methods on an invalid conext will throw errors!
]]
function ServerContext.connect(http, replacer)
	local context = {
		http = http,
		serverId = nil,
		currentTime = 0,
		project = nil,
		valid = true,
		replacer = replacer,
	}

	setmetatable(context, ServerContext)

	context:_start():await()

	return context
end

function ServerContext:_start()
	return self.http:get("/")
		:andThen(function(response)
			response = response:json()

			self.serverId = response.serverId
			self.currentTime = response.currentTime
			self.project = response.project
		end)
end

function ServerContext:_validate()
	if not self.valid then
		error("ServerContext is no longer valid!", 3)
	end
end

function ServerContext:_validateResponse(response)
	if response.serverId ~= self.serverId then
		warn("Got different server, marking context as invalid...")
		self.valid = false

		if self.replacer then
			self.replacer(ServerContext.connect(self.http, self.replacer))
		end

		return false
	end

	return true
end

function ServerContext:ping()
	self:_validate()

	return self.http:get("/")
		:andThen(function(response)
			response = response:json()

			if not self:_validateResponse(response) then
				return ServerContext.Replaced
			end

			return response
		end)
end

function ServerContext:read(paths)
	self:_validate()

	local body = HttpService:JSONEncode(paths)

	return self.http:post("/read", body)
		:andThen(function(response)
			response = response:json()

			if not self:_validateResponse(response) then
				return ServerContext.Replaced
			end

			return response.items
		end)
end

function ServerContext:getChanges()
	self:_validate()

	local url = ("/changes/%f"):format(self.currentTime)

	return self.http:get(url)
		:andThen(function(response)
			response = response:json()

			if not self:_validateResponse(response) then
				return ServerContext.Replaced
			end

			self.currentTime = response.currentTime

			return response.changes
		end)
end

return ServerContext
