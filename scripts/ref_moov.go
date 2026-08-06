// Reference shim over github.com/moov-io/dukpt for differential testing.
//
// Usage:
//   ref_moov tdes-ipek <bdk_hex> <ksn_hex>   -> prints IPEK hex
//   ref_moov tdes-txn  <bdk_hex> <ksn_hex>   -> prints transaction key hex
//   ref_moov aes-ik    <bdk_hex> <ikid_hex>  -> prints AES Initial Key hex
//   ref_moov aes-pin   <ik_hex>  <ksn_hex>   -> prints AES PIN working key hex
//
package main

import (
	"encoding/hex"
	"fmt"
	"os"
	"strings"
)

// decodeArgs decodes the two hex arguments after the mode, or exits non-zero.
func decodeArgs() ([]byte, []byte) {
	if len(os.Args) < 4 {
		fmt.Fprintln(os.Stderr, "usage: ref_moov MODE <hex_a> <hex_b>")
		os.Exit(2)
	}
	a, errA := hex.DecodeString(os.Args[2])
	b, errB := hex.DecodeString(os.Args[3])
	if errA != nil || errB != nil {
		fmt.Fprintln(os.Stderr, "arguments must be valid hex")
		os.Exit(2)
	}
	return a, b
}

// emit prints the result as uppercase hex, matching paycrypt's to_hex_upper.
func emit(result []byte) {
	fmt.Println(strings.ToUpper(hex.EncodeToString(result)))
}

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: ref_moov MODE ARG...")
		os.Exit(2)
	}
	mode := os.Args[1]
	_, _ = decodeArgs()

	switch mode {
	case "tdes-ipek":
		fmt.Fprintln(os.Stderr, "TODO: bind des IPEK derivation; then emit(ipek)")
		os.Exit(3)
	case "tdes-txn":
		fmt.Fprintln(os.Stderr, "TODO: bind des transaction-key derivation; then emit(key)")
		os.Exit(3)
	case "aes-ik":
		fmt.Fprintln(os.Stderr, "TODO: bind aes initial-key derivation; then emit(ik)")
		os.Exit(3)
	case "aes-pin":
		fmt.Fprintln(os.Stderr, "TODO: bind aes PIN working-key derivation; then emit(key)")
		os.Exit(3)
	default:
		fmt.Fprintf(os.Stderr, "unknown mode %q\n", mode)
		os.Exit(2)
	}
}
