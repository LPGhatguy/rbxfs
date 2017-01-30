local Net = {
	baseUrl = "http://localhost:8000/"
}

local http = game:GetService("HttpService")

local function getNetName(scriptObject)
	local fullName = scriptObject:GetFullName()

	return (fullName:gsub("^game%.", ""))
end

function Net:setBaseUrl(baseUrl)
	self.baseUrl = baseUrl
end

function Net:get(endpoint)
	local ok, result = pcall(function()
		return http:GetAsync(self.baseUrl .. endpoint, true)
	end)

	if (not ok) then
		error(result)
	end

	return result
end

function Net:post(endpoint, body)
	local ok, result = pcall(function()
		return http:PostAsync(self.baseUrl .. endpoint, body)
	end)

	if (not ok) then
		error(result)
	end

	return result
end

function Net:version()
	return http:JSONDecode(self:get("version"))
end

function Net:list()
	return http:JSONDecode(self:get("files"))
end

function Net:read(scriptObject)
	return self:get("files/" .. getNetName(scriptObject) .. "/" .. scriptObject.ClassName)
end

function Net:write(scriptObject)
	return self:post("files/" .. getNetName(scriptObject) .. "/" .. scriptObject.ClassName, scriptObject.Source)
end

function Net:getChangedSince(timestamp)
	return http:JSONDecode(self:get("changed-since/" .. timestamp))
end

return Net