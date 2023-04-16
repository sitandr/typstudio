use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TypstCompileEvent {
    pub pages: usize,
    pub hash: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct TypstRenderResponse {
    pub image: String,
    pub width: u32,
    pub height: u32
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectChangeEvent {
    pub project: Option<ProjectModel>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectModel {
    pub root: String,
}
