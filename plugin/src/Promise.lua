--[[
	An implementation of Promises similar to Promise/A+.
]]

local PROMISE_DEBUG = true

local PromiseState = {
	Started = "Started",
	Resolved = "Resolved",
	Rejected = "Rejected",
}

-- If promise debugging is on, use a version of pcall that warns on failure.
-- This is useful for finding errors that happen within Promise itself.
local wpcall
if PROMISE_DEBUG then
	wpcall = function(f, ...)
		local result = { pcall(f, ...) }

		if not result[1] then
			warn(result[2])
		end

		return unpack(result)
	end
else
	wpcall = pcall
end

--[[
	Creates a function that invokes a callback with correct error handling and
	resolution mechanisms.
]]
local function createAdvancer(callback, resolve, reject)
	return function(...)
		local result = { wpcall(callback, ...) }
		local ok = table.remove(result, 1)

		if ok then
			resolve(unpack(result))
		else
			reject(unpack(result))
		end
	end
end

local Promise = {}
Promise.__index = Promise

--[[
	Constructs a new Promise with the given initializing callback.

	This is generally only called when directly wrapping a non-promise API into
	a promise-based version.

	The callback will receive 'resolve' and 'reject' methods, used to start
	invoking the promise chain.

	For example:

		local function get(url)
			return Promise.new(function(resolve, reject)
				spawn(function()
					resolve(HttpService:GetAsync(url))
				end)
			end)
		end

		get("https://google.com")
			:andThen(function(stuff)
				print("Got some stuff!", stuff)
			end)
]]
function Promise.new(callback)
	local promise = {
		_source = debug.traceback(),
		_type = "Promise",
		_status = PromiseState.Started,
		_value = nil,
		_queuedResolve = {},
		_queuedReject = {},
		_queuedPromises = {},
	}

	setmetatable(promise, Promise)

	local function resolve(...)
		promise:_resolve(...)
	end

	local function reject(...)
		promise:_reject(...)
	end

	local ok, err = wpcall(callback, resolve, reject)

	if not ok and promise._status == PromiseState.Started then
		reject(err)
	end

	return promise
end

--[[
	Is the given object a Promise instance?
]]
function Promise.is(object)
	if type(object) ~= "table" then
		return false
	end

	return object._type == "Promise"
end

function Promise:_resolve(...)
	if self._status ~= PromiseState.Started then
		return
	end

	self._status = PromiseState.Resolved
	self._value = {...}

	-- We assume that these callbacks will not throw errors.
	for _, callback in ipairs(self._queuedResolve) do
		callback(...)
	end
end

function Promise:_reject(...)
	if self._status ~= PromiseState.Started then
		return
	end

	self._status = PromiseState.Rejected
	self._value = {...}

	-- If no one attaches a handler to this promise before the next frame, this
	-- promise becomes an unhandled rejection.
	if #self._queuedReject == 0 and #self._queuedPromises == 0 then
		self._isUnhandled = true

		spawn(function()
			if self._isUnhandled then
				local message = ("Unhandled promise rejection:\n%s\n\n%s"):format(
					tostring(self._value[1]),
					self._source
				)
				error(message)
			end
		end)
	end

	for _, promise in ipairs(self._queuedPromises) do
		promise:_reject(...)
	end

	-- We assume that these callbacks will not throw errors.
	for _, callback in ipairs(self._queuedReject) do
		callback(...)
	end
end

function Promise:andThen(successCallback, failureCallback)
	self._isUnhandled = false

	local promise = Promise.new(function(resolve, reject)
		if self._status == PromiseState.Started then
			if successCallback then
				table.insert(self._queuedResolve, createAdvancer(successCallback, resolve, reject))
			end

			if failureCallback then
				table.insert(self._queuedReject, createAdvancer(failureCallback, resolve, reject))
			end
		elseif self._status == PromiseState.Resolved then
			if successCallback then
				createAdvancer(successCallback, resolve, reject)(unpack(self._value))
			end
		elseif self._status == PromiseState.Rejected then
			if failureCallback then
				createAdvancer(failureCallback, resolve, reject)(unpack(self._value))
			end
		end
	end)

	table.insert(self._queuedPromises, promise)

	return promise
end

function Promise:catch(failureCallback)
	return self:andThen(nil, failureCallback)
end

function Promise.resolve(value)
	return Promise.new(function(resolve)
		resolve(value)
	end)
end

function Promise.reject(value)
	return Promise.new(function(_, reject)
		reject(value)
	end)
end

function Promise.all(...)
	error("unimplemented", 2)
end

function Promise:await()
	local bindable = Instance.new("BindableEvent")
	local result

	self:andThen(function(...)
		result = {...}
		bindable:Fire(true)
	end, function(...)
		result = {...}
		bindable:Fire(false)
	end)

	local ok = bindable.Event:Wait()
	bindable:Destroy()

	if not ok then
		error(tostring(result[1]), 2)
	end

	return unpack(result)
end

return Promise
