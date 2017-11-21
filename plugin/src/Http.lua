local HttpService = game:GetService("HttpService")

local HttpError = require(script.Parent.HttpError)
local HttpResponse = require(script.Parent.HttpResponse)

local Http = {}
Http.__index = Http

function Http.new(baseUrl)
	assert(type(baseUrl) == "string", "Http.new needs a baseUrl!")

	local http = {
		baseUrl = baseUrl
	}

	setmetatable(http, Http)

	return http
end

function Http:get(endpoint)
	local err = nil
	local ok, result = pcall(function()
		return HttpService:GetAsync(self.baseUrl .. endpoint, true)
	end)

	if not ok then
		err = HttpError.fromErrorString(result)
		result = nil
	end

	return HttpResponse.new(ok, result, err)
end

function Http:post(endpoint, body)
	local err = nil
	local ok, result = pcall(function()
		return HttpService:PostAsync(self.baseUrl .. endpoint, body)
	end)

	if not ok then
		err = HttpError.fromErrorString(result)
		result = nil
	end

	return HttpResponse.new(ok, result, err)
end

return Http
