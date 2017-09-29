# rbxfs 2.0 spec

## Server
*The server is limited to GET and POST requests due to limitations in the Roblox HTTP client.*

### type `Instance`
```
{
	"name": "<name>",
	"details": InstanceDetails,
	"children": {
		"<name>": DomNode
	}
}
```

### type `InstanceDetails`
```
{
	"type": "<instance type>",
	<other properties>
}
```

### type `DomChange`
```
{
	"route": ["path", "to", "instance"],
	"timestamp": 0.0
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

### `GET /fs/changes-since/<time>`
Response:
```
{
	"now": 0.0,
	"changes": [
		...DomChange
	]
}
```

### `GET /fs/read/<path..>?`
Response:
```
{
	"now": 0.0,
	"node": DomNode?
}
```

## Client