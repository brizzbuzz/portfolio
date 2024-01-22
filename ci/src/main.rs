use clap::Parser;
use pipelines::{build_pipeline, test_pipeline, deploy_pipeline};

mod pipelines;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Pipeline {
    Build,
    Test,
    Deploy,
}

impl std::fmt::Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipeline::Build => write!(f, "build"),
            Pipeline::Test => write!(f, "test"),
            Pipeline::Deploy => write!(f, "deploy"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pipeline to run
    #[arg(value_enum, short='p')]
    pipeline: Pipeline,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();
    let client = dagger_sdk::connect().await?;

    match args.pipeline {
        Pipeline::Build => build_pipeline::build_image(&client).await?,
        Pipeline::Test => test_pipeline::run_tests().await?,
        Pipeline::Deploy => deploy_pipeline::deploy_application().await?,
    }

    Ok(())
}
