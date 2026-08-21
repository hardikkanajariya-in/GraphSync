import { DocArticle } from "../../components/DocArticle";

export function SyncProtocolPage() {
  return (
    <DocArticle
      title="Sync protocol"
      lede="This is the conversation devices have — first with the API about metadata, then with each other about files."
    >
      <h2>File identity</h2>
      <p>Every synchronized file is described by metadata, not by absolute paths on disk:</p>
      <ul>
        <li><code>path</code> — normalized relative path, always forward slashes</li>
        <li><code>sha256</code> — content hash</li>
        <li><code>size</code> and <code>mtime</code></li>
        <li><code>revision</code> — monotonic per path on the originating device</li>
        <li><code>operation</code> — CREATE, UPDATE, DELETE, or RENAME</li>
      </ul>
      <p>
        Absolute paths never leave the machine they belong to. That rule keeps the protocol
        portable and avoids leaking usernames or drive letters.
      </p>

      <h2>Publishing a local change</h2>
      <pre>{`POST /sync/events

{
  "event_id": "uuid",
  "graph_id": "uuid",
  "device_id": "uuid",
  "path": "pages/ideas.md",
  "operation": "UPDATE",
  "sha256": "…",
  "size": 2048,
  "revision": 17,
  "timestamp": "2026-08-21T09:00:00Z"
}`}</pre>
      <p>
        Events are idempotent. If the same event arrives twice because of a retry, the receiver
        should recognize that and avoid corrupting local state.
      </p>

      <h2>Catching up</h2>
      <p>
        Offline devices call <code>GET /sync/events</code> with their graph ID and optional cursor.
        They also fetch <code>GET /sync/manifest/:graph_id</code> during initial sync or recovery
        to see the latest known file set without walking the API event log by hand.
      </p>

      <h2>Peer file transfer</h2>
      <p>After signaling completes and a WebRTC data channel opens:</p>
      <ol>
        <li>The requester sends a JSON message with path and expected hash</li>
        <li>The provider streams binary chunks</li>
        <li>The provider sends <code>EOF</code></li>
        <li>The requester verifies SHA-256 before committing the file atomically</li>
      </ol>
      <p>
        Large files stay streamed end to end. I cared about this because note graphs can contain
        thousands of small files and the occasional large asset.
      </p>

      <h2>Deletions and renames</h2>
      <p>
        Deletes propagate as first-class events. If a remote delete would remove a file you edited
        offline, GraphSync treats that as a conflict and preserves your version under a conflict
        name instead of silently wiping it.
      </p>
      <p>
        Renames use a RENAME operation with <code>old_path</code> when the OS gives us enough
        information. When it does not, the client falls back to DELETE + CREATE without losing
        data.
      </p>
    </DocArticle>
  );
}
