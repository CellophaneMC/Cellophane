use bit_set::BitSet;
use uuid::Uuid;

use crate::{Fixed256VecU8, FixedBitSet, LengthInferredVecU8, VarIntPrefixedVec};
use crate::Angle;
use crate::io::Decoder;
use crate::VarInt;

macro_rules! user_type {
    (VarInt) => {
        i32
    };
    (Angle) => {
        u8
    };
    (VarIntPrefixedVec <$inner:ident>) => {
        Vec<$inner>
    };
    (ShortPrefixedVec <$inner:ident>) => {
        Vec<$inner>
    };
    (LengthInferredVecU8) => {
        Vec<u8>
    };
    (Fixed256VecU8) => {
        Vec<u8>
    };
    (FixedBitSet<$_:literal>) => {
        BitSet
    };
    ($typ:ty) => {
        $typ
    };
}

macro_rules! encoder_type {
    (VarInt, $e:expr) => {
        VarInt($e)
    };
    (Angle, $e:expr) => {
        Angle($e)
    };
    (VarIntPrefixedVec <$inner:ident>, $e:expr) => {
        VarIntPrefixedVec::from($e.as_slice())
    };
    (ShortPrefixedVec <$inner:ident>, $e:expr) => {
        ShortPrefixedVec::from($e.as_slice())
    };
    (LengthInferredVecU8, $e:expr) => {
        LengthInferredVecU8::from($e.as_slice())
    };
    (Fixed256VecU8, $e:expr) => {
        Fixed256VecU8::from($e.as_slice())
    };
    ($typ:ty, $e:expr) => {
        $e
    };
}

macro_rules! packets {
    (
        $(
            $packet:ident {
                $(
                    $field:ident $typ:ident $(<$generics:ident>)? $(<$c:literal>)?
                );* $(;)?
            } $(,)?
        )*
    ) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $packet {
                $(
                    pub $field: user_type!($typ $(<$generics>)? $(<$c>)?),
                )*
            }

            #[allow(clippy::useless_conversion)]
            #[allow(unused_imports)]
            #[allow(unused_variables)]
            impl crate::Decoder for $packet {
                fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {

                    $(
                        let $field = <$typ $(<$generics>)? $(<$c>)?>::read(reader)
                                .map_err(|e| crate::error::Error::FieldDecode {
                                    field: stringify!($field).to_string(),
                                    packet: stringify!($packet).to_string(),
                                    source: Box::new((e)),
                                })?
                                .into();
                    )*

                    Ok(Self {
                        $(
                            $field,
                        )*
                    })
                }
            }

            #[allow(clippy::useless_conversion)]
            #[allow(unused_imports)]
            #[allow(unused_variables)]
            impl crate::Encoder for $packet {
                    fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {

                    $(
                        encoder_type!{$typ $(<$generics>)? $(<$c>)?, self.$field}
                            .write(writer)
                            .map_err(|e| crate::error::Error::FieldEncode {
                                field: stringify!($field).to_string(),
                                packet: stringify!($packet).to_string(),
                                source: Box::new(crate::error::Error::from((e))),
                            })?;
                    )*

                    Ok(())
                }
            }
        )*
    };
}

macro_rules! enum_packets {
    (
        $ident:ident {
            $($opcode:literal = $packet:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $packet($packet),
            )*
        }

        impl crate::Decoder for $ident {

            fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
                let opcode = crate::VarInt::read(reader)?.0;
                match opcode {
                    $(
                        $opcode => Ok($ident::$packet($packet::read(reader)?)),
                    )*
                    _ => Err(crate::error::Error::InvalidDiscriminant(opcode)),
                }
            }
        }

        impl crate::Encoder for $ident {
            fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
                match self {
                    $(
                        $ident::$packet(packet) => {
                            crate::VarInt($opcode).write(writer)?;
                            packet.write(writer)?;
                        }
                    )*
                }
                Ok(())
            }
        }
    };
}

pub mod client;
pub mod server;
