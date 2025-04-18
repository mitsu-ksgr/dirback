/**
 *  API: Backup target
 */

import { dispatch } from "./dispatcher";
import type { Target } from "$lib/types/target";

export async function backupTarget(target_id: string, note: string): Promise<Target> {
  return await dispatch({
    type: "BackupTarget",
    payload: {
      target_id,
      note,
    },
  });
}
