use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;
use testcontainers::core::{ContainerState, ExecCommand, WaitFor};
use testcontainers::Image;

pub struct AtlasImage {
    postgres_port: u16,
    volumes: BTreeMap<String, String>,
}

impl Image for AtlasImage {
    type Args = ();

    fn name(&self) -> String {
        "arigaio/atlas".to_owned()
    }

    fn tag(&self) -> String {
        "latest".to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![]
    }

    fn volumes(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.volumes.iter())
    }

    fn exec_after_start(&self, _: ContainerState) -> Vec<ExecCommand> {
        let mut commands = vec![];

        let cmd = format!(
            "migrate apply --url postgres://postgres:postgres@postgres:{}/postgres?sslmode=disable&search_path=public",
            self.postgres_port
        );

        commands.push(ExecCommand {
            cmd,
            ready_conditions: vec![],
        });

        commands
    }
}

impl AtlasImage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_postgres_port(mut self, postgres_port: u16) -> Self {
        self.postgres_port = postgres_port;
        self
    }
}

impl Default for AtlasImage {
    fn default() -> Self {
        let project_root =
            env::var("CARGO_MANIFEST_DIR").expect("Failed to get project root directory");
        let migrations_dir = PathBuf::from(project_root).join("migrations");
        Self {
            postgres_port: 5432,
            volumes: [(
                migrations_dir.to_str().unwrap().to_string(),
                "/migrations".to_string(),
            )]
            .iter()
            .cloned()
            .collect(),
        }
    }
}
