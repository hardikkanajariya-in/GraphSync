import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

interface SetupPageProps {
  onComplete: () => void;
}

export function SetupPage({ onComplete }: SetupPageProps) {
  const [apiUrl, setApiUrl] = useState("https://your-api.example.com");
  const [syncFolder, setSyncFolder] = useState("");
  const [deviceName, setDeviceName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function pickFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") {
      setSyncFolder(selected);
    }
  }

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    setError(null);
    setLoading(true);
    try {
      const { saveSetup } = await import("../lib/tauri");
      await saveSetup(
        apiUrl,
        syncFolder,
        deviceName || "GraphSync Device",
      );
      onComplete();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="mx-auto flex min-h-full w-full max-w-md flex-col justify-center px-6 py-10">
      <h1 className="mb-8 text-2xl font-semibold tracking-tight">GraphSync</h1>

      <form className="space-y-6" onSubmit={handleSubmit}>
        <div>
          <label className="mb-2 block text-sm text-muted">Folder to sync</label>
          <div className="flex gap-2">
            <input
              className="min-w-0 flex-1 rounded-md border border-border bg-white px-3 py-2 text-sm"
              value={syncFolder}
              onChange={(e) => setSyncFolder(e.target.value)}
              placeholder="Select a folder"
              required
            />
            <button
              type="button"
              className="rounded-md border border-border bg-white px-3 py-2 text-sm hover:bg-white"
              onClick={pickFolder}
            >
              Select Folder
            </button>
          </div>
        </div>

        <div>
          <label className="mb-2 block text-sm text-muted">API Server</label>
          <input
            className="w-full rounded-md border border-border bg-white px-3 py-2 text-sm"
            value={apiUrl}
            onChange={(e) => setApiUrl(e.target.value)}
            placeholder="https://your-api.example.com"
            required
          />
        </div>

        <div>
          <label className="mb-2 block text-sm text-muted">Device name</label>
          <input
            className="w-full rounded-md border border-border bg-white px-3 py-2 text-sm"
            value={deviceName}
            onChange={(e) => setDeviceName(e.target.value)}
            placeholder="My Windows PC"
          />
        </div>

        {error ? <p className="text-sm text-danger">{error}</p> : null}

        <button
          type="submit"
          disabled={loading || !syncFolder || !apiUrl}
          className="w-full rounded-md bg-accent px-4 py-2.5 text-sm font-medium text-white hover:opacity-95"
        >
          {loading ? "Starting..." : "Start Sync"}
        </button>
      </form>
    </div>
  );
}
