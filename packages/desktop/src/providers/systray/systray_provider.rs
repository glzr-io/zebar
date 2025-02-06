use serde::{Deserialize, Serialize};
use systray_util::{ImageFormat, Systray, SystrayIcon, SystrayIconAction};

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
  pub icons: Vec<SystrayOutputIcon>,
}

#[derive(Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystrayOutputIcon {
  pub id: String,
  pub tooltip: String,
  pub icon_bytes: Vec<u8>,
}

impl TryFrom<SystrayIcon> for SystrayOutputIcon {
  type Error = anyhow::Error;

  fn try_from(icon: SystrayIcon) -> Result<Self, Self::Error> {
    Ok(SystrayOutputIcon {
      id: icon.stable_id.to_string(),
      tooltip: icon.tooltip.clone(),
      icon_bytes: icon.to_image_format(ImageFormat::Png)?,
    })
  }
}

impl std::fmt::Debug for SystrayOutputIcon {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "SystrayOutputIcon {{ id: {}, tooltip: {} }}",
      self.id, self.tooltip
    )
  }
}

pub struct SystrayProvider {
  _config: SystrayProviderConfig,
  common: CommonProviderState,
}

impl SystrayProvider {
  pub fn new(
    _config: SystrayProviderConfig,
    common: CommonProviderState,
  ) -> SystrayProvider {
    SystrayProvider { _config, common }
  }

  fn handle_function(
    systray: &mut Systray,
    function: SystrayFunction,
  ) -> anyhow::Result<ProviderFunctionResponse> {
    match &function {
      SystrayFunction::IconHoverEnter(args) => systray
        .send_action(args.icon_id.parse()?, SystrayIconAction::HoverEnter),
      SystrayFunction::IconHoverLeave(args) => systray
        .send_action(args.icon_id.parse()?, SystrayIconAction::HoverLeave),
      SystrayFunction::IconHoverMove(args) => systray
        .send_action(args.icon_id.parse()?, SystrayIconAction::HoverMove),
      SystrayFunction::IconLeftClick(args) => systray
        .send_action(args.icon_id.parse()?, SystrayIconAction::LeftClick),
      SystrayFunction::IconRightClick(args) => systray
        .send_action(args.icon_id.parse()?, SystrayIconAction::RightClick),
      SystrayFunction::IconMiddleClick(args) => systray.send_action(
        args.icon_id.parse()?,
        SystrayIconAction::MiddleClick,
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
              .filter_map(|icon| SystrayOutputIcon::try_from(icon).ok())
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
