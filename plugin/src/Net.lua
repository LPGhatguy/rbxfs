local Net = {
	baseUrl = "http://localhost:8000/"
}

local HttpService = game:GetService("HttpService")

local function getNetName(scriptObject)
	local fullName = scriptObject:GetFullName()

	return (fullName:gsub("^game%.", ""))
end

function Net:setBaseUrl(baseUrl)
	self.baseUrl = baseUrl
end

function Net:get(endpoint)
	local ok, result = pcall(function()
		return HttpService:GetAsync(self.baseUrl .. endpoint, true)
	end)

	return ok, result
end

function Net:post(endpoint, body)
	local ok, result = pcall(function()
		return HttpService:PostAsync(self.baseUrl .. endpoint, body)
	end)

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