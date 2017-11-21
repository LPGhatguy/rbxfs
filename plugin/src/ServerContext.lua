local ServerContext = {}
ServerContext.__index = ServerContext

function ServerContext.new(http)
	local context = {
		http = http,
		serverId = nil,
		currentTime = 0,
		project = nil,
	}

	setmetatable(context, ServerContext)

	return context
end

function ServerContext:connect()
	self.http
		:get("/")
		:andThen(function(response)
			response = response:json()

			self.serverId = response.serverId
			self.currentTime = response.currentTime
			self.project = response.project

			print(("Connected!\nServer version: %s\nProtocol version: %s"):format(
				response.serverVersion,
				response.protocolVersion
			))
		end)
end

function ServerContext:getChanges()
	local url = ("/changes/%f"):format(self.currentTime)

	self.http
		:get(url)
		:andThen(function(response)
			print("got changes:", response.body)

			response = response:json()

			if response.serverId ~= self.serverId then
				-- Abort! This is a new server!
				error("Not yet implemented: server switching!")
			end

			self.currentTime = response.currentTime
		end)
end

return ServerContext
