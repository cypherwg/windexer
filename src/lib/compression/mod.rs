use zstd;

pub fn compress(data: &[u8]) -> Vec<u8> {
    zstd::encode_all(data, 0).unwrap()
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    zstd::decode_all(data).unwrap()
}

TODO: ZK compression