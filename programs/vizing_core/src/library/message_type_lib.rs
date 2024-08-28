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
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => MessageType::Default,
            1 => MessageType::StandardActivate,
            2 => MessageType::ArbitraryActivate,
            3 => MessageType::MessagePost,
            4 => MessageType::NativeTokenSend,
            _ => {
                panic!("Unknown byte value: {}", byte);
            }
        }
    }

    pub fn fetch_msg_mode(message: &[u8]) -> Self {
        if message.is_empty() {
            return MessageType::Default;
        }
        let message_slice = message[0];
        MessageType::from_byte(message_slice)
    }
}

impl AdditionParamsType {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            1 => AdditionParamsType::SingleSend,
            2 => AdditionParamsType::ERC20Handler,
            3 => AdditionParamsType::MultiMany2One,
            4 => AdditionParamsType::MultiUniversal,
            255 => AdditionParamsType::MaxMode,
            _ => {
                panic!("Unknown byte value: {}", byte);
            }
        }
    }

    pub fn fetch_msg_mode(message: &[u8]) -> Self {
        let message_slice = message[0];
        AdditionParamsType::from_byte(message_slice)
    }
}

pub fn process_message(message: &[u8]) -> MessageType {
    MessageType::fetch_msg_mode(&message)
}
