// Reference shim over github.com/moov-io/dukpt for differential testing.
//
// Usage:
//   ref_moov tdes-ipek <bdk_hex> <ksn_hex>       -> prints IPEK hex
//   ref_moov tdes-txn  <bdk_hex> <ksn_hex>       -> prints transaction key hex
//   ref_moov aes-ik    <bdk_hex> <ikid_hex>      -> prints AES Initial Key hex
//   ref_moov aes-pin   <ik_hex>  <ksn_hex>       -> prints AES PIN working key hex
//
package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: ref_moov MODE ARG...")
		os.Exit(2)
	}
	mode := os.Args[1]
	switch mode {
	case "tdes-ipek", "tdes-txn", "aes-ik", "aes-pin":
		fmt.Fprintf(os.Stderr, "bind moov-io API for %q in CI setup\n", strings.ToUpper(mode))
		os.Exit(3)
	default:
		fmt.Fprintf(os.Stderr, "unknown mode %q\n", mode)
		os.Exit(2)
	}
}
