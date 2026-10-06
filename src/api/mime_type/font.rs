define_mime_type! {
    pub enum Font {
        Woff => {
            mime: "font/woff",
            extensions: ["woff"],
            aliases: ["application/font-woff"]
        },
        Woff2 => {
            mime: "font/woff2",
            extensions: ["woff2"],
            aliases: ["application/font-woff2"]
        },
        Otf => {
            mime: "font/otf",
            extensions: ["otf"],
        },
        Ttf => {
            mime: "font/ttf",
            extensions: ["ttf"],
        },
        ApplicationXFontBdf => {
            mime: "application/x-font-bdf",
            extensions: ["bdf"],
        },
        ApplicationXFontGhostscript => {
            mime: "application/x-font-ghostscript",
            extensions: ["gsf"],
        },
        ApplicationXFontLinuxPsf => {
            mime: "application/x-font-linux-psf",
            extensions: ["psf"],
        },
        ApplicationXFontOtf => {
            mime: "application/x-font-otf",
            extensions: [],
        },
        ApplicationXFontPcf => {
            mime: "application/x-font-pcf",
            extensions: ["pcf"],
        },
        ApplicationXFontSnf => {
            mime: "application/x-font-snf",
            extensions: ["snf"],
        },
        ApplicationXFontTtf => {
            mime: "application/x-font-ttf",
            extensions: [],
            aliases: ["application/x-font-truetype"]
        },
        ApplicationXFontType1 => {
            mime: "application/x-font-type1",
            extensions: ["pfa", "pfb"],
        },
        ApplicationVndMsFontobject => {
            mime: "application/vnd.ms-fontobject",
            extensions: ["eot"],
        },
    }
}
