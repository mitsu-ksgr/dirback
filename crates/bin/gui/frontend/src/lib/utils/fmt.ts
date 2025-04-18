/**
 *  lib/utils/fmt.ts
 *
 *  formatter
 */

export function fmtDateTime(timestamp: string): string {
  const dt = new Date(timestamp);
  if (isNaN(dt.getTime())) {
    return timestamp;
  }

  const pad = (n: number) => n.toString().padStart(2, "0");

  const year = dt.getFullYear();
  const mon = pad(dt.getMonth() + 1); // 0-indexed
  const day = pad(dt.getDate());
  const h = pad(dt.getHours());
  const m = pad(dt.getMinutes());
  const s = pad(dt.getSeconds());

  return `${year}/${mon}/${day} ${h}:${m}:${s}`;
}
