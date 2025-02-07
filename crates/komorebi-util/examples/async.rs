use komorebi_util::KomorebiClient;

#[tokio::main]
async fn main() -> komorebi_util::Result<()> {
  let mut client = KomorebiClient::new("zebar.sock")?;

  while let Ok(output) = client.output().await {
    println!("Output: {:?}", output);
  }

  Ok(())
}
