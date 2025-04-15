/**
 *  Command types
 */

export type Command =
  | { type: "ListTarget"; payload: {} }
  | { type: "GetTarget"; payload: { id: string } }
;

