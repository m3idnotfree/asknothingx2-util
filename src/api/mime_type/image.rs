define_mime_type! {
    pub enum Image {
        Apng => {
            mime: "image/apng",
            extensions: ["apng"],
        },
        Jpeg => {
            mime: "image/jpeg",
            extensions: ["jpg", "jpeg", "jpe", "jfif"],
            aliases: ["image/jpg"]
        },
        Png => {
            mime: "image/png",
            extensions: ["png"],
        },
        Gif => {
            mime: "image/gif",
            extensions: ["gif"],
        },
        Webp => {
            mime: "image/webp",
            extensions: ["webp"],
        },
        SvgXml => {
            mime: "image/svg+xml",
            extensions: ["svg", "svgz"],
            aliases: ["image/svg"]
        },
        Tiff => {
            mime: "image/tiff",
            extensions: ["tif", "tiff"],
            aliases: ["image/tif"]
        },
        Bmp => {
            mime: "image/bmp",
            extensions: ["bmp"],
        },
        Icon => {
            mime: "image/x-icon",
            extensions: ["ico"],
            aliases: ["image/ico", "image/x-ico"]
        },
        Avif => {
            mime: "image/avif",
            extensions: ["avif"],
        },
        Heic => {
            mime: "image/heic",
            extensions: ["heic", "heif"],
        },
        Cgm => {
            mime: "image/cgm",
            extensions: ["cgm"],
        },
        Ief => {
            mime: "image/ief",
            extensions: ["ief"],
        },
        G3fax => {
            mime: "image/g3fax",
            extensions: ["g3"],
        },
        PrsBtif => {
            mime: "image/prs.btif",
            extensions: ["btif"],
        },
        VndDjvu => {
            mime: "image/vnd.djvu",
            extensions: ["djv", "djvu"],
        },
        VndDwg => {
            mime: "image/vnd.dwg",
            extensions: ["dwg"],
        },
        VndDxf => {
            mime: "image/vnd.dxf",
            extensions: ["dxf"],
        },
        VndFastbidsheet => {
            mime: "image/vnd.fastbidsheet",
            extensions: ["fbs"],
        },
        VndFpx => {
            mime: "image/vnd.fpx",
            extensions: ["fpx"],
        },
        VndFst => {
            mime: "image/vnd.fst",
            extensions: ["fst"],
        },
        VndNetFpx => {
            mime: "image/vnd.net-fpx",
            extensions: ["npx"],
        },
        VndWapWbmp => {
            mime: "image/vnd.wap.wbmp",
            extensions: ["wbmp"],
        },
        VndXiff => {
            mime: "image/vnd.xiff",
            extensions: ["xif"],
        },
        VndMsModi => {
            mime: "image/vnd.ms-modi",
            extensions: ["mdi"],
        },
        XAdobeDng => {
            mime: "image/x-adobe-dng",
            extensions: ["dng"],
        },
        XCanonCr2 => {
            mime: "image/x-canon-cr2",
            extensions: ["cr2"],
        },
        XCanonCrw => {
            mime: "image/x-canon-crw",
            extensions: ["crw"],
        },
        XCmuRaster => {
            mime: "image/x-cmu-raster",
            extensions: ["ras"],
        },
        XCmx => {
            mime: "image/x-cmx",
            extensions: ["cmx"],
        },
        XEpsonErf => {
            mime: "image/x-epson-erf",
            extensions: ["erf"],
        },
        XFreehand => {
            mime: "image/x-freehand",
            extensions: ["fh", "fh4", "fh5", "fh7", "fhc"],
        },
        XFujiRaf => {
            mime: "image/x-fuji-raf",
            extensions: ["raf"],
        },
        XIcns => {
            mime: "image/x-icns",
            extensions: ["icns"],
        },
        XKodakDcr => {
            mime: "image/x-kodak-dcr",
            extensions: ["dcr"],
        },
        XKodakK25 => {
            mime: "image/x-kodak-k25",
            extensions: ["k25"],
        },
        XKodakKdc => {
            mime: "image/x-kodak-kdc",
            extensions: ["kdc"],
        },
        XMinoltaMrw => {
            mime: "image/x-minolta-mrw",
            extensions: ["mrw"],
        },
        XNikonNef => {
            mime: "image/x-nikon-nef",
            extensions: ["nef"],
        },
        XOlympusOrf => {
            mime: "image/x-olympus-orf",
            extensions: ["orf"],
        },
        XPanasonicRaw => {
            mime: "image/x-panasonic-raw",
            extensions: ["raw", "rw2", "rwl"],
        },
        XPcx => {
            mime: "image/x-pcx",
            extensions: ["pcx"],
        },
        XPentaxPef => {
            mime: "image/x-pentax-pef",
            extensions: ["pef", "ptx"],
        },
        XPict => {
            mime: "image/x-pict",
            extensions: ["pct", "pic"],
        },
        XPortableAnymap => {
            mime: "image/x-portable-anymap",
            extensions: ["pnm"],
        },
        XPortableBitmap => {
            mime: "image/x-portable-bitmap",
            extensions: ["pbm"],
        },
        XPortableGraymap => {
            mime: "image/x-portable-graymap",
            extensions: ["pgm"],
        },
        XPortablePixmap => {
            mime: "image/x-portable-pixmap",
            extensions: ["ppm"],
        },
        XRgb => {
            mime: "image/x-rgb",
            extensions: ["rgb"],
        },
        XSigmaX3f => {
            mime: "image/x-sigma-x3f",
            extensions: ["x3f"],
        },
        XSonyArw => {
            mime: "image/x-sony-arw",
            extensions: ["arw"],
        },
        XSonySr2 => {
            mime: "image/x-sony-sr2",
            extensions: ["sr2"],
        },
        XSonySrf => {
            mime: "image/x-sony-srf",
            extensions: ["srf"],
        },
        XXbitmap => {
            mime: "image/x-xbitmap",
            extensions: ["xbm"],
        },
        XXpixmap => {
            mime: "image/x-xpixmap",
            extensions: ["xpm"],
        },
        XXwindowdump => {
            mime: "image/x-xwindowdump",
            extensions: ["xwd"],
        },
    }
}
