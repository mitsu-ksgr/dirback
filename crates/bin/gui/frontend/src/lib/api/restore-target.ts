/**
 *  API: Restore target
 */

import { dispatch } from "./dispatcher";

export async function restoreTarget(target_id: string, backup_id: number): Promise<null> {
  return await dispatch({
    type: "RestoreTarget",
    payload: {
      target_id,
      backup_id,
    },
  });
}
