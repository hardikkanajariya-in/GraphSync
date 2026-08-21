import { DocArticle } from "../../components/DocArticle";
import { LINKS } from "../../lib/site";

export function DesktopClientPage() {
  return (
    <DocArticle
      title="The desktop client"
      lede="The app you install is deliberately small. It exists to watch your folder, speak to the API, and move bytes between peers."
    >
      <p>
        GraphSync is built with Tauri, React, and Rust. The UI handles setup and status. The sync
        engine — watching, hashing, networking, conflict handling — runs in Rust so it stays light
        while idle.
      </p>

      <h2>What you see in the app</h2>
      <p>
        First launch asks for a folder and an API URL. That is intentional. I did not want a long
        onboarding funnel for a utility that should feel like a tool, not a platform.
      </p>
      <p>After setup you get:</p>
      <ul>
        <li>A tray icon with pause, resume, open folder, and quit</li>
        <li>A compact status screen with sync state and last activity</li>
        <li>Settings for folder, API URL, device name, and device reset</li>
      </ul>

      <h2>What happens in the background</h2>
      <ul>
        <li>Filesystem events are debounced (~750ms) to handle editor temp files sensibly</li>
        <li>SHA-256 hashing streams large files instead of loading them entirely into RAM</li>
        <li>Local state persists across restarts in a lightweight JSON store</li>
        <li>Failed events go into a retry queue with backoff instead of killing the engine</li>
      </ul>

      <h2>Downloads</h2>
      <p>
        Prebuilt installers are published on{" "}
        <a href={LINKS.releases}>GitHub Releases</a>. Windows builds ship as{" "}
        <code>.msi</code> or <code>.exe</code>. macOS builds ship as <code>.dmg</code> for Apple
        Silicon and Intel separately.
      </p>
      <p>
        Community releases are unsigned by default. Your OS may ask for confirmation on first
        install. Signed builds can be added later for teams that need them.
      </p>

      <h2>Logseq and beyond</h2>
      <p>
        I use GraphSync with Logseq graph folders — <code>pages/</code>, <code>journals/</code>,{" "}
        <code>assets/</code>, and the rest — but nothing in the client is Logseq-specific. Point
        it at any directory tree and it behaves the same way.
      </p>
    </DocArticle>
  );
}
