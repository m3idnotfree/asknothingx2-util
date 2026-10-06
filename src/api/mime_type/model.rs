define_mime_type! {
    pub enum Model {
        Iges => {
            mime: "model/iges",
            extensions: ["igs", "iges"],
        },
        Mesh => {
            mime: "model/mesh",
            extensions: ["msh", "mesh"],
        },
        Vrml => {
            mime: "model/vrml",
            extensions: ["wrl", "vrml"],
        },
        VndDwf => {
            mime: "model/vnd.dwf",
            extensions: ["dwf"],
        },
        VndGdl => {
            mime: "model/vnd.gdl",
            extensions: ["gdl"],
        },
        VndGtw => {
            mime: "model/vnd.gtw",
            extensions: ["gtw"],
        },
        VndMts => {
            mime: "model/vnd.mts",
            extensions: ["mts"],
        },
        VndVtu => {
            mime: "model/vnd.vtu",
            extensions: ["vtu"],
        },
    }
}
