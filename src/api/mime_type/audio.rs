define_mime_type! {
    pub enum Audio {
        Mpeg => {
            mime: "audio/mpeg",
            extensions: ["mp3", "mpga"],
            aliases: ["audio/mp3"]
        },
        Mp4 => {
            mime: "audio/mp4",
            extensions: ["m4a", "mp4a"]
        },
        Ogg => {
            mime: "audio/ogg",
            extensions: ["oga", "ogg", "spx"]
        },
        Webm => {
            mime: "audio/webm",
            extensions: ["webm"]
        },
        Wav => {
            mime: "audio/wav",
            extensions: ["wav", "wave"],
            aliases: ["audio/wave", "audio/x-wav"]
        },
        Flac => {
            mime: "audio/flac",
            extensions: ["flac"]
        },
        Aac => {
            mime: "audio/aac",
            extensions: ["aac"]
        },
        Aiff => {
            mime: "audio/aiff",
            extensions: ["aif", "aiff", "aifc"]
        },
        Basic => {
            mime: "audio/basic",
            extensions: ["au", "snd"]
        },
        Midi => {
            mime: "audio/midi",
            extensions: ["mid", "midi", "kar", "rmi"]
        },
        Opus => {
            mime: "audio/opus",
            extensions: ["opus"]
        },
        VndDigitalWinds => {
            mime: "audio/vnd.digital-winds",
            extensions: []
        },
        VndDts => {
            mime: "audio/vnd.dts",
            extensions: []
        },
        VndDtsHd => {
            mime: "audio/vnd.dts.hd",
            extensions: []
        },
        VndLucentVoice => {
            mime: "audio/vnd.lucent.voice",
            extensions: []
        },
        VndMsPlayready => {
            mime: "audio/vnd.ms-playready.media.pya",
            extensions: []
        },
        VndNueraEcelp4800 => {
            mime: "audio/vnd.nuera.ecelp4800",
            extensions: []
        },
        VndNueraEcelp7470 => {
            mime: "audio/vnd.nuera.ecelp7470",
            extensions: []
        },
        VndNueraEcelp9600 => {
            mime: "audio/vnd.nuera.ecelp9600",
            extensions: []
        },
        XMatroska => {
            mime: "audio/x-matroska",
            extensions: ["mka"]
        },
        XMpegurl => {
            mime: "audio/x-mpegurl",
            extensions: ["m3u"]
        },
        XMsWax => {
            mime: "audio/x-ms-wax",
            extensions: ["wax"]
        },
        XMsWma => {
            mime: "audio/x-ms-wma",
            extensions: ["wma"]
        },
        XPnRealaudio => {
            mime: "audio/x-pn-realaudio",
            extensions: ["ra", "ram"]
        },
        XPnRealaudioPlugin => {
            mime: "audio/x-pn-realaudio-plugin",
            extensions: ["rmp"]
        }
    }
}
