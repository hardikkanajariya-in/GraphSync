import { DocArticle } from "../../components/DocArticle";

export function SecurityPage() {
  return (
    <DocArticle
      title="Security"
      lede="GraphSync reduces risk by keeping file bytes off the server. What remains still deserves careful treatment."
    >
      <h2>What the API is allowed to know</h2>
      <p>
        Device IDs, graph membership, relative paths, content hashes, sizes, revisions, and
        signaling payloads. That is enough to coordinate sync. It is also sensitive in aggregate —
        paths can reveal project structure — so treat API access like infrastructure, not a public
        toy endpoint.
      </p>

      <h2>Transport</h2>
      <ul>
        <li>HTTPS for REST</li>
        <li>WSS for WebSockets</li>
        <li>Encrypted WebRTC data channels for file bytes</li>
      </ul>
      <p>
        I do not log tokens or file contents in the desktop client. If you operate the API, adopt
        the same discipline on the server side.
      </p>

      <h2>Path handling</h2>
      <p>
        Clients reject paths containing <code>..</code> and never send absolute filesystem paths.
        Incoming peer metadata is validated before any write touches disk. Atomic replacement is
        used when applying downloaded files so partial writes do not masquerade as success.
      </p>

      <h2>Authentication model</h2>
      <p>
        Each installation receives a device token from <code>POST /auth/device</code>. Tokens should
        be scoped to that device. Rotate or invalidate them if a machine is lost.
      </p>

      <h2>Conflicts and deletes</h2>
      <p>
        The security story includes data safety, not just network safety. Remote deletes do not
        override unsynced local edits. Conflicts produce a second file instead of a destructive
        merge. For binary assets that matters as much as for Markdown notes.
      </p>

      <h2>Reporting issues</h2>
      <p>
        If you find a vulnerability in the client or the documented protocol, please report it
        privately through the project&apos;s security policy on GitHub rather than opening a public
        issue with exploit details.
      </p>
    </DocArticle>
  );
}
