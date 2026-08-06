// Glue only: every cryptographic value comes from the WASM library.

/** One derivation frame returned by the WASM `*_steps` entry points. */
export type Step = { label: string; bytes: number[]; hex: string; note: string };

/** Build a TDES KSN from a counter, using the demo IKSN `FFFF9876543210E0`. */
export function ksnHex(counter: number): string {
  return "FFFF9876543210E0" + counter.toString(16).toUpperCase().padStart(4, "0");
}
