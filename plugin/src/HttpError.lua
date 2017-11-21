local HttpError = {}
HttpError.__index = HttpError

HttpError.Error = {
	HttpNotEnabled = {
		message = "RBXFS requires HTTP access, which is not enabled.\n" ..
			"Check your game settings, located in the 'Home' tab of Studio.",
	},
	ConnectFailed = {
		message = "RBXFS plugin couldn't connect to the RBXFS server.\n" ..
			"Make sure the server is running -- use 'rbxfs serve' to run it!",
	},
	Unknown = {
		message = "RBXFS encountered an unknown error: {{message}}",
	},
}

function HttpError.new(type, extraMessage)
	extraMessage = extraMessage or ""
	local message = type.message:gsub("{{message}}", extraMessage)

	local err = {
		type = type,
		message = message,
	}

	setmetatable(err, HttpError)

	return err
end

function HttpError:__tostring()
	return self.message
end

--[[
	This method shouldn't have to exist. Ugh.
]]
function HttpError.fromErrorString(err)
	err = err:lower()

	if err:find("^http requests are not enabled") then
		return HttpError.new(HttpError.Error.HttpNotEnabled)
	end

	if err:find("^curl error") then
		return HttpError.new(HttpError.Error.ConnectFailed)
	end

	return HttpError.new(HttpError.Error.Unknown, err)
end

function HttpError:report()
	warn(self.message)
end

return HttpError
