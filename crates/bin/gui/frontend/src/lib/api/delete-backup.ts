/**
 *  API: Delete backup
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./dispatcher";
import type { BackupEntry } from "$lib/types/backup-entry";

export async function deleteBackup(target_id: string, backup_id: number): Promise<BackupEntry | null> {
  return await dispatch({
    type: "DeleteBackup",
    payload: {
      target_id,
      backup_id,
    },
  });
}
