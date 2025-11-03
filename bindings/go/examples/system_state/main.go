// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package main

import (
	"fmt"
	"log"

	sdk "bindings/iota_sdk_ffi"
)

func main() {
	client := sdk.GraphQlClientNewDevnet()

	systemState, err := client.LatestSystemState()
	if err.(*sdk.SdkFfiError) != nil {
		log.Fatalf("Failed to get latest system state: %v", err)
	}

	fmt.Printf("Latest system state: %v", *systemState.ValueAsJson)
}