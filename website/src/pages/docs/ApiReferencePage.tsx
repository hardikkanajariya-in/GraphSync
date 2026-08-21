import { DocArticle } from "../../components/DocArticle";

export function ApiReferencePage() {
  return (
    <DocArticle
      title="API reference"
      lede="Exact shapes for implementers. If you are building a compatible server, this is the checklist I use myself."
    >
      <p>
        Base URL is whatever you deploy — clients accept the URL configured in the desktop app.
        Authenticated routes expect <code>Authorization: Bearer &lt;device_token&gt;</code>.
      </p>

      <h2>Authentication</h2>
      <h3>POST /auth/device</h3>
      <pre>{`Request
{ "device_id": "uuid", "protocol_version": 1 }

Response
{ "token": "…", "device_id": "uuid", "protocol_version": 1 }`}</pre>

      <h2>Devices</h2>
      <h3>POST /devices/register</h3>
      <pre>{`Request
{
  "device_id": "uuid",
  "device_name": "MacBook",
  "platform": "macos",
  "protocol_version": 1
}`}</pre>
      <h3>GET /devices</h3>
      <pre>{`Response
{ "devices": [ { "device_id": "…", "device_name": "…", "platform": "…" } ] }`}</pre>

      <h2>Graphs</h2>
      <h3>POST /graphs/register</h3>
      <pre>{`Response
{ "graph_id": "uuid", "name": "MyGraph" }`}</pre>
      <h3>GET /graphs/:id</h3>
      <p>Returns graph metadata for the given ID.</p>

      <h2>Sync</h2>
      <h3>POST /sync/events</h3>
      <p>Accepts a sync event object. See the sync protocol page for field definitions.</p>
      <h3>GET /sync/events?graph_id=…&cursor=…&protocol_version=1</h3>
      <pre>{`Response
{ "events": [ … ], "cursor": "optional" }`}</pre>
      <h3>POST /sync/ack</h3>
      <pre>{`Request
{ "event_id": "uuid" }`}</pre>
      <h3>GET /sync/manifest/:graph_id</h3>
      <pre>{`Response
{
  "graph_id": "uuid",
  "files": [
    { "path": "pages/a.md", "sha256": "…", "size": 1, "mtime": 0, "revision": 1 }
  ]
}`}</pre>

      <h2>WebSocket</h2>
      <p>
        Connect to <code>/ws?device_id=…&protocol_version=1</code>. The server pushes sync events
        and signaling messages as JSON text frames.
      </p>

      <h2>Signaling</h2>
      <h3>POST /signaling/offer</h3>
      <p>Returns <code>{`{ "session_id": "uuid" }`}</code>. Target device receives the offer over WebSocket.</p>
      <h3>POST /signaling/answer</h3>
      <h3>POST /signaling/ice</h3>
      <p>Exchange SDP and ICE candidates. The API relays them; it does not terminate WebRTC media.</p>

      <h2>Errors</h2>
      <pre>{`{ "error": "invalid_request", "message": "Human-readable detail" }`}</pre>
      <p>
        Typical codes: <code>400</code> bad payload, <code>401</code> auth failure,{" "}
        <code>404</code> missing resource, <code>409</code> revision conflict,{" "}
        <code>500</code> server error.
      </p>
    </DocArticle>
  );
}
