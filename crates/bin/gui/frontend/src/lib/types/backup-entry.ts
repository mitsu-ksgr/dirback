/**
 * BackupEntry Type
 *
 * Rust: crates/lib/dirback/src/usecase/dto/backup_entry.rs
 */

import type { Timestamp } from "./timestamp";

export interface BackupEntry {
    id: number;
    path: string;
    timestamp: Timestamp;
    note: string;
}
