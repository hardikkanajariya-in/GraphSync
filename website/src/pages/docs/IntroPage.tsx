import { Link } from "react-router-dom";
import { DocArticle } from "../../components/DocArticle";

export function DocsIntroPage() {
  return (
    <DocArticle
      title="Welcome to GraphSync"
      lede="I wrote these docs the way I explain the project to someone sitting next to me — clear, honest, and without pretending the hard parts are trivial."
    >
      <p>
        GraphSync helps you keep a folder in sync across your own devices. Your Windows machine,
        your MacBook, maybe a small home server — they all watch the same logical graph or
        project directory, and they stay aligned even when they are not online at the same time.
      </p>
      <p>
        The important distinction is this: GraphSync is not a cloud drive. The coordination API
        remembers <em>what changed</em>, not the contents of your files. When a device needs an
        update, it asks another device directly.
      </p>

      <h2>Who this is for</h2>
      <p>
        I originally shaped GraphSync around Logseq graph folders, but the client is generic. If
        you can point it at a directory, it can sync it — notes, assets, config, plain Markdown,
        anything that lives as normal files on disk.
      </p>
      <p>You might be:</p>
      <ul>
        <li>Running the desktop client and connecting it to a GraphSync API</li>
        <li>Building or hosting that API for your team</li>
        <li>Reading the protocol because you want peer-to-peer sync without central file storage</li>
      </ul>

      <h2>How to read these docs</h2>
      <p>
        Start with <Link to="/docs/how-it-works">How it works</Link> if you want the mental model.
        Jump to <Link to="/docs/getting-started">Getting started</Link> if you already understand
        the shape and want concrete steps. The{" "}
        <Link to="/docs/api">API reference</Link> is here when you need exact request and response
        shapes.
      </p>
      <p>
        Protocol version <code>1</code> is current. When I introduce breaking changes, I will bump
        that number and document the migration path here first.
      </p>
    </DocArticle>
  );
}
