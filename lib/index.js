#!/usr/bin/env node

const express = require("express");
const { createReadStream, createWriteStream, readFileSync } = require("fs");
const path = require("path");

const PACKAGE = require("../package");
const VFS = require("./vfs");

const defaultConfig = {
	rootObject: "",
	rootDirectory: "."
};

let config;

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

app.get("/", (req, res) => {
	res.send("Server is up and running!");
});

app.get("/version", (req, res) => {
	res.send(JSON.stringify({
		version: PACKAGE.version
	}));
});

app.get("/now", (req, res) => {
	res.send(JSON.stringify({
		now: vfs.now()
	}));
});

app.get("/changed-since/:timestamp", (req, res) => {
	const time = parseFloat(req.params.timestamp);

	res.send(JSON.stringify({
		changed: vfs.getChangedSince(time),
		now: vfs.now()
	}));
});

app.get("/files", (req, res) => {
	vfs.list()
		.then(files => {
			res.send(JSON.stringify({ files }));
		});
});

app.get("/files/:name/:type", (req, res) => {
	const filename = vfs.rbxToFile({
		name: req.params.name,
		type: req.params.type
	});

	createReadStream(filename).pipe(res);
});

app.post("/files/:name/:type", (req, res) => {
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