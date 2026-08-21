import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import type { PublicConfig, SyncStatus } from "../types";

export async function getConfig(): Promise<PublicConfig> {
  return invoke("get_config");
}

export async function getStatus(): Promise<SyncStatus> {
  return invoke("get_status");
}

export async function saveSetup(
  apiUrl: string,
  syncFolder: string,
  deviceName: string,
): Promise<PublicConfig> {
  return invoke("save_setup", {
    apiUrl,
    syncFolder,
    deviceName,
  });
}

export async function pauseSync(): Promise<void> {
  return invoke("pause_sync");
}

export async function resumeSync(): Promise<void> {
  return invoke("resume_sync");
}

export async function updateSettings(input: {
  apiUrl?: string;
  syncFolder?: string;
  deviceName?: string;
}): Promise<PublicConfig> {
  return invoke("update_settings", {
    apiUrl: input.apiUrl,
    syncFolder: input.syncFolder,
    deviceName: input.deviceName,
  });
}

export async function resetDevice(): Promise<PublicConfig> {
  return invoke("reset_device");
}

export async function getLogPath(): Promise<string> {
  return invoke("get_log_path");
}

export async function isSetupComplete(): Promise<boolean> {
  return invoke("is_setup_complete");
}

export function onSyncStatus(callback: (status: SyncStatus) => void): Promise<UnlistenFn> {
  return listen<SyncStatus>("sync-status", (event) => callback(event.payload));
}

export function onConnectionStatus(
  callback: (status: string) => void,
): Promise<UnlistenFn> {
  return listen<string>("connection-status", (event) => callback(event.payload));
}
