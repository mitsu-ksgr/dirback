/**
 *  API: List targets
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./types/command";
import type { Target } from "./types/target";

export async function listTargets(): Promise<Target[]> {
  return await dispatch({
    type: "ListTargets",
    payload: {},
  });
}
