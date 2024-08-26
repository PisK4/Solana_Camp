use anchor_lang::prelude::*;

pub mod message_monitor {
    use super::*;

    pub fn slice_message(message: &[u8]) -> Option<([u8; 32], u32, u64, Vec<u8>)> {
        if message.len() < 48 {
            return None;
        }
        let contract_addr = &message[1..33];
        let mut contract_addr_array = [0u8; 32];
        contract_addr_array.copy_from_slice(contract_addr);

        let mut gas_limit_bytes = [0u8; 4];
        for (i, &value) in message[33..35].iter().enumerate() {
            gas_limit_bytes[i * 2] = (value >> 8) as u8;
            gas_limit_bytes[i * 2 + 1] = (value & 0xFF) as u8;
        }
        let gas_limit = u32::from_be_bytes([
            gas_limit_bytes[0],
            gas_limit_bytes[1],
            gas_limit_bytes[2],
            gas_limit_bytes[3],
        ]);

        let mut max_fee_per_gas_bytes = [0u8; 8];
        for (i, &value) in message[36..40].iter().enumerate() {
            max_fee_per_gas_bytes[i * 2] = (value >> 8) as u8;
            max_fee_per_gas_bytes[i * 2 + 1] = (value & 0xFF) as u8;
        }
        let max_fee_per_gas = u64::from_be_bytes(max_fee_per_gas_bytes);

        let standard_message_bytes = &message[44..48];

        let signature = message[44..].to_vec();
        Some((contract_addr_array, gas_limit, max_fee_per_gas, signature))
    }

    pub fn slice_transfer(message: &[u8]) -> Option<([u8; 32], u32)> {
        let receiver = &message[1..33];
        let mut receiver_addr_array = [0u8; 32];
        receiver_addr_array.copy_from_slice(receiver);

        let mut gas_limit_bytes = [0u8; 4];
        for (i, &value) in message[33..35].iter().enumerate() {
            gas_limit_bytes[i * 2] = (value >> 8) as u8;
            gas_limit_bytes[i * 2 + 1] = (value & 0xFF) as u8;
        }
        let gas_limit = u32::from_be_bytes([
            gas_limit_bytes[0],
            gas_limit_bytes[1],
            gas_limit_bytes[2],
            gas_limit_bytes[3],
        ]);
        Some((receiver_addr_array, gas_limit))
    }
}
