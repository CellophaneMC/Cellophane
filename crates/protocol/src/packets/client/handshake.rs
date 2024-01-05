use std::io::{Read, Write};

use bevy_reflect::Reflect;

use crate::Encoder;
use crate::error::Error;
use crate::error::Result;
use crate::io::Decoder;

use super::*;

packets!(
    Handshake {
        protocol_version VarInt;
        server_address String;
        server_port u16;
        next_state HandshakeState;
    }
);


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
pub enum HandshakeState {
    #[default]
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Configuration = 3,
    Play = 4,
}

impl Decoder for HandshakeState {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let discriminant = VarInt::read(reader)?.0;
        match discriminant {
            0 => Ok(HandshakeState::Handshaking),
            1 => Ok(HandshakeState::Status),
            2 => Ok(HandshakeState::Login),
            3 => Ok(HandshakeState::Configuration),
            4 => Ok(HandshakeState::Play),
            _ => Err(Error::InvalidDiscriminant(discriminant)),
        }
    }
}

impl Encoder for HandshakeState {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        let discriminant = match self {
            HandshakeState::Handshaking => 0,
            HandshakeState::Status => 1,
            HandshakeState::Login => 2,
            HandshakeState::Configuration => 3,
            HandshakeState::Play => 4,
        };
        VarInt(discriminant).write(writer)
    }
}
