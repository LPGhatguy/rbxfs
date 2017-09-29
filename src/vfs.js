// @flow

import { join, relative } from "path";
import { watch } from "chokidar";

import globPromise from "./glob-promise";
import escapeForRegex from "./escape-for-regex";

type ChokidarWatcher = Object;

export type ChangeEvent = {
	type: "change" | "delete",
	timestamp: number,
	object: RBXObject
};

export type RBXObject = {
	type: string,
	name: string
};

export class VFS {
	rootObject: string;
	rootDirectory: string;

	_watcher: ?ChokidarWatcher;
	_latestChanges: Map<string, ChangeEvent> = new Map();

	constructor(config: Object) {
		this.rootDirectory = config.rootDirectory;
		this.rootObject = config.rootObject;
	}

	_addChange(type: "change" | "delete", object: RBXObject): void {
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

	now(): number {
		const time = process.hrtime();

		return time[0] + time[1] / 1e9;
	}

	startWatching(): void {
		if (this._watcher != null) {
			return;
		}

		const watcher = this._watcher = watch(join(this.rootDirectory, "**/*.lua"), {
			ignoreInitial: true
		});

		const handleChange = (filename: string) => this._addChange("change", this.fileToRBX(filename));
		const handleDelete = (filename: string) => this._addChange("delete", this.fileToRBX(filename));

		watcher.on("add", handleChange);
		watcher.on("change", handleChange);
		watcher.on("unlink", handleDelete);
	}

	stopWatching(): void {
		if (this._watcher == null) {
			return;
		}

		this._watcher.close();
		this._watcher = null;
	}

	getChangedSince(timestamp: number): ChangeEvent[] {
		return Array.from(this._latestChanges.values())
			.filter(change => change.timestamp >= timestamp);
	}

	list(): Promise<RBXObject[]> {
		return globPromise(join(this.rootDirectory, "**/*.lua"))
			.then(filenames => filenames
				.map(name => this.fileToRBX(name))
				.filter(obj => obj != null)
			);
	}

	fileToRBX(filename: string): RBXObject {
		let name = filename;
		let type = "ModuleScript";

		name = relative(this.rootDirectory, filename);

		let serverMatch = /^([^\.]+)\.server\.lua$/.exec(name);
		let clientMatch = /^([^\.]+)\.client\.lua$/.exec(name);
		let moduleMatch = /^([^\.]+)\.lua$/.exec(name);

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

	rbxToFile(rbx: RBXObject): string {
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