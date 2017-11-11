function escapeForRegex(value) {
	return value.replace(/[.\[\]\(\)]/g, "\\$&");
}

module.exports = escapeForRegex;