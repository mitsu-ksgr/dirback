/**
 *  Mock dispatcher.
 */
import type { Command } from "./../types/command";

const mockTargets = [
  { id: "mock-target-001" },
  { id: "mock-target-002" },
  { id: "mock-target-003" },
];

export async function mockDispatch<T>(cmd: Command): Promise<T> {
  switch (cmd.type) {
    case "ListTargets":
      return mockTargets as T;

    case "GetTarget":
      const target = mockTargets.find((t) => t.id === cmd.payload.id);
      if (!target) {
        //throw new Error(`Target not found`);
        return null as T;
      }
      return target as T;

    default:
      throw new Error(`Unknown command: ${cmd.type}`);
  }
}

