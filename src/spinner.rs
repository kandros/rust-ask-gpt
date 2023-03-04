use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    pub fn new() -> Self {
        let pb = ProgressBar::new_spinner();
        return Spinner { pb };
    }
    pub fn start(&self) {
        self.pb.enable_steady_tick(Duration::from_millis(120));
        self.pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        self.pb.set_message("Calculating...")
    }
    pub fn stop(&self) {
        self.pb.finish_and_clear();
    }
}
