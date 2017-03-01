export default function escapeForRegex(value: string) {
	return value
		.replace(/[.\[\]\(\)]/g, "\\$&");
}