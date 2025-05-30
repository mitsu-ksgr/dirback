/**
 *  API: Get target
 */

import { dispatch } from "./dispatcher";
import type { Target } from "$lib/types/target";

export async function getTarget(target_id: string): Promise<Target | null> {
  return await dispatch({
    type: "GetTarget",
    payload: {
      target_id,
    },
  });
}
