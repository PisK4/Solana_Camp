pub mod message_monitor {

    pub fn slice_message(message: &[u8]) -> Option<(u8, [u8; 32], u32, u64, Vec<u8>)> {
        if message.len() < 48 {
            return None;
        }

        let contract_addr = &message[1..33];
        let mut contract_addr_array = [0u8; 32];
        contract_addr_array.copy_from_slice(contract_addr);

        let gas_limit_bytes = &message[33..36];

        let gas_limit = u32::from_le_bytes([
            0,
            gas_limit_bytes[0],
            gas_limit_bytes[1],
            gas_limit_bytes[2],
        ]);

        let mut max_fee_per_gas_bytes = [0u8; 8];
        for (i, &value) in message[36..44].iter().enumerate() {
            max_fee_per_gas_bytes[i] = value;
        }
        let max_fee_per_gas = u64::from_le_bytes(max_fee_per_gas_bytes);

        let _standard_message_bytes = &message[44..48];

        let signature = message[44..].to_vec();
        Some((
            message[0],
            contract_addr_array,
            gas_limit,
            max_fee_per_gas,
            signature,
        ))
    }

    pub fn slice_transfer(message: &[u8]) -> Option<([u8; 32], u32)> {
        let receiver = &message[1..33];
        let mut receiver_addr_array = [0u8; 32];
        receiver_addr_array.copy_from_slice(receiver);

        let gas_limit_bytes = &message[33..36];

        let gas_limit = u32::from_be_bytes([
            0,
            gas_limit_bytes[0],
            gas_limit_bytes[1],
            gas_limit_bytes[2],
        ]);

        Some((receiver_addr_array, gas_limit))
    }
}
