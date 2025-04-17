/**
 *  sys: get-version
 */

import { IS_MOCK } from "../config";
import { getVersion } from '@tauri-apps/api/app';

export async function getAppVersion(): string {
  if (IS_MOCK) {
    return "0.0.1";
  } else {
    return await getVersion();
  }
}
