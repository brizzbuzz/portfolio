use clap::Parser;
use pipelines::{build_pipeline, deploy_pipeline, test_pipeline, Pipeline};

mod pipelines;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pipeline to run
    #[arg(value_enum, short = 'p')]
    pipeline: Pipeline,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();
    let client = dagger_sdk::connect().await?;

    match args.pipeline {
        Pipeline::Build => build_pipeline::push_image(&client).await?,
        Pipeline::Test => test_pipeline::run_tests(&client).await?,
        Pipeline::Deploy => deploy_pipeline::deploy_application().await?,
    }

    Ok(())
}
