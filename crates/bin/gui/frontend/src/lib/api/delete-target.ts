/**
 *  API: Delete target
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./types/command";
import type { Target } from "./types/target";

export async function deleteTarget(target_id: string): Promise<Target> {
  return await dispatch({
    type: "DeleteTarget",
    payload: {
      target_id,
    },
  });
}

