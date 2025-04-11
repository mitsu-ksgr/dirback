/**
 *  lib/api/dispatcher.ts
 */

import { invoke } from "@tauri-apps/api/core";

import { IS_MOCK } from "../config";
import type { Command } from "./command";
import { mockDispatch } from "./mock/dispatcher";

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


