use std::sync::mpsc;
use std::io::Write;
use tracing_subscriber::layer::SubscriberExt;

struct GuiTracingWriter {
    gui_sender: mpsc::SyncSender<String>,
}

impl Write for GuiTracingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.gui_sender
            .try_send(String::from_utf8_lossy(buf).trim_start().into())
            .ok();
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct GuiTracingWriterFactory {
    sender: mpsc::SyncSender<String>,
}

impl tracing_subscriber::fmt::MakeWriter for GuiTracingWriterFactory {
    type Writer = GuiTracingWriter;
    fn make_writer(&self) -> Self::Writer {
        Self::Writer {
            gui_sender: self.sender.clone(),
        }
    }
}

pub fn get_log_level() -> tracing::Level {
    #[cfg(debug_assertions)]
    let mut level = tracing::Level::DEBUG;

    #[cfg(not(debug_assertions))]
    let mut level = tracing::Level::INFO;

    if let Ok(var) = std::env::var("WEYLUS_LOG_LEVEL") {
        let l: Result<tracing::Level, _> = var.parse();
        if let Ok(l) = l {
            level = l;
        }
    }
    level
}

pub fn setup_logging(sender: mpsc::SyncSender<String>) {
    let logger = tracing_subscriber::fmt()
        .with_max_level(get_log_level())
        .finish()
        .with(
            tracing_subscriber::fmt::Layer::default()
                .with_ansi(false)
                .without_time()
                .with_target(false)
                .compact()
                .with_writer(GuiTracingWriterFactory { sender }),
        );
    tracing::subscriber::set_global_default(logger).expect("Failed to setup logger!");
}
