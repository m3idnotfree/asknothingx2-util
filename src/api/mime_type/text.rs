define_mime_type! {
    pub enum Text {
        Plain => {
            mime: "text/plain",
            extensions: ["txt"],
        },
        Html => {
            mime: "text/html",
            extensions: ["html", "htm"],
        },
        Css => {
            mime: "text/css",
            extensions: ["css"],
        },
        Javascript => {
            mime: "text/javascript",
            extensions: ["js", "mjs"],
        },
        Csv => {
            mime: "text/csv",
            extensions: ["csv"],
        },
        Xml => {
            mime: "text/xml",
            extensions: ["xml"],
        },
        Markdown => {
            mime: "text/markdown",
            extensions: ["md", "markdown"],
            aliases: ["text/x-markdown"]
        },
        Calendar => {
            mime: "text/calendar",
            extensions: ["ics"],
        },
        Richtext => {
            mime: "text/richtext",
            extensions: ["rtx"],
            aliases: ["text/rtf"]
        },
        Sgml => {
            mime: "text/sgml",
            extensions: ["sgml", "sgm"],
        },
        TabSeparatedValues => {
            mime: "text/tab-separated-values",
            extensions: ["tsv"],
        },
        Troff => {
            mime: "text/troff",
            extensions: ["tr", "roff", "man", "me", "ms"],
        },
        UriList => {
            mime: "text/uri-list",
            extensions: ["uri", "uris", "urls"],
        },
        VCard => {
            mime: "text/x-vcard",
            extensions: ["vcf", "vcard"],
        },
        VCalendar => {
            mime: "text/x-vcalendar",
            extensions: ["vcs"],
        },
        Setext => {
            mime: "text/x-setext",
            extensions: ["etx"],
        },
        Uuencode => {
            mime: "text/x-uuencode",
            extensions: ["uu"],
        },
        Asm => {
            mime: "text/x-asm",
            extensions: ["s", "asm"],
        },
        C => {
            mime: "text/x-c",
            extensions: ["c", "cc", "cxx", "cpp", "h", "hh", "dic"],
        },
        Fortran => {
            mime: "text/x-fortran",
            extensions: ["f", "for", "f77", "f90"],
        },
        JavaSource => {
            mime: "text/x-java-source",
            extensions: ["java"],
        },
        Pascal => {
            mime: "text/x-pascal",
            extensions: ["p", "pas"],
        },
        Python => {
            mime: "text/x-python",
            extensions: ["py"],
        },
    }
}
