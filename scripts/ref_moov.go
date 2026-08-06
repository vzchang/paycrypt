// Reference shim over github.com/moov-io/dukpt for differential testing.
//
// Usage:
//   ref_moov tdes-ipek <bdk_hex> <ksn_hex>   -> prints IPEK hex
//   ref_moov tdes-txn  <bdk_hex> <ksn_hex>   -> prints transaction key hex
//   ref_moov aes-ik    <bdk_hex> <ksn_hex>   -> prints AES Initial Key hex
//
// Output is uppercase hex to match paycrypt's to_hex_upper.
package main

import (
	"encoding/hex"
	"fmt"
	"os"
	"strings"

	aesdukpt "github.com/moov-io/dukpt/pkg/aes"
	desdukpt "github.com/moov-io/dukpt/pkg/des"
)

func die(msg string) {
	fmt.Fprintln(os.Stderr, msg)
	os.Exit(2)
}

func main() {
	if len(os.Args) < 4 {
		die("usage: ref_moov MODE <hex_a> <hex_b>")
	}
	mode := os.Args[1]
	a, errA := hex.DecodeString(os.Args[2])
	b, errB := hex.DecodeString(os.Args[3])
	if errA != nil || errB != nil {
		die("arguments must be valid hex")
	}

	var out []byte
	var err error
	switch mode {
	case "tdes-ipek":
		out, err = desdukpt.DerivationOfInitialKey(a, b) // a=bdk, b=ksn
	case "tdes-txn":
		var ik []byte
		if ik, err = desdukpt.DerivationOfInitialKey(a, b); err == nil {
			out, err = desdukpt.DeriveCurrentTransactionKey(ik, b)
		}
	case "aes-ik":
		out, err = aesdukpt.DerivationOfInitialKey(a, b) // a=bdk, b=ksn
	default:
		die(fmt.Sprintf("unknown mode %q", mode))
	}
	if err != nil {
		fmt.Fprintf(os.Stderr, "moov-io error: %v\n", err)
		os.Exit(3)
	}
	fmt.Println(strings.ToUpper(hex.EncodeToString(out)))
}
