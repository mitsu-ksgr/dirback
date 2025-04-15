/**
 * api/types/backup-entry.ts
 *
 * Rust: crates/lib/dirback/src/usecase/dto/backup_entry.rs
 */

import { Timestamp } from "./timestamp";

export interface BackupEntry {
    id: number;
    path: String;
    timestamp: Timestamp;
    note: String;
}
