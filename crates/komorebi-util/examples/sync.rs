use komorebi_util::{Client, KomorebiOutput};

fn main() -> komorebi_util::Result<()> {
  let client = Client::new()?;

  while let Some(output) = client.output_blocking() {
    println!("Output: {:?}", output);
  }

  Ok(())
}
