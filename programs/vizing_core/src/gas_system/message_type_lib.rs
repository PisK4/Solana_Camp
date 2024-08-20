use anchor_lang::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MessageType {
    Default,
    StandardActivate,
    ArbitraryActivate,
    MessagePost,
    NativeTokenSend,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AdditionParamsType {
    SingleSend,
    ERC20Handler,
    MultiMany2One,
    MultiUniversal,
    MaxMode,
}

impl MessageType {
    pub fn from_byte(byte: u16) -> Self {
        match byte {
            0x00 => MessageType::Default,
            0x01 => MessageType::StandardActivate,
            0x02 => MessageType::ArbitraryActivate,
            0x03 => MessageType::MessagePost,
            0x04 => MessageType::NativeTokenSend,
            _ => {
                panic!("Unknown byte value: {}", byte);
            }
        }
    }

    pub fn fetch_msg_mode(message: &[u16]) -> Self {
        if message.is_empty() {
            return MessageType::Default;
        }
        let message_slice = message[0];
        MessageType::from_byte(message_slice)
    }
}

impl AdditionParamsType {
    pub fn from_byte(byte: u16) -> Self {
        match byte {
            0x01 => AdditionParamsType::SingleSend,
            0x02 => AdditionParamsType::ERC20Handler,
            0x03 => AdditionParamsType::MultiMany2One,
            0x04 => AdditionParamsType::MultiUniversal,
            0xFF => AdditionParamsType::MaxMode,
            _ => {
                panic!("Unknown byte value: {}", byte);
            }
        }
    }

    pub fn fetch_msg_mode(message: &[u16]) -> Self {
        let message_slice = message[0];
        AdditionParamsType::from_byte(message_slice)
    }
}

pub fn process_message(message: &[u16]) -> MessageType {
    MessageType::fetch_msg_mode(&message)
}
