# GraphSync API Contract

Protocol version: **1**

GraphSync clients communicate with a central coordination API. The API stores **metadata only** — never file contents.

## Authentication

All authenticated endpoints require:

```http
Authorization: Bearer <device_token>
```

### `POST /auth/device`

Issue or refresh a device token.

Request:

```json
{
  "device_id": "uuid",
  "protocol_version": 1
}
```

Response:

```json
{
  "token": "jwt-or-opaque-token",
  "device_id": "uuid",
  "protocol_version": 1
}
```

## Devices

### `POST /devices/register`

Register the current installation.

Request:

```json
{
  "device_id": "uuid",
  "device_name": "MacBook",
  "platform": "macos",
  "protocol_version": 1
}
```

Response:

```json
{
  "device_id": "uuid",
  "device_name": "MacBook",
  "platform": "macos",
  "registered_at": "2026-08-21T00:00:00Z"
}
```

### `GET /devices`

Response:

```json
{
  "devices": [
    {
      "device_id": "uuid",
      "device_name": "Windows PC",
      "platform": "windows",
      "registered_at": "2026-08-21T00:00:00Z"
    }
  ]
}
```

## Graphs

### `POST /graphs/register`

Associate a device with a sync graph.

Request:

```json
{
  "device_id": "uuid",
  "name": "MyGraph",
  "protocol_version": 1
}
```

Response:

```json
{
  "graph_id": "uuid",
  "name": "MyGraph"
}
```

### `GET /graphs/:id`

Response:

```json
{
  "graph_id": "uuid",
  "name": "MyGraph",
  "protocol_version": 1
}
```

## Sync metadata

### `POST /sync/events`

Publish a filesystem event.

Request:

```json
{
  "event_id": "uuid",
  "graph_id": "uuid",
  "device_id": "uuid",
  "path": "pages/foo.md",
  "old_path": null,
  "operation": "UPDATE",
  "sha256": "hex",
  "size": 1234,
  "revision": 42,
  "timestamp": "2026-08-21T00:00:00Z"
}
```

Operations: `CREATE`, `UPDATE`, `DELETE`, `RENAME`

For `RENAME`, include `old_path`.

Response: `204 No Content` or `{ "accepted": true }`

### `GET /sync/events?graph_id=<id>&cursor=<optional>&protocol_version=1`

Response:

```json
{
  "events": [ /* SyncEvent[] */ ],
  "cursor": "optional-cursor"
}
```

### `POST /sync/ack`

Acknowledge event processing.

Request:

```json
{
  "event_id": "uuid"
}
```

### `GET /sync/manifest/:graph_id`

Response:

```json
{
  "graph_id": "uuid",
  "files": [
    {
      "path": "pages/foo.md",
      "sha256": "hex",
      "size": 1234,
      "mtime": 1755744000,
      "revision": 42
    }
  ]
}
```

## WebSocket

`GET /ws?device_id=<uuid>&protocol_version=1`

The server pushes:

- serialized `SyncEvent` objects
- signaling messages (`offer`, `answer`, `ice`)

Clients respond to WebSocket ping frames.

## WebRTC signaling

### `POST /signaling/offer`

Request:

```json
{
  "graph_id": "uuid",
  "source_device_id": "uuid",
  "target_device_id": "uuid",
  "sdp": "webrtc-offer-sdp",
  "protocol_version": 1
}
```

Response:

```json
{
  "session_id": "uuid"
}
```

The target device receives a websocket message:

```json
{
  "type": "offer",
  "session_id": "uuid",
  "graph_id": "uuid",
  "source_device_id": "uuid",
  "target_device_id": "uuid",
  "sdp": "..."
}
```

### `POST /signaling/answer`

Request:

```json
{
  "session_id": "uuid",
  "sdp": "webrtc-answer-sdp",
  "protocol_version": 1
}
```

### `POST /signaling/ice`

Request:

```json
{
  "session_id": "uuid",
  "candidate": "candidate:...",
  "sdp_mid": "0",
  "sdp_mline_index": 0,
  "protocol_version": 1
}
```

## P2P file transfer

After the WebRTC data channel opens:

1. Requester sends JSON `{ "path": "...", "sha256": "..." }`
2. Provider streams binary chunks
3. Provider sends text message `"EOF"`
4. Requester verifies SHA-256

File bytes never pass through the API.

## Error responses

```json
{
  "error": "invalid_request",
  "message": "Human-readable detail"
}
```

Common status codes:

- `400` invalid payload
- `401` missing/invalid token
- `404` unknown graph/event/device
- `409` revision conflict
- `500` server failure

## Security requirements

- HTTPS/WSS in production
- Reject relative paths containing `..`
- Never accept absolute local filesystem paths
- Tokens must not appear in logs
- Device tokens scoped to `device_id`
