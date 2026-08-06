#!/usr/bin/env python3
"""Reference shim over the `psec` library for differential testing.

Usage:
    ref_psec.py iso0 <pin> <pan>   -> prints the ISO-0 clear PIN block hex
    ref_psec.py iso2 <pin>         -> prints the ISO-2 clear PIN block hex
    ref_psec.py iso3dec <block_hex> <pan>  -> prints the decoded PIN

psec covers ISO 9564 formats 0/2/3/4 (not format 1). Pin the exact psec
version in CI (see scripts/README.md).
"""
import sys


def main() -> int:
    import psec.pinblock as pb  # type: ignore

    mode = sys.argv[1]
    if mode == "iso0":
        pin, pan = sys.argv[2], sys.argv[3]
        print(pb.encode_pinblock_iso_0(pin, pan).hex().upper())
    elif mode == "iso2":
        pin = sys.argv[2]
        print(pb.encode_pinblock_iso_2(pin).hex().upper())
    elif mode == "iso3dec":
        block, pan = bytes.fromhex(sys.argv[2]), sys.argv[3]
        print(pb.decode_pinblock_iso_3(block, pan))
    else:
        print(f"unknown mode {mode}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
