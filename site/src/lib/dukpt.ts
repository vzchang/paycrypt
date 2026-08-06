// Glue only: every cryptographic value comes from the WASM library.

/** One derivation frame returned by the WASM `*_steps` entry points. */
export type Step = { label: string; bytes: number[]; hex: string; note: string };

/** The canonical published demo BDK (X9.24-1 test vector). Synthetic. */
export const DEMO_BDK = "0123456789ABCDEFFEDCBA9876543210";

/** Build a TDES KSN from a counter, using the demo IKSN `FFFF9876543210E0`. */
export function ksnHex(counter: number): string {
  return "FFFF9876543210E0" + counter.toString(16).toUpperCase().padStart(4, "0");
}

/** Run a WASM step call, returning `[]` if it rejects the inputs. */
export function safeSteps(fn: () => unknown): Step[] {
  try {
    return fn() as Step[];
  } catch {
    return [];
  }
}
