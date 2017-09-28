# rbxfs 2.0 spec

## Server
*The server is limited to GET and POST requests due to limitations in the Roblox HTTP client.*

### type `DomNode`
```
{
	"name": "<name>",
	"instance": RobloxInstance,
	"children": {
		"<name>": DomNode
	}
}
```

### type `RobloxInstance`
```
{
	"type": "<instance type>",
	<other properties>
}
```

### `GET /fs/info`
Response:
```
{
	"server_version": "<version string>",
	"protocol_version": "1.0.0"
}
```

### `GET /fs/now`
Response:
```
{
	"now": 0.0
}
```

### `GET /fs/changed-since/<time>`
Response:
```
{
	"now": 0.0,
	"changes": [
		DomChange
	]
}
```

### `GET /fs/read-all`
Response:
```
{
	"now": 0.0,
	"root": DomNode
}
```

### `GET /fs/read/<path..>`
Response:
```
{
	"now": 0.0,
	"node": DomNode?
}
```

## Client