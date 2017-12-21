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
        assert_eq!(header.ncsrc >> 4, 0);
        assert!(header.payload_type < 0b10000000);

        data.put_u8(0b01000000 + 
                ((header.padding as u8) << 5) +
                ((header.extension as u8) << 4) + header.ncsrc);
        data.put_u8(header.payload_type + ((header.marker as u8) << 7));
        data.put_u16::<BigEndian>(header.seq);
        data.put_u32::<BigEndian>(header.timestamp);
        data.put_u32::<BigEndian>(header.ssrc);
        data.freeze()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rtpheader_to_bytes() {
        let header = RtpHeader {
            padding: true,
            extension: false,
            ncsrc: 13,
            marker: true,
            payload_type: 0x7c,
            seq: 0x9345,
            ssrc: 0x0f0fff0,
            timestamp: 0xcfe1341,
        };

        let bytes: Bytes = Bytes::from(header);

        assert_eq!(bytes[0], 0b_01_1_0_1101);
        assert_eq!(bytes[1], 0b1_111_1100);
        assert_eq!(bytes[2], 0x93);
        assert_eq!(bytes[3], 0x45);
        assert_eq!(bytes[4], 0x0c);
        assert_eq!(bytes[8], 0x00);
    }
}