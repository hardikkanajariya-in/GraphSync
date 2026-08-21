import { DocArticle } from "../../components/DocArticle";

export function GettingStartedPage() {
  return (
    <DocArticle
      title="Getting started"
      lede="You need two things: the desktop client on your machines, and a GraphSync API endpoint they can talk to."
    >
      <h2>Step 1 — Run or deploy the API</h2>
      <p>
        Point your client at the base URL of your coordination server, for example{" "}
        <code>https://sync.example.com</code>. The API must speak protocol version{" "}
        <code>1</code> and expose the routes documented in the API reference.
      </p>
      <p>
        Use HTTPS in production. WebSockets should use <code>wss://</code>. I am strict about
        this because the API carries device tokens even though it does not carry file contents.
      </p>

      <h2>Step 2 — Install the desktop client</h2>
      <p>
        Grab the latest release for your platform from GitHub — Windows installer or macOS disk
        image. Open it, choose the folder you want to sync, paste your API URL, and start sync.
      </p>
      <p>
        On first launch the client generates a permanent device ID, registers with the API, and
        then moves to the system tray so it can work quietly in the background.
      </p>

      <h2>Step 3 — Add a second device</h2>
      <p>
        Install the client on another machine, point it at the same API, and select the folder
        you want kept in sync. When a new device joins an existing graph, it compares manifests
        and pulls missing files peer-to-peer. Nothing uploads the whole tree through the API.
      </p>

      <h2>What you configure once</h2>
      <pre>{`{
  "api_url": "https://sync.example.com",
  "sync_folder": "/Users/you/Documents/MyGraph",
  "device_name": "MacBook",
  "stun_servers": ["stun:stun.l.google.com:19302"],
  "turn_servers": []
}`}</pre>
      <p>
        STUN is often enough for home and office networks. If you deploy GraphSync for a team
        behind strict corporate firewalls, plan for TURN early rather than debugging mysterious
        peer failures later.
      </p>

      <h2>A quick sanity check</h2>
      <p>After setup, you should see:</p>
      <ol>
        <li>The client reporting a connected status</li>
        <li>Your device listed on the API</li>
        <li>A test file change appearing as an event on the other machine within a few seconds</li>
      </ol>
      <p>
        If metadata syncs but files do not move, look at signaling and STUN/TURN before you
        assume the API is broken.
      </p>
    </DocArticle>
  );
}
