import { useEffect, useState } from "react";
import { SetupPage } from "./pages/SetupPage";
import { StatusPage } from "./pages/StatusPage";
import { SettingsPage } from "./pages/SettingsPage";
import {
  getStatus,
  isSetupComplete,
  onSyncStatus,
  pauseSync,
  resumeSync,
} from "./lib/tauri";
import type { AppView, SyncStatus } from "./types";

const defaultStatus: SyncStatus = {
  status: "setup",
  connected: false,
  paused: false,
  last_sync: null,
  folder: null,
  device_name: "",
  pending_events: 0,
  failed_events: 0,
};

function App() {
  const [view, setView] = useState<AppView>("setup");
  const [status, setStatus] = useState<SyncStatus>(defaultStatus);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    void (async () => {
      const complete = await isSetupComplete();
      setView(complete ? "status" : "setup");
      if (complete) {
        setStatus(await getStatus());
      }
      setReady(true);
    })();

    const unsubs: Array<() => void> = [];
    void onSyncStatus((next) => {
      setStatus(next);
    }).then((unlisten) => unsubs.push(unlisten));

    return () => {
      unsubs.forEach((fn) => fn());
    };
  }, []);

  if (!ready) {
    return <div className="px-6 py-10 text-sm text-muted">Loading...</div>;
  }

  if (view === "setup") {
    return <SetupPage onComplete={() => setView("status")} />;
  }

  if (view === "settings") {
    return <SettingsPage onBack={() => setView("status")} />;
  }

  return (
    <StatusPage
      status={status}
      onOpenSettings={() => setView("settings")}
      onPause={() => void pauseSync().then(() => getStatus().then(setStatus))}
      onResume={() => void resumeSync().then(() => getStatus().then(setStatus))}
    />
  );
}

export default App;
