/**
 *  Mock data.
 */
import type { Target } from "$lib/types/target";
import type { BackupEntry } from "$lib/types/backup-entry";

const DIRBACK_BASE_PATH = `/tmp/dirback/.data/`;

function zfill(n: number, digit: number = 2): string {
  return n.toString().padStart(digit, '0');
}

function randInt(min: number, max: number): number {
  const lower = Math.ceil(min);
  const upper = Math.floor(max);
  return Math.floor(Math.random() * (upper - lower + 1)) + lower;
}

function makeTimestamps(msec: number): [string, string] {
  const t = new Date(msec);

  const y = t.getUTCFullYear();
  const m = zfill(t.getUTCMonth() + 1); // 0-indexed
  const d = zfill(t.getUTCDate());
  const hh = zfill(t.getUTCHours());
  const mm = zfill(t.getUTCMinutes());
  const ss = zfill(t.getUTCSeconds());

  return [
    `${y}${m}${d}T${hh}${mm}${ss}Z`,
    t.toISOString(),
  ];
}

export function generateNewMockBackup(target: Target, note: string): BackupEntry {
  const prev = target.backups.at(-1);

  const id = prev === undefined ? 1 : prev.id + 1;
  const idx = zfill(id, 4);
  const ts = makeTimestamps(Date.now());

  return {
    id: id,
    path: `${DIRBACK_BASE_PATH}/targets/${target.id}/backups/${idx}_${ts[0]}.tar.gz`,
    timestamp: ts[1],
    note: note,
  };
}

export function generateMockTargets(): Target[] {
  const now = Date.now();
  const timerange = [now - (365 * 24 * 60 * 60 * 1000), now];

  let targets: Target[] = [];

  for (let i = 1; i <= 10; ++i) {
    // Target info
    const idx = zfill(i, 4);
    const id = crypto.randomUUID();
    const name = `Mock Target ${idx}`;
    const path = `/tmp/dirback/projects/${id}`;
    let backups: BackupEntry[] = [];

    // Backups
    const backupCount = 10 - i;
    let backupTimes = [];
    for (let k = 0; k < backupCount; ++k) {
      backupTimes.push(randInt(timerange[0], timerange[1]));
    }
    backupTimes.sort((a, b) => b - a);

    for (let k = 0; k < backupCount; ++k) {
      const bkId = k + 1;
      const bkIdx = zfill(bkId, 4);
      const ts = makeTimestamps(backupTimes[k]);

      backups.push({
        id: bkId,
        path: `${DIRBACK_BASE_PATH}/targets/${id}/backups/${bkIdx}_${ts[0]}.tar.gz`,
        timestamp: ts[1],
        note: `The ${name}'s backup ${bkId}.`,
      });
    }

    // Add targets
    targets.push({ id, name, path, backups });
  }

  return targets;
}

