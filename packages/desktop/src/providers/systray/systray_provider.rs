use serde::{Deserialize, Serialize};
use systray_util::{IconData, IconEvent, Systray};

use crate::providers::{
  CommonProviderState, Provider, ProviderFunction,
  ProviderFunctionResponse, ProviderInputMsg, RuntimeType,
  SystrayFunction,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystrayProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystrayOutput {
  pub icons: Vec<SystrayIcon>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystrayIcon {
  pub id: String,
  pub tooltip: String,
  pub icon_bytes: Vec<u8>,
}

impl TryFrom<IconData> for SystrayIcon {
  type Error = anyhow::Error;

  fn try_from(icon: IconData) -> Result<Self, Self::Error> {
    Ok(SystrayIcon {
      id: icon.uid.to_string(),
      tooltip: icon.tooltip.clone(),
      icon_bytes: icon.to_png()?,
    })
  }
}

pub struct SystrayProvider {
  config: SystrayProviderConfig,
  common: CommonProviderState,
}

impl SystrayProvider {
  pub fn new(
    config: SystrayProviderConfig,
    common: CommonProviderState,
  ) -> SystrayProvider {
    SystrayProvider { config, common }
  }

  fn handle_function(
    systray: &mut Systray,
    function: SystrayFunction,
  ) -> anyhow::Result<ProviderFunctionResponse> {
    match &function {
      SystrayFunction::IconHoverEnter(args) => systray.send_icon_event(
        args.icon_id.parse::<u32>()?,
        IconEvent::HoverEnter,
      ),
      SystrayFunction::IconHoverLeave(args) => systray.send_icon_event(
        args.icon_id.parse::<u32>()?,
        IconEvent::HoverLeave,
      ),
      SystrayFunction::IconHoverMove(args) => systray.send_icon_event(
        args.icon_id.parse::<u32>()?,
        IconEvent::HoverMove,
      ),
      SystrayFunction::IconLeftClick(args) => systray.send_icon_event(
        args.icon_id.parse::<u32>()?,
        IconEvent::LeftClick,
      ),
      SystrayFunction::IconRightClick(args) => systray.send_icon_event(
        args.icon_id.parse::<u32>()?,
        IconEvent::RightClick,
      ),
      SystrayFunction::IconMiddleClick(args) => systray.send_icon_event(
        args.icon_id.parse::<u32>()?,
        IconEvent::MiddleClick,
      ),
    }?;

    Ok(ProviderFunctionResponse::Null)
  }
}

#[async_trait]
impl Provider for SystrayProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Async
  }

  async fn start_async(&mut self) {
    let Ok(mut systray) = Systray::new() else {
      self.common.emitter.emit_output::<SystrayOutput>(Err(
        anyhow::anyhow!("Failed to initialize systray."),
      ));

      return;
    };

    loop {
      tokio::select! {
        _ = systray.events() => {
          self.common.emitter.emit_output(Ok(SystrayOutput {
            icons: systray
              .icons()
              .into_iter()
              .filter_map(|icon| SystrayIcon::try_from(icon).ok())
              .collect(),
          }));
        }
        Some(input) = self.common.input.async_rx.recv() => {
          match input {
            ProviderInputMsg::Stop => {
              break;
            }
            ProviderInputMsg::Function(
              ProviderFunction::Systray(systray_function),
              sender,
            ) => {
              let res = Self::handle_function(&mut systray, systray_function).map_err(|err| err.to_string());
              sender.send(res).unwrap();
            }
            _ => {}
          }
        }
      }
    }
  }
}
