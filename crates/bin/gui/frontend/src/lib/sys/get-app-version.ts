/**
 *  sys: get-version
 */

import { IS_MOCK } from "../config";
import { getVersion } from '@tauri-apps/api/app';

export async function getAppVersion(): Promise<string> {
  if (IS_MOCK) {
    return "0.1.2";
  } else {
    return await getVersion();
  }
}
