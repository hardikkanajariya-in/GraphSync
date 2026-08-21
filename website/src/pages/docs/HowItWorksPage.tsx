import { DocArticle } from "../../components/DocArticle";

export function HowItWorksPage() {
  return (
    <DocArticle
      title="How GraphSync works"
      lede="Once you see the split between metadata and file bytes, the rest of the system clicks into place."
    >
      <p>
        Imagine two computers working on the same folder. When you save a file on one machine,
        GraphSync notices, computes a SHA-256 hash, and sends a small event to the API: path,
        operation, hash, size, revision. That event is the entire server-side story for that
        change.
      </p>
      <p>
        Your other machine receives the event — usually over a WebSocket, sometimes by catching
        up after being offline. It compares the event with its local state. If the file is
        missing or the hash differs, it opens a peer connection and downloads the bytes from a
        device that already has them.
      </p>

      <h2>The three layers</h2>
      <h3>1. Filesystem watching (on each device)</h3>
      <p>
        The desktop client uses OS-level file notifications. It does not poll your tree every
        second. Rapid saves from editors are debounced so you do not spam the network with ten
        events for one keystroke session.
      </p>

      <h3>2. Coordination (the API)</h3>
      <p>The API is the shared memory of the system. It tracks:</p>
      <ul>
        <li>Which devices belong to a graph</li>
        <li>Which files changed, were deleted, or were renamed</li>
        <li>Who still needs to acknowledge an event</li>
        <li>WebRTC offers, answers, and ICE candidates for signaling</li>
      </ul>
      <p>
        It never persists file payloads. If someone asks what you would need to rebuild GraphSync
        on a new machine, the honest answer is: the metadata plus a peer that still has the files.
      </p>

      <h3>3. Peer transfer (device to device)</h3>
      <p>
        WebRTC carries the actual sync traffic. STUN helps peers find each other across NAT. In
        tougher networks you can configure TURN as a relay fallback. The API stays out of that
        path once the session is established.
      </p>

      <h2>Offline work is normal</h2>
      <p>
        I designed GraphSync assuming real life: laptops sleep, flights have no Wi‑Fi, APIs
        restart. A device that comes back online fetches missed events, reconciles its local
        manifest, and pulls whatever it still needs from peers. You should not need every device
        awake at once for metadata to stay coherent.
      </p>

      <h2>When two edits collide</h2>
      <p>
        If two devices change the same file while offline, GraphSync does not pick a silent
        winner. It keeps both versions and names the conflict copy in a predictable way so you
        can merge manually. I would rather leave you with an extra file than destroy work.
      </p>
    </DocArticle>
  );
}
