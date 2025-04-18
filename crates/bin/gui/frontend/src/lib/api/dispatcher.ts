/**
 *  lib/api/dispatcher.ts
 */

import { invoke } from "@tauri-apps/api/core";

import { IS_MOCK } from "../config";
import { mockDispatch } from "./mock/dispatcher";

export type Command =
  | { type: "BackupTarget"; payload: { target_id: string; note: string } }
  | { type: "DeleteBackup"; payload: { target_id: string; backup_id: number } }
  | { type: "DeleteTarget"; payload: { target_id: string } }
  | { type: "GetTarget"; payload: { target_id: string } }
  | { type: "ListTargets"; payload: {} }
  | { type: "RegisterTarget"; payload: { name: string; path: string } }
  | {
      type: "RestoreTarget";
      payload: { target_id: string; backup_id: number };
    };

export async function dispatch<T>(cmd: Command): Promise<T> {
  if (IS_MOCK) {
    return mockDispatch<T>(cmd);
  } else {
    return tauriDispatch<T>(cmd);
  }
}

//-----------------------------------------------------------------------------
// Dispatch
//-----------------------------------------------------------------------------
async function tauriDispatch<T>(cmd: Command): Promise<T> {
  return await invoke("command_dispatcher", { cmd });
}
