# RBXFS
A system to replicate scripts from a ROBLOX place to the filesystem.

## Installation
- Install [this ROBLOX plugin](https://www.roblox.com/library/394835268/RBXFS)
- Run `npm install -g rbxfs`
- Navigate to the folder to sync and run `rbxfs`
- Use buttons in plugin to move files between server and ROBLOX

## Configuration
As of RBXFS version 0.2.0, the system supports configuration via a file called `rbxfs.json` in the root of your project.

To synchronize files in the `src` directory to `ReplicatedStorage.MyGame`, use this configuration:

```json
{
	"rootDirectory": "src",
	"rootObject": "ReplicatedStorage.MyGame"
}
```

## Developing
Building requires Node.js 6.x.

To build from this repository:

- Run `npm install -g gulp`
- Run `gulp`

See `plugin` for the ROBLOX plugin source. This can be synced to ROBLOX using the last released version of the plugin.

See `src` for the Node.js server source.