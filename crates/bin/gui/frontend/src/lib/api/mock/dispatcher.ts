/**
 *  Mock dispatcher.
 */
import type { Command } from "./../dispatcher";
import type { Target } from "$lib/types/target";
import type { BackupEntry } from "$lib/types/backup-entry";
import { generateMockTargets, generateNewMockBackup } from "./data";

const mockTargets: Target[] = generateMockTargets();

export async function mockDispatch<T>(cmd: Command): Promise<T> {
  switch (cmd.type) {
    case "ListTargets":
      return mockTargets as T;

    case "GetTarget":
      return getTarget(cmd.payload.target_id) as T;

    case "BackupTarget":
      return backupTarget(cmd.payload.target_id, cmd.payload.note) as T;

    case "DeleteBackup":
      return deleteBackup(cmd.payload.target_id, cmd.payload.backup_id) as T;

    case "DeleteTarget":
      return deleteTarget(cmd.payload.target_id) as T;

    case "RegisterTarget":
      return registerTarget(cmd.payload.name, cmd.payload.path) as T;

    case "RestoreTarget":
      restoreTarget(cmd.payload.target_id, cmd.payload.backup_id);
      return null as T;

    default:
      throw new Error(`Unknown command: ${cmd.type}`);
  }
}


//-----------------------------------------------------------------------------
// Mock APIs
//-----------------------------------------------------------------------------
function findMockTarget(target_id: string): Target | null {
  const target = mockTargets.find((t) => t.id === target_id);
  if (target === undefined) {
    console.log(`Mock#findMockTarget: target not found.`);
    return null;
  }
  console.log(`Mock#findMockTarget: target found.`);
  return target;
}

function getTarget(target_id: string): Target | null {
  console.log(`Mock: getTarget: target_id = ${target_id}`);
  return findMockTarget(target_id);
}

function backupTarget(target_id: string, note: string): Target {
  const target = findMockTarget(target_id);
  if (target === null) {
    throw new Error(`Target not found: '${target_id}'`);
  }

  const backup = generateNewMockBackup(target, note);
  target.backups.push(backup);

  return target;
}

function deleteBackup(target_id: string, backup_id: number): BackupEntry {
  const target = findMockTarget(target_id);
  if (target === null) {
    throw new Error(`Target not found: '${target_id}'`);
  }

  const idx = target.backups.findIndex(be => be.id === backup_id);
  if (idx === -1) {
    throw new Error(`Backup not found: '${backup_id}'`);
  }

  const deleted = target.backups.splice(idx, 1);
  return deleted[0];
}

function deleteTarget(target_id: string): Target {
  const idx = mockTargets.findIndex(t => t.id === target_id);
  if (idx === -1) {
    throw new Error(`Target not found: '${target_id}'`);
  }

  const deleted = mockTargets.splice(idx, 1);
  return deleted[0];
}

function registerTarget(name: string, path: string): Target {
  if (name === "") {
    throw new Error(`Invalid name.`);
  }

  if (path === "") {
    throw new Error(`Invalid path.`);
  }

  const backups: BackupEntry[] = [];
  const id = crypto.randomUUID();

  const target: Target = {id, name, path, backups};
  mockTargets.push(target);

  return target;
}

function restoreTarget(target_id: string, backup_id: number) {
  const target = findMockTarget(target_id);
  if (target === null) {
    throw new Error(`Target not found: '${target_id}'`);
  }

  const idx = target.backups.findIndex(be => be.id === backup_id);
  if (idx === -1) {
    throw new Error(`Backup not found: '${backup_id}'`);
  }

  // do nothing.

  return;
}

