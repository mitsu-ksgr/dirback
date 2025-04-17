/**
 *  API: Restore target
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./dispatcher";
import type { Target } from "$lib/types/target";

export async function restoreTarget(target_id: string, backup_id: number): Promise<null> {
  return await dispatch({
    type: "RestoreTarget",
    payload: {
      target_id,
      backup_id,
    },
  });
}
