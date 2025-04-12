/**
 *  API: Register target
 */

import { dispatch } from "./dispatcher";
import type { Command } from "./types/command";
import type { Target } from "./types/target";

export async function registerTarget(name: string, path: string): Promise<Target> {
  return await dispatch({
    type: "RegisterTarget",
    payload: {
      name,
      path,
    },
  });
}

