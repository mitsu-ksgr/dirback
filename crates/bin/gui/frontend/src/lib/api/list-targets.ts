/**
 *  API: List targets
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./dispatcher";
import type { Target } from "$lib/types/target";

export async function listTargets(): Promise<Target[]> {
  return await dispatch({
    type: "ListTargets",
    payload: {},
  });
}
