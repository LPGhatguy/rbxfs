local HttpService = game:GetService("HttpService")

local HttpResponse = {}
HttpResponse.__index = HttpResponse

function HttpResponse.new(ok, body, err)
	local response = {
		_ok = ok,
		body = body,
		error = err,
	}

	setmetatable(response, HttpResponse)

	return response
end

function HttpResponse:andThen(success, failure)
	if self:isOk() then
		success(self)
	else
		if failure then
			failure(failure)
		else
			self:report()
		end
	end
end

function HttpResponse:isOk()
	return self._ok
end

function HttpResponse:json()
	return HttpService:JSONDecode(self.body)
end

function HttpResponse:report()
	self.error:report()
end

return HttpResponse
