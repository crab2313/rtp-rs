use bytes::{BigEndian, BufMut, Bytes, BytesMut};
use byteorder::ReadBytesExt;


pub struct RtpHeader {
    ncsrc: u8,
    marker: bool,
    padding: bool,
    extension: bool,
    payload_type: u8,
    seq: u16,
    timestamp: u32,
    ssrc: u32,
}

pub struct RtpPacket {
    header: RtpHeader,
}

impl From<RtpHeader> for Bytes {
    fn from(header: RtpHeader) -> Bytes {
        let mut data = BytesMut::new();

        // since we only need the lower 4 bits of header.ncsrc
        assert_eq!(header.ncsrc & 0xf, 0);

        data.put_u8(0b01000000 + 
                (header.padding as u8) << 5 + 
                (header.extension as u8) << 4 + header.ncsrc);
        data.put_u8(header.payload_type + (header.marker as u8) << 7);
        data.put_u16::<BigEndian>(header.seq);
        data.put_u32::<BigEndian>(header.timestamp);
        data.put_u32::<BigEndian>(header.ssrc);
        data.freeze()
    }
}

