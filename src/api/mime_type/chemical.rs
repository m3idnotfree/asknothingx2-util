define_mime_type! {
    pub enum Chemical {
        XCdx => {
            mime: "chemical/x-cdx",
            extensions: ["cdx"],
        },
        XCif => {
            mime: "chemical/x-cif",
            extensions: ["cif"],
        },
        XCml => {
            mime: "chemical/x-cml",
            extensions: ["cml"],
        },
        XCsml => {
            mime: "chemical/x-csml",
            extensions: ["csml"],
        },
        XXyz => {
            mime: "chemical/x-xyz",
            extensions: ["xyz"],
        },
    }
}
