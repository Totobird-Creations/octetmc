pub mod handshake;

pub mod status;

pub mod login;

pub mod config;

pub mod play;


pub mod decode;


/// ### Safety
/// You probably shouldn't be implementing this.
#[diagnostic::on_unimplemented(
    label = "`{Self}` is not a packet bound",
    message = "`{Self}` is not a packet bound"
)]
pub unsafe trait PacketBound { }
pub struct BoundC2S;
unsafe impl PacketBound for BoundC2S { }
pub struct BoundS2C;
unsafe impl PacketBound for BoundS2C { }

/// ### Safety
/// You probably shouldn't be implementing this.
#[diagnostic::on_unimplemented(
    label = "`{Self}` is not a packet state",
    message = "`{Self}` is not a packet state"
)]
pub unsafe trait PacketState { }
pub struct StateHandshake;
unsafe impl PacketState for StateHandshake { }
pub struct StateStatus;
unsafe impl PacketState for StateStatus { }
pub struct StateLogin;
unsafe impl PacketState for StateLogin { }
pub struct StateConfig;
unsafe impl PacketState for StateConfig { }
pub struct StatePlay;
unsafe impl PacketState for StatePlay { }

/// ### Safety
/// You probably shouldn't be implementing this.
#[diagnostic::on_unimplemented(
    label = "`{Self}` is not a valid packet bound-state pair",
    message = "`{Self}` is not a valid packet bound-state pair"
)]
pub unsafe trait PacketBoundState {}
unsafe impl PacketBoundState for (BoundC2S, StateHandshake,) { }
unsafe impl PacketBoundState for (BoundC2S, StateStatus,) { }
unsafe impl PacketBoundState for (BoundS2C, StateStatus,) { }
unsafe impl PacketBoundState for (BoundC2S, StateLogin,) { }
unsafe impl PacketBoundState for (BoundS2C, StateLogin,) { }
unsafe impl PacketBoundState for (BoundC2S, StateConfig,) { }
unsafe impl PacketBoundState for (BoundS2C, StateConfig,) { }
unsafe impl PacketBoundState for (BoundC2S, StatePlay,) { }
unsafe impl PacketBoundState for (BoundS2C, StatePlay,) { }


#[derive(Default)]
pub struct BufHead {
    head : usize
}

impl BufHead {

    #[inline(always)]
    pub fn consumed(&self) -> usize { self.head }

}


macro_rules! packet_group { (
    type Bound = $bound:ty;
    type State = $state:ty;
    type Error<$errorlt:lifetime> = $error:ty;
    $vis:vis enum $ident:ident $( < $lt:lifetime $(,)? > )? {
        $( $varident:ident ( $varinner:ty $(,)? ) ),* $(,)?
    }
) => {

    #[derive(Debug, Clone)]
    $vis enum $ident $( < $lt , > )? {
        $( $varident ( $varinner ) , )*
    }

    impl $( < $lt , > )? $crate::packet::decode::PacketDecodeGroup for $ident $( < $lt , > )? {
        type Bound = $bound;
        type State = $state;

        $crate::packet::packet_group_output!{ $ident , $( $lt , )? }
        type Error<'errorlt> = $error;

        #[expect(non_snake_case)]
        fn decode_prefixed<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>(
            buf  : $crate::packet::decode::DecodeBuf<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>,
            head : &mut $crate::packet::BufHead
        ) -> Result<
            Self::Output<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>,
            $crate::packet::decode::UnknownPrefix<Self::Error<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>>
        > {
            let prefix = buf.read(head).map_err(|e| $crate::packet::decode::UnknownPrefix::Error(Self::Error::from(e)))?;
            $( if (prefix == <$varinner as $crate::packet::decode::PacketDecode>::PREFIX) {
                return Ok($ident::$varident(
                    <$varinner as $crate::packet::decode::PacketDecode>::decode(buf, head)
                        .map_err(|e| $crate::packet::decode::UnknownPrefix::Error(
                            Into::<Self::Error<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>>::into(e)
                        ))?
                ));
            } )*
            Err($crate::packet::decode::UnknownPrefix::UnknownPrefix(prefix))
        }

    }

} }
macro_rules! packet_group_output {
    ( $ident:ident , $lt:lifetime $(,)? ) => {
        #[expect(non_snake_case)]
        type Output<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME> = $ident<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>;
    };
    ( $ident:ident $(,)? ) => {
        #[expect(non_snake_case)]
        type Output<'__PACKET_GROUP_OUTPUT_UNUSED_LIFETIME> = $ident;
    };
}
pub(crate) use packet_group;
pub(crate) use packet_group_output;
