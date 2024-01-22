use dagger_sdk::{DaggerConn, HostDirectoryOpts};

pub async fn run_tests(client: &DaggerConn) -> eyre::Result<()> {
    let result = client
        .container()
        .from("rust:1.74") // TODO: use const for version... and maybe slim?
        .with_directory(
            ".",
            client.host().directory_opts(
                ".",
                HostDirectoryOpts {
                    exclude: Some(vec!["target/"]),
                    include: None,
                },
            ),
        )
        .with_exec(vec!["cargo", "test"])
        .stdout()
        .await?;

    println!("{}", result);

    Ok(())
}
