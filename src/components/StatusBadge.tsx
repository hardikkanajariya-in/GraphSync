interface StatusBadgeProps {
  status: string;
  paused?: boolean;
}

const labels: Record<string, string> = {
  synced: "Synced",
  paused: "Paused",
  starting: "Starting",
  degraded: "Degraded",
  error: "Error",
  idle: "Idle",
  setup: "Setup required",
};

export function StatusBadge({ status, paused }: StatusBadgeProps) {
  const label = paused ? "Paused" : labels[status] ?? status;
  const tone =
    status === "synced" && !paused
      ? "text-success"
      : status === "error" || status === "degraded"
        ? "text-danger"
        : status === "paused" || paused
          ? "text-warning"
          : "text-muted";

  return (
    <div className={`flex items-center gap-2 text-sm ${tone}`}>
      <span aria-hidden="true">●</span>
      <span>{label}</span>
    </div>
  );
}
