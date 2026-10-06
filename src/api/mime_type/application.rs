define_mime_type! {
    pub enum Application {
        Json => {
            mime: "application/json",
            extensions: ["json"],
        },
        Xml => {
            mime: "application/xml",
            extensions: ["xml"],
        },
        Pdf => {
            mime: "application/pdf",
            extensions: ["pdf"],
        },
        Zip => {
            mime: "application/zip",
            extensions: ["zip"],
        },
        Gzip => {
            mime: "application/gzip",
            extensions: ["gz", "tgz"],
        },
        OctetStream => {
            mime: "application/octet-stream",
            extensions: [],
        },
        FormUrlEncoded => {
            mime: "application/x-www-form-urlencoded",
            extensions: [],
        },
        Postscript => {
            mime: "application/postscript",
            extensions: ["ps", "eps", "ai"],
        },
        Rtf => {
            mime: "application/rtf",
            extensions: ["rtf"],
        },
        AtomXml => {
            mime: "application/atom+xml",
            extensions: ["atom"],
        },
        RssXml => {
            mime: "application/rss+xml",
            extensions: ["rss"],
        },
        SoapXml => {
            mime: "application/soap+xml",
            extensions: [],
        },
        XhtmlXml => {
            mime: "application/xhtml+xml",
            extensions: ["xhtml"],
        },
        XsltXml => {
            mime: "application/xslt+xml",
            extensions: ["xsl", "xslt"],
        },
        Yaml => {
            mime: "application/yaml",
            extensions: ["yaml", "yml"],
        },
        Wasm => {
            mime: "application/wasm",
            extensions: ["wasm"],
        },
        // Microsoft Office
        MsWord => {
            mime: "application/msword",
            extensions: ["doc", "dot"],
        },
        MsExcel => {
            mime: "application/vnd.ms-excel",
            extensions: ["xls", "xla", "xlb", "xlc", "xlm", "xlt", "xlw"],
        },
        MsPowerpoint => {
            mime: "application/vnd.ms-powerpoint",
            extensions: ["ppt", "pot", "ppa", "pps", "pwz"],
        },
        MsProject => {
            mime: "application/vnd.ms-project",
            extensions: ["mpp", "mpt"],
        },
        MsWorks => {
            mime: "application/vnd.ms-works",
            extensions: ["wcm", "wdb", "wks", "wps"],
        },
        MsVisio => {
            mime: "application/vnd.visio",
            extensions: ["vsd", "vss", "vst", "vsw"],
        },
        MsOneNote => {
            mime: "application/onenote",
            extensions: ["one", "onepkg", "onetmp", "onetoc", "onetoc2"],
        },
        // Office Open XML (newer Office formats)
        VndOpenXmlWordDoc => {
            mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            extensions: ["docx"],
        },
        VndOpenXmlWordTemplate => {
            mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
            extensions: ["dotx"],
        },
        VndOpenXmlSpreadsheet => {
            mime: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            extensions: ["xlsx"],
        },
        VndOpenXmlSpreadsheetTemplate => {
            mime: "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
            extensions: ["xltx"],
        },
        VndOpenXmlPresentation => {
            mime: "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            extensions: ["pptx"],
        },
        VndOpenXmlPresentationTemplate => {
            mime: "application/vnd.openxmlformats-officedocument.presentationml.template",
            extensions: ["potx"],
        },
        // OpenDocument formats
        VndOasisText => {
            mime: "application/vnd.oasis.opendocument.text",
            extensions: ["odt"],
        },
        VndOasisSpreadsheet => {
            mime: "application/vnd.oasis.opendocument.spreadsheet",
            extensions: ["ods"],
        },
        VndOasisPresentation => {
            mime: "application/vnd.oasis.opendocument.presentation",
            extensions: ["odp"],
        },
        VndOasisGraphics => {
            mime: "application/vnd.oasis.opendocument.graphics",
            extensions: ["odg"],
        },
        VndOasisFormula => {
            mime: "application/vnd.oasis.opendocument.formula",
            extensions: ["odf"],
        },
        VndOasisDatabase => {
            mime: "application/vnd.oasis.opendocument.database",
            extensions: ["odb"],
        },
        // Compression and archives
        X7zCompressed => {
            mime: "application/x-7z-compressed",
            extensions: ["7z"],
        },
        XRarCompressed => {
            mime: "application/x-rar-compressed",
            extensions: ["rar"],
        },
        XTar => {
            mime: "application/x-tar",
            extensions: ["tar"],
        },
        XBzip2 => {
            mime: "application/x-bzip2",
            extensions: ["bz2", "boz"],
        },
        XAceCompressed => {
            mime: "application/x-ace-compressed",
            extensions: ["ace"],
        },
        XStuffit => {
            mime: "application/x-stuffit",
            extensions: ["sit", "sitx"],
        },
        VndDebian => {
            mime: "application/vnd.debian.binary-package",
            extensions: ["deb", "udeb"],
        },
        VndRar => {
            mime: "application/vnd.rar",
            extensions: [],
        },
        // Programming and development
        JavaArchive => {
            mime: "application/java-archive",
            extensions: ["jar"],
        },
        JavaSerializedObject => {
            mime: "application/java-serialized-object",
            extensions: ["ser"],
        },
        JavaVm => {
            mime: "application/java-vm",
            extensions: ["class"],
        },
        XShellScript => {
            mime: "application/x-shellscript",
            extensions: ["sh"],
        },
        XPerl => {
            mime: "application/x-perl",
            extensions: ["pl", "pm"],
        },
        XTcl => {
            mime: "application/x-tcl",
            extensions: ["tcl"],
        },
        XPython => {
            mime: "application/x-python",
            extensions: ["py", "pyc", "pyo", "pyd"],
        },
        XRuby => {
            mime: "application/x-ruby",
            extensions: ["rb"],
        },
        // Ebooks and documents
        EpubZip => {
            mime: "application/epub+zip",
            extensions: ["epub"],
        },
        VndAmazonEbook => {
            mime: "application/vnd.amazon.ebook",
            extensions: ["azw"],
        },
        XMobipocketEbook => {
            mime: "application/x-mobipocket-ebook",
            extensions: ["mobi", "prc"],
        },
        VndMsHtmlhelp => {
            mime: "application/vnd.ms-htmlhelp",
            extensions: ["chm"],
        },
        // Database and data
        VndSqlite3 => {
            mime: "application/vnd.sqlite3",
            extensions: ["sqlite", "sqlite3", "db"],
        },
        XNetcdf => {
            mime: "application/x-netcdf",
            extensions: ["nc", "cdf"],
        },
        XHdf => {
            mime: "application/x-hdf",
            extensions: ["hdf"],
        },
        VndMbox => {
            mime: "application/mbox",
            extensions: ["mbox"],
        },
        // Adobe formats
        VndAdobeAir => {
            mime: "application/vnd.adobe.air-application-installer-package+zip",
            extensions: ["air"],
        },
        VndAdobeXdp => {
            mime: "application/vnd.adobe.xdp+xml",
            extensions: ["xdp"],
        },
        VndAdobeXfdf => {
            mime: "application/vnd.adobe.xfdf",
            extensions: ["xfdf"],
        },
        VndAdobePhotoshop => {
            mime: "image/vnd.adobe.photoshop",
            extensions: ["psd"],
        },
        // Google formats
        VndGoogleEarthKml => {
            mime: "application/vnd.google-earth.kml+xml",
            extensions: ["kml"],
        },
        VndGoogleEarthKmz => {
            mime: "application/vnd.google-earth.kmz",
            extensions: ["kmz"],
        },
        // Apple formats
        VndAppleInstaller => {
            mime: "application/vnd.apple.installer+xml",
            extensions: ["mpkg"],
        },
        // Android
        VndAndroidPackage => {
            mime: "application/vnd.android.package-archive",
            extensions: ["apk"],
        },
        // CAD and engineering
        VndAutocadDwg => {
            mime: "application/vnd.dwg",
            extensions: ["dwg"],
        },
        VndAutocadDxf => {
            mime: "application/vnd.dxf",
            extensions: ["dxf"],
        },
    }
}
