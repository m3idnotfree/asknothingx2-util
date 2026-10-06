define_mime_type! {
    pub enum Video {
        Mp4 => {
            mime: "video/mp4",
            extensions: ["mp4", "m4v"],
        },
        Mpeg => {
            mime: "video/mpeg",
            extensions: ["mpeg", "mpg", "mpe", "m1v", "m2v"],
        },
        Ogg => {
            mime: "video/ogg",
            extensions: ["ogv"],
        },
        Webm => {
            mime: "video/webm",
            extensions: ["webm"],
        },
        Quicktime => {
            mime: "video/quicktime",
            extensions: ["mov", "qt"],
        },
        XMsvideo => {
            mime: "video/x-msvideo",
            extensions: ["avi"],
            aliases: ["video/avi"]
        },
        XFlv => {
            mime: "video/x-flv",
            extensions: ["flv"],
        },
        XMatroska => {
            mime: "video/x-matroska",
            extensions: ["mkv"],
        },
        XMsAsf => {
            mime: "video/x-ms-asf",
            extensions: ["asf"],
        },
        XMsWm => {
            mime: "video/x-ms-wm",
            extensions: ["wm"],
        },
        XMsWmv => {
            mime: "video/x-ms-wmv",
            extensions: ["wmv"],
        },
        XMsWmx => {
            mime: "video/x-ms-wmx",
            extensions: ["wmx"],
        },
        XMsWvx => {
            mime: "video/x-ms-wvx",
            extensions: ["wvx"],
        },
        XSgiMovie => {
            mime: "video/x-sgi-movie",
            extensions: ["movie"],
        },
        XF4v => {
            mime: "video/x-f4v",
            extensions: ["f4v"],
        },
        XFli => {
            mime: "video/x-fli",
            extensions: ["fli"],
        },
        XM4v => {
            mime: "video/x-m4v",
            extensions: [],
        },
        Video3gpp => {
            mime: "video/3gpp",
            extensions: ["3gp"],
        },
        Video3gpp2 => {
            mime: "video/3gpp2",
            extensions: ["3g2"],
        },
        H261 => {
            mime: "video/h261",
            extensions: ["h261"],
        },
        H263 => {
            mime: "video/h263",
            extensions: ["h263"],
        },
        H264 => {
            mime: "video/h264",
            extensions: ["h264"],
        },
        Jpeg => {
            mime: "video/jpeg",
            extensions: ["jpgv"],
        },
        Jpm => {
            mime: "video/jpm",
            extensions: ["jpm", "jpgm"],
        },
        Mj2 => {
            mime: "video/mj2",
            extensions: ["mj2", "mjp2"],
        },
        Mp2t => {
            mime: "video/mp2t",
            extensions: ["ts"],
        },
        VndFvt => {
            mime: "video/vnd.fvt",
            extensions: ["fvt"],
        },
        VndMpegurl => {
            mime: "video/vnd.mpegurl",
            extensions: ["m3u8"],
        },
        VndMsPlayready => {
            mime: "video/vnd.ms-playready.media.pyv",
            extensions: ["pyv"],
        },
        VndVivo => {
            mime: "video/vnd.vivo",
            extensions: ["viv"],
        },
    }
}
