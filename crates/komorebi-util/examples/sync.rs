use komorebi_util::KomorebiClient;

fn main() -> komorebi_util::Result<()> {
  let mut client = KomorebiClient::new("zebar.sock")?;

  loop {
    match client.output_blocking() {
      Ok(output) => println!("Output: {:?}", output),
      Err(e) => {
        println!("Error: {:?}", e);
        break;
      }
    }
  }

  Ok(())
}
