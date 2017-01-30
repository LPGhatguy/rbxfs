// @flow

import { join, relative } from "path";
import { watch } from "chokidar";

import globPromise from "./glob-promise";

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
	root: string;

	_watcher: ?ChokidarWatcher;
	_latestChanges: Map<string, ChangeEvent> = new Map();

	constructor(root: string) {
		this.root = root;
	}

	_addChange(type: "change" | "delete", object: RBXObject): void {
		const timestamp = this.now();

		this._latestChanges.set(object.name, {
			type,
			timestamp,
			object
		});

		console.log("Added change for", object.name, timestamp);
	}

	now(): number {
		const time = process.hrtime();

		return time[0] + time[1] / 1e9;
	}

	startWatching(): void {
		if (this._watcher != null) {
			return;
		}

		const watcher = this._watcher = watch(join(this.root, "**/*.lua"), {
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
		return globPromise(join(this.root, "**/*.lua"))
			.then(filenames => filenames.map(name => this.fileToRBX(name)));
	}

	fileToRBX(filename: string): RBXObject {
		let name = filename;
		let type = "ModuleScript";

		name = relative(this.root, filename);

		if (filename.endsWith(".server.lua")) {
			type = "Script";
			name = name.replace(/\.server\.lua$/, "");
		} else if (filename.endsWith(".client.lua")) {
			type = "LocalScript";
			name = name.replace(/\.client\.lua$/, "");
		} else {
			name = name.replace(/\.lua$/, "");
		}

		name = name.replace(/\//g, ".");

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

		return rbx.name
			.replace(/\./g, "/") + suffix;
	}
}