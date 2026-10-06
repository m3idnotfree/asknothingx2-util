define_mime_type! {
    pub enum Message {
        Rfc822 => {
            mime: "message/rfc822",
            extensions: ["eml", "mime"],
        },
        Partial => {
            mime: "message/partial",
            extensions: [],
        },
        ExternalBody => {
            mime: "message/external-body",
            extensions: [],
        },
    }
}
