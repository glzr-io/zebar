use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait Provider {
  async fn start(&self, output_sender: Sender<String>);
  async fn stop(&self);
}
