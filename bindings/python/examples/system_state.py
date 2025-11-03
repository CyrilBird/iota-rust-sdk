# Copyright (c) 2025 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from lib.iota_sdk_ffi import *

import asyncio


async def main():
    client = GraphQlClient.new_devnet()

    system_state = await client.latest_system_state()
    print("Latest system state:", system_state.value_as_json)


if __name__ == "__main__":
    asyncio.run(main())