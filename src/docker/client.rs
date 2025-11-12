use bollard::Docker;
use bollard::container::ListContainersOptions;
use bollard::models::ContainerSummary;
use anyhow::Result;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }

    pub async fn list_containers(&self) -> Result<Vec<ContainerSummary>> {
        let options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });
        
        let containers = self.docker.list_containers(options).await?;
        Ok(containers)
    }

    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.docker.start_container::<String>(id, None).await?;
        Ok(())
    }

    pub async fn stop_container(&self, id: &str) -> Result<()> {
        self.docker.stop_container(id, None).await?;
        Ok(())
    }

    pub async fn remove_container(&self, id: &str) -> Result<()> {
        self.docker.remove_container(id, None).await?;
        Ok(())
    }

    pub async fn restart_container(&self, id: &str) -> Result<()> {
        self.docker.restart_container(id, None).await?;
        Ok(())
    }
}