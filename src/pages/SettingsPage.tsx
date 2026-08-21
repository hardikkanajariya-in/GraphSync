import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import type { PublicConfig } from "../types";
import {
  getConfig,
  getLogPath,
  resetDevice,
  updateSettings,
} from "../lib/tauri";

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const [config, setConfig] = useState<PublicConfig | null>(null);
  const [logPath, setLogPath] = useState("");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    void (async () => {
      setConfig(await getConfig());
      setLogPath(await getLogPath());
    })();
  }, []);

  async function pickFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string" && config) {
      setConfig({ ...config, sync_folder: selected });
    }
  }

  async function save() {
    if (!config) return;
    setSaving(true);
    setError(null);
    setMessage(null);
    try {
      const updated = await updateSettings({
        apiUrl: config.api_url,
        syncFolder: config.sync_folder ?? undefined,
        deviceName: config.device_name,
      });
      setConfig(updated);
      setMessage("Settings saved.");
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSaving(false);
    }
  }

  async function handleReset() {
    setSaving(true);
    setError(null);
    setMessage(null);
    try {
      const updated = await resetDevice();
      setConfig(updated);
      setMessage("Device identity reset.");
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSaving(false);
    }
  }

  if (!config) {
    return <div className="px-6 py-10 text-sm text-muted">Loading settings...</div>;
  }

  return (
    <div className="mx-auto flex min-h-full w-full max-w-md flex-col px-6 py-10">
      <button className="mb-6 text-left text-sm text-muted hover:text-ink" onClick={onBack}>
        ← Back
      </button>
      <h1 className="mb-8 text-2xl font-semibold tracking-tight">Settings</h1>

      <div className="space-y-5 text-sm">
        <div>
          <label className="mb-2 block text-muted">Sync folder</label>
          <div className="flex gap-2">
            <input
              className="min-w-0 flex-1 rounded-md border border-border bg-white px-3 py-2"
              value={config.sync_folder ?? ""}
              onChange={(e) => setConfig({ ...config, sync_folder: e.target.value })}
            />
            <button
              type="button"
              className="rounded-md border border-border bg-white px-3 py-2"
              onClick={pickFolder}
            >
              Browse
            </button>
          </div>
        </div>

        <div>
          <label className="mb-2 block text-muted">API URL</label>
          <input
            className="w-full rounded-md border border-border bg-white px-3 py-2"
            value={config.api_url}
            onChange={(e) => setConfig({ ...config, api_url: e.target.value })}
          />
        </div>

        <div>
          <label className="mb-2 block text-muted">Device name</label>
          <input
            className="w-full rounded-md border border-border bg-white px-3 py-2"
            value={config.device_name}
            onChange={(e) => setConfig({ ...config, device_name: e.target.value })}
          />
        </div>

        <div>
          <label className="mb-2 block text-muted">Logs</label>
          <p className="break-all text-muted">{logPath}</p>
        </div>
      </div>

      {message ? <p className="mt-6 text-sm text-success">{message}</p> : null}
      {error ? <p className="mt-6 text-sm text-danger">{error}</p> : null}

      <div className="mt-8 flex flex-wrap gap-3">
        <button
          className="rounded-md bg-accent px-4 py-2 text-sm font-medium text-white"
          disabled={saving}
          onClick={save}
        >
          Save
        </button>
        <button
          className="rounded-md border border-border bg-white px-4 py-2 text-sm"
          disabled={saving}
          onClick={handleReset}
        >
          Reset device
        </button>
      </div>
    </div>
  );
}
