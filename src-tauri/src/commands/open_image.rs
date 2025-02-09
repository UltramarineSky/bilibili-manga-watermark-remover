use std::path::PathBuf;

use anyhow::Context;
use base64::engine::general_purpose;
use base64::Engine;

use crate::errors::CommandResult;
use crate::types::{CommandResponse, JpgImageData, JpgImageInfo};

#[tauri::command(async)]
#[specta::specta]
#[allow(clippy::cast_possible_truncation)]
pub fn open_image(path: String) -> CommandResult<CommandResponse<JpgImageData>> {
    let path = PathBuf::from(path);
    let size = imagesize::size(&path)
        .context(format!("获取图片 {path:?} 的尺寸失败"))
        .map_err(anyhow::Error::from)?;
    let (width, height) = (size.width as u32, size.height as u32);
    let image_data: Vec<u8> = std::fs::read(&path)
        .context(format!("读取图片 {path:?} 失败"))
        .map_err(anyhow::Error::from)?;
    // 将图片数据转换为base64编码
    let base64 = general_purpose::STANDARD.encode(image_data);
    // JpgImage对象
    let data = JpgImageData {
        info: JpgImageInfo {
            width,
            height,
            path,
        },
        base64,
    };
    let res = CommandResponse {
        code: 0,
        msg: String::new(),
        data,
    };
    Ok(res)
}
