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
		return success(self)
	else
		if failure then
			return failure(failure)
		else
			error(tostring(self.error))
		end
	end
end

function HttpResponse:isOk()
	return self._ok
end

function HttpResponse:json()
	return HttpService:JSONDecode(self.body)
end

return HttpResponse
