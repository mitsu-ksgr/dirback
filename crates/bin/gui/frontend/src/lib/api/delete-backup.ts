/**
 *  API: Delete backup
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./types/command";
import type { Target } from "./types/target";

export async function deleteBackup(target_id: string, backup_id: int): Promise<Target | null> {
  return await dispatch({
    type: "DeleteBackup",
    payload: {
      target_id,
      backup_id,
    },
  });
}
