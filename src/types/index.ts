export interface PublicConfig {
  api_url: string;
  sync_folder: string | null;
  device_id: string;
  device_name: string;
  graph_id: string | null;
  enabled: boolean;
  paused: boolean;
  setup_complete: boolean;
  stun_servers: string[];
  turn_servers: Array<{
    urls: string[];
    username?: string;
    credential?: string;
  }>;
  protocol_version: number;
}

export interface SyncStatus {
  status: string;
  connected: boolean;
  paused: boolean;
  last_sync: string | null;
  folder: string | null;
  device_name: string;
  pending_events: number;
  failed_events: number;
}

export type AppView = "setup" | "status" | "settings";
