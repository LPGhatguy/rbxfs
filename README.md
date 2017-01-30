# RBXFS
A system to replicate scripts from a ROBLOX place to the filesystem.

## Installation
- Install [this ROBLOX plugin](https://www.roblox.com/library/394835268/RBXFS)
- Run `npm install -g rbxfs`
- Navigate to the folder to sync and run `rbxfs`
- Use buttons in plugin to move files between server and ROBLOX

## Developing
Building requires Node.js 6.x.

To build from this repository:

- Run `npm install -g gulp`
- Run `gulp`

See `plugin` for the ROBLOX plugin source. This can be synced to ROBLOX using the last released version of the plugin.

See `src` for the Node.js server source.