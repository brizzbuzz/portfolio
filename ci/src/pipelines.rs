pub mod build_pipeline;
pub mod deploy_pipeline;
pub mod test_pipeline;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Pipeline {
    Build,
    Test,
    Deploy,
}

impl std::fmt::Display for crate::Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::Pipeline::Build => write!(f, "build"),
            crate::Pipeline::Test => write!(f, "test"),
            crate::Pipeline::Deploy => write!(f, "deploy"),
        }
    }
}
