use systray_util::Systray;

fn main() -> systray_util::Result<()> {
  tracing_subscriber::fmt().init();

  let mut systray = Systray::new()?;

  let mut index = 0;
  while let Some(event) = systray.changes() {
    println!("{} Event: {:?}", index, event);
    index += 1;

    if index == 25 {
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("================================================");
      println!("Sending left click");
      let uid = systray.icons.values().next().unwrap();
      println!("Icon: {:?}", uid);
      systray.send_left_click(uid.uid)?;
    }
  }

  Ok(())
}
