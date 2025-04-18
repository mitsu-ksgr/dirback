/**
 *  API: Register target
 */

import { dispatch } from "./dispatcher";
import type { Target } from "$lib/types/target";

export async function registerTarget(
  name: string,
  path: string,
): Promise<Target> {
  return await dispatch({
    type: "RegisterTarget",
    payload: {
      name,
      path,
    },
  });
}
