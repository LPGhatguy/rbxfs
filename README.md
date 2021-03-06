# RBXFS

## RBXFS has been replaced by [Rojo](https://github.com/LPGhatguy/Rojo) which is faster, more stable, and doesn't require installing Node.js!

A system to replicate scripts from the filesystem into Roblox Studio.

## Installation
- Make sure you have Node.js 8.0+ installed
- Install [this ROBLOX plugin](https://www.roblox.com/library/394835268/RBXFS)
- Install the server: `npm install -g rbxfs`

## Usage
- Navigate to the folder to sync and run `rbxfs`
- Use buttons in plugin to move files between server and Roblox

File names map to different script types:
- `*.server.lua` - `Script`
- `*.client.lua` - `LocalScript`
- `*.lua` - `ModuleScript`

## Configuration
RBXFS supports configuration via a file called `rbxfs.json` in the root of your project.

The default configuration values are:

```json
{
	"rootDirectory": "",
	"rootObject": ""
}
```

It assumes that your code begins in the current directory and matches the Roblox hierarchy starting from `game`. You might have folders named `ReplicatedStorage`, `ServerScriptStorage`, and these would map to the top-level services.

To synchronize files in the `src` directory to `ReplicatedStorage.MyGame`, use this configuration:

```json
{
	"rootDirectory": "src",
	"rootObject": "ReplicatedStorage.MyGame"
}
```

## Developing
Developing requires Node.js 8.x.

See `plugin` for the ROBLOX plugin source. This can be synced to ROBLOX using the last released version of the plugin.

See `lib` for the Node.js server source. Use `npm link` to use the Git version in your projects.

## Why not [RbxRefresh](https://github.com/osyrisrblx/RbxRefresh)?
I hadn't heard of RbxRefresh when I built this!

Other than that:
* RbxRefresh's naming conventions make `ModuleScript` files unnecessarily verbose
* I wanted a testbed for experimenting with further syncing than just Lua scripts

## License
RBXFS is available under the MIT license. See [LICENSE.md](LICENSE.md) for details.