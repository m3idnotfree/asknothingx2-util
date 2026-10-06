define_mime_type! {
    pub enum Multipart {
        FormData => {
            mime: "multipart/form-data",
            extensions: [],
        },
        ByteRanges => {
            mime: "multipart/byteranges",
            extensions: [],
        },
        Mixed => {
            mime: "multipart/mixed",
            extensions: [],
        },
        Alternative => {
            mime: "multipart/alternative",
            extensions: [],
        },
    }
}
