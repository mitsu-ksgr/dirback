/**
 *  API: Delete target
 */

import { dispatch } from "./dispatcher";
import type { Target } from "$lib/types/target";

export async function deleteTarget(target_id: string): Promise<Target> {
  return await dispatch({
    type: "DeleteTarget",
    payload: {
      target_id,
    },
  });
}
