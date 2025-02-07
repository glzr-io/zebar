use komorebi_util::KomorebiClient;

fn main() -> komorebi_util::Result<()> {
  let mut client = KomorebiClient::new("zebar.sock")?;

  while let Ok(output) = client.output_blocking() {
    println!("Output: {:?}", output);
  }

  Ok(())
}
