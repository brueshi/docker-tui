use crate::docker::DockerClient;
use anyhow::Result;
use bollard::models::ContainerSummary;
use ratatui::widgets::ListState;

pub struct App {
    pub should_quit: bool,
    pub containers: Vec<ContainerSummary>,
    pub list_state: ListState,
    pub status_message: Option<String>,
    pub show_delete_confirm: bool,
    docker_client: DockerClient,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        Ok(Self {
            should_quit: false,
            containers: Vec::new(),
            list_state,
            status_message: None,
            show_delete_confirm: false,
            docker_client: DockerClient::new()?,
        })
    }

    pub async fn refresh_containers(&mut self) -> Result<()> {
        self.containers = self.docker_client.list_containers().await?;
        let selected = self.list_state.selected().unwrap_or(0);
        if selected >= self.containers.len() && !self.containers.is_empty() {
            self.list_state.select(Some(self.containers.len() - 1));
        } else if self.containers.is_empty() {
            self.list_state.select(None);
        }
        Ok(())
    }

    pub fn next_container(&mut self) {
        if !self.containers.is_empty() {
            let i = self.list_state.selected().unwrap_or(0);
            let next = (i + 1) % self.containers.len();
            self.list_state.select(Some(next));
        }
    }

    pub fn previous_container(&mut self) {
        if !self.containers.is_empty() {
            let i = self.list_state.selected().unwrap_or(0);
            let prev = if i > 0 {
                i - 1
            } else {
                self.containers.len() - 1
            };
            self.list_state.select(Some(prev));
        }
    }

    pub async fn start_selected(&mut self) -> Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(container) = self.containers.get(i) {
                if let Some(id) = &container.id {
                    self.status_message = Some("Starting container...".to_string());
                    self.docker_client.start_container(id).await?;
                    self.refresh_containers().await?;
                    self.status_message = Some("Container started successfully".to_string());
                }
            }
        }
        Ok(())
    }

    pub async fn stop_selected(&mut self) -> Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(container) = self.containers.get(i) {
                if let Some(id) = &container.id {
                    self.status_message = Some("Stopping container...".to_string());
                    self.docker_client.stop_container(id).await?;
                    self.refresh_containers().await?;
                    self.status_message = Some("Container stopped successfully".to_string());
                }
            }
        }
        Ok(())
    }

    pub async fn restart_selected(&mut self) -> Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(container) = self.containers.get(i) {
                if let Some(id) = &container.id {
                    self.status_message = Some("Restarting container...".to_string());
                    self.docker_client.restart_container(id).await?;
                    self.refresh_containers().await?;
                    self.status_message = Some("Container restarted successfully".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn request_delete_confirm(&mut self) {
        if self.list_state.selected().is_some() {
            self.show_delete_confirm = true;
        }
    }

    pub fn cancel_delete(&mut self) {
        self.show_delete_confirm = false;
    }

    pub async fn confirm_delete(&mut self) -> Result<()> {
        self.show_delete_confirm = false;
        if let Some(i) = self.list_state.selected() {
            if let Some(container) = self.containers.get(i) {
                if let Some(id) = &container.id {
                    self.status_message = Some("Removing container...".to_string());
                    self.docker_client.remove_container(id).await?;
                    self.refresh_containers().await?;
                    self.status_message = Some("Container removed successfully".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}