const { join, relative } = require("path");
const { watch } = require("chokidar");

const globPromise = require("./glob-promise");
const escapeForRegex = require("./escape-for-regex");

class VFS {
	constructor(config) {
		this.rootDirectory = config.rootDirectory;
		this.rootObject = config.rootObject;

		this._watcher = null;
		this._latestChanges = new Map();
	}

	_addChange(type, object) {
		if (object == null) {
			return;
		}

		const timestamp = this.now();

		this._latestChanges.set(object.name, {
			type,
			timestamp,
			object
		});

		console.log("Registered change for", object.name);
	}

	now() {
		const time = process.hrtime();

		return time[0] + time[1] / 1e9;
	}

	startWatching() {
		if (this._watcher != null) {
			return;
		}

		const watcher = this._watcher = watch(join(this.rootDirectory, "**/*.lua"), {
			ignoreInitial: true
		});

		const handleChange = filename => this._addChange("change", this.fileToRBX(filename));
		const handleDelete = filename => this._addChange("delete", this.fileToRBX(filename));

		watcher.on("add", handleChange);
		watcher.on("change", handleChange);
		watcher.on("unlink", handleDelete);
	}

	stopWatching() {
		if (this._watcher == null) {
			return;
		}

		this._watcher.close();
		this._watcher = null;
	}

	getChangedSince(timestamp) {
		return Array.from(this._latestChanges.values())
			.filter(change => change.timestamp >= timestamp);
	}

	list() {
		return globPromise(join(this.rootDirectory, "**/*.lua"))
			.then(filenames => filenames
				.map(name => this.fileToRBX(name))
				.filter(obj => obj != null)
			);
	}

	fileToRBX(filename) {
		let name = filename;
		let type = "ModuleScript";

		name = relative(this.rootDirectory, filename);

		const serverMatch = /^([^\.]+)\.server\.lua$/.exec(name);
		const clientMatch = /^([^\.]+)\.client\.lua$/.exec(name);
		const moduleMatch = /^([^\.]+)\.lua$/.exec(name);

		if (serverMatch) {
			type = "Script";
			name = serverMatch[1];
		} else if (clientMatch) {
			type = "LocalScript";
			name = clientMatch[1];
		} else if (moduleMatch) {
			type = "ModuleScript";
			name = moduleMatch[1];
		} else {
			return null;
		}

		name = name.replace(/[\/\\]/g, ".");

		name = this.rootObject + "." + name;

		return {
			type,
			name
		};
	}

	rbxToFile(rbx) {
		let suffix = ".lua";

		if (rbx.type === "Script") {
			suffix = ".server.lua";
		} else if (rbx.type === "LocalScript") {
			suffix = ".client.lua";
		}

		const replaceRoot = `^${ escapeForRegex(this.rootObject) }\\.?`;

		const name = rbx.name
			.replace(new RegExp(replaceRoot), "")
			.replace(/\./g, "/");

		const filename = join(this.rootDirectory, name) + suffix;

		return filename;
	}
}

module.exports = VFS;