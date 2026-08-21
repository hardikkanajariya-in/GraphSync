import { StatusBadge } from "../components/StatusBadge";
import type { SyncStatus } from "../types";

interface StatusPageProps {
  status: SyncStatus;
  onPause: () => void;
  onResume: () => void;
  onOpenSettings: () => void;
}

export function StatusPage({
  status,
  onPause,
  onResume,
  onOpenSettings,
}: StatusPageProps) {
  return (
    <div className="mx-auto flex min-h-full w-full max-w-md flex-col px-6 py-10">
      <div className="mb-8 flex items-start justify-between">
        <h1 className="text-2xl font-semibold tracking-tight">GraphSync</h1>
        <button
          className="text-sm text-muted hover:text-ink"
          onClick={onOpenSettings}
        >
          Settings
        </button>
      </div>

      <StatusBadge status={status.status} paused={status.paused} />

      <dl className="mt-8 space-y-4 text-sm">
        <div>
          <dt className="text-muted">Folder</dt>
          <dd className="mt-1 break-all">{status.folder ?? "—"}</dd>
        </div>
        <div>
          <dt className="text-muted">Device</dt>
          <dd className="mt-1">{status.device_name}</dd>
        </div>
        <div>
          <dt className="text-muted">Last sync</dt>
          <dd className="mt-1">{status.last_sync ?? "—"}</dd>
        </div>
      </dl>

      <div className="mt-10">
        {status.paused ? (
          <button
            className="rounded-md border border-border bg-white px-4 py-2 text-sm hover:bg-white"
            onClick={onResume}
          >
            Resume
          </button>
        ) : (
          <button
            className="rounded-md border border-border bg-white px-4 py-2 text-sm hover:bg-white"
            onClick={onPause}
          >
            Pause
          </button>
        )}
      </div>
    </div>
  );
}
