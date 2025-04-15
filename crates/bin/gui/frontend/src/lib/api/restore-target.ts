/**
 *  API: Restore target
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./types/command";
import type { Target } from "./types/target";

export async function restoreTarget(target_id: string, backup_id: int): Promise<Target | null> {
  return await dispatch({
    type: "RestoreTarget",
    payload: {
      target_id,
      backup_id,
    },
  });
}
