#!/usr/bin/env node

// @flow

import express from "express";
import { createReadStream, createWriteStream, readFileSync } from "fs";
import * as path from "path";

import PROJECT from "../package";
import { VFS } from "./vfs";

console.log("??????????");

const defaultConfig = {
	rootObject: "",
	rootDirectory: "."
};

let config: Object;

try {
	const configPath = path.join(process.cwd(), "rbxfs.json");
	const configContents = readFileSync(configPath, "utf8");

	try {
		const configData = JSON.parse(configContents);

		config = {
			...defaultConfig,
			...configData
		};

		console.log("Loaded configuration at", configPath);
	} catch (error) {
		console.error("Couldn't parse rbxfs.json:", error);
	}
} catch (error) {
	console.log("Using default configuration...");
	config = {
		...defaultConfig
	};
}

const vfs = new VFS(config);
vfs.startWatching();

const app = express();

app.get("/", (req: express$Request, res) => {
	res.send("Server is up and running!");
});

app.get("/version", (req: express$Request, res) => {
	res.send(JSON.stringify({
		version: PROJECT.version
	}));
});

app.get("/now", (req: express$Request, res) => {
	res.send(JSON.stringify({
		now: vfs.now()
	}));
});

app.get("/changed-since/:timestamp", (req: express$Request, res) => {
	const time = parseFloat(req.params.timestamp);

	res.send(JSON.stringify({
		changed: vfs.getChangedSince(time),
		now: vfs.now()
	}));
});

app.get("/files", (req: express$Request, res) => {
	vfs.list()
		.then(files => {
			res.send(JSON.stringify({ files }));
		});
});

app.get("/files/:name/:type", (req: express$Request, res) => {
	const filename = vfs.rbxToFile({
		name: req.params.name,
		type: req.params.type
	});

	createReadStream(filename).pipe(res);
});

app.post("/files/:name/:type", (req: express$Request, res) => {
	const filename = vfs.rbxToFile({
		name: req.params.name,
		type: req.params.type
	});

	req.pipe(createWriteStream(filename));

	res.send(JSON.stringify({
		success: true
	}));
});

app.listen(8000, () => {
	console.log("Server listening on port 8000");
});