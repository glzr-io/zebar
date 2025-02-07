use komorebi_util::{Client, KomorebiOutput};

#[tokio::main]
async fn main() -> komorebi_util::Result<()> {
  let client = Client::new()?;

  while let Some(output) = client.output().await {
    println!("Output: {:?}", output);
  }

  Ok(())
}
