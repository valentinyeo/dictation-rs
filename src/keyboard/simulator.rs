use std::mem::size_of;
use std::time::Duration;
use tokio::sync::mpsc;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub struct KeyboardSimulator;

impl KeyboardSimulator {
    pub async fn start(mut text_rx: mpsc::UnboundedReceiver<String>) {
        while let Some(text) = text_rx.recv().await {
            Self::type_text(&text).await;
        }
    }

    async fn type_text(text: &str) {
        // Add space before text if not empty
        let text_with_space = if text.is_empty() {
            text.to_string()
        } else {
            format!(" {}", text)
        };

        for ch in text_with_space.chars() {
            Self::send_unicode_char(ch);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    fn send_unicode_char(ch: char) {
        unsafe {
            let mut input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch as u16,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            SendInput(&[input], size_of::<INPUT>() as i32);
        }
    }
}
