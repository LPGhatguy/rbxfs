local HttpService = game:GetService("HttpService")

local Net = {
	baseUrl = "http://localhost:8000/",
}

Net.Error = {
	HttpNotEnabled = {
		message = "RBXFS requires HTTP access, which is not enabled.\n" ..
			"Check your game settings.",
	},
	ConnectFailed = {
		message = "RBXFS plugin couldn't connect to the RBXFS server.\n" ..
			"Make sure the server is running!",
	},
	Unknown = {
		message = "RBXFS encountered unknown error: {{message}}",
	},
}

local ErrorType = {}

function ErrorType:report()
	warn(self.message)
end

local function makeError(type, extraMessage)
	extraMessage = extraMessage or ""
	local message = type.message:gsub("{{message}}", extraMessage)

	local err = {
		type = type,
		message = message,
	}

	setmetatable(err, {
		__tostring = function(self)
			return self.message
		end,
		__index = ErrorType,
	})

	return err
end

local function getNetName(scriptObject)
	local fullName = scriptObject:GetFullName()

	return (fullName:gsub("^game%.", ""))
end

--[[
	This is so horrible.

	I don't want to do this, I don't understand why I have to do this.
]]
local function wrapErrorString(err)
	err = err:lower()

	if err:find("^http requests are not enabled") then
		return makeError(Net.Error.HttpNotEnabled)
	end

	if err:find("^curl error") then
		return makeError(Net.Error.ConnectFailed)
	end

	return makeError(Net.Error.Unknown, err)
end

function Net:setBaseUrl(baseUrl)
	self.baseUrl = baseUrl
end

function Net:get(endpoint)
	local ok, result = pcall(function()
		return HttpService:GetAsync(self.baseUrl .. endpoint, true)
	end)

	if not ok then
		result = wrapErrorString(result)
	end

	return ok, result
end

function Net:post(endpoint, body)
	local ok, result = pcall(function()
		return HttpService:PostAsync(self.baseUrl .. endpoint, body)
	end)

	if not ok then
		result = wrapErrorString(result)
	end

	return ok, result
end

function Net:version()
	local ok, response = self:get("version")

	if not ok then
		return ok, response
	end

	return true, HttpService:JSONDecode(response)
end

function Net:list()
	local ok, response = self:get("files")

	if not ok then
		return ok, response
	end

	return true, HttpService:JSONDecode(response)
end

function Net:read(scriptObject)
	local url = ("files/%s/%s"):format(
		getNetName(scriptObject),
		scriptObject.ClassName
	)

	return self:get(url)
end

function Net:write(scriptObject)
	local url = ("files/%s/%s"):format(
		getNetName(scriptObject),
		scriptObject.ClassName
	)

	return self:post(url, scriptObject.Source)
end

function Net:getChangedSince(timestamp)
	local ok, response = self:get("changed-since/" .. timestamp)

	if not ok then
		return ok, response
	end

	return true, HttpService:JSONDecode(response)
end

return Net