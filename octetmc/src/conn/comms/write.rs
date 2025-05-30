use super::ConnPeerComms;
use crate::conn::{ ConnPeerResult, ConnPeerError };
use octetmc_protocol::packet::encode::{ PacketEncodeGroup, PacketPartEncode, EncodeBuf };


impl ConnPeerComms {

    pub(crate) async fn send_packet<P>(&mut self, packet : &P) -> ConnPeerResult
    where
        P : PacketEncodeGroup
    {
        let mut buf = EncodeBuf::default();
        P::encode_prefixed(packet, &mut buf);

        todo!()
    }

}
