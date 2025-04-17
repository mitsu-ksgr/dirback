/**
 * Target Type
 *
 * Rust: crates/lib/dirback/src/usecase/dto/target.rs
 */

import { BackupEntry } from "./backup-entry";

export interface Target {
    id: string;
    name: string;
    path: string;
    backups: BackupEntry[];
}

