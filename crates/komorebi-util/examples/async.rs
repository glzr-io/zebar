use komorebi_util::KomorebiClient;

#[tokio::main]
async fn main() -> komorebi_util::Result<()> {
  let mut client = KomorebiClient::new("zebar.sock")?;

  loop {
    match client.output().await {
      Ok(output) => println!("Output: {:?}", output),
      Err(e) => {
        println!("Error: {:?}", e);
        break;
      }
    }
  }

  Ok(())
}
