/**
 *  API: Get target
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./types/command";
import type { Target } from "./types/target";

export async function getTarget(target_id: string): Promise<Target | null> {
  return await dispatch({
    type: "GetTarget",
    payload: {
      target_id,
    },
  });
}
