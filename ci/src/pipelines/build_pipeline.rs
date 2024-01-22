use dagger_sdk::DaggerConn;

pub async fn build_image(client: &DaggerConn) -> eyre::Result<()> {
    let context_dir = client.host().directory(".");
    let image = context_dir
        .docker_build()
        .publish(format!("ttl.sh/hello-dagger-{}", rand::random::<u64>()))
        .await?;

    println!("Built image {}", image.trim());

    Ok(())
}
