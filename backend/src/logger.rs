use anyhow::Context;

pub fn init_logger() -> anyhow::Result<()> {
    #[cfg(feature = "file_logging")]
    let file_cfg = {
        use anyhow::anyhow;
        use std::io::ErrorKind;
        use std::path::Path;

        // create logs directory
        let log_path = Path::new("logs");
        if let Err(err) = std::fs::create_dir(&log_path) {
            if !matches!(err.kind(), ErrorKind::AlreadyExists) {
                return Err(anyhow!("failed to create logs directory"));
            }
        }

        // create a log file where its name is when this program is started running
        let log_file_path = chrono::Local::now().format("%Y-%m-%d:%H:%M:%S.log");
        let log_file_path = log_path.join(log_file_path.to_string());

        // not as pretty as stdio logger but good enough to read the *.log file
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}][{}] {}",
                    chrono::Local::now().format("%Y-%m-%dT%H:%M:%SZ"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .chain(fern::log_file(log_file_path).with_context(|| "failed to enter log file")?)
    };

    // custom env_logger style in fern
    let stdio_cfg = fern::Dispatch::new()
        .format(|out, message, record| {
            use nu_ansi_term::{Color::*, Style};

            let open_bracket = DarkGray.paint("[");
            let close_bracket = DarkGray.paint("]");
            let boldy = Style::new().bold();
            let target = record.target();

            out.finish(format_args!(
                "{0}{5}{1}{0}{2}{1}{0}{3}{1} {4}",
                open_bracket,
                close_bracket,
                match record.level() {
                    log::Level::Error => Red.bold().paint("ERROR"),
                    log::Level::Warn => Yellow.bold().paint("WARN "),
                    log::Level::Info => Green.bold().paint("INFO "),
                    log::Level::Debug => Cyan.bold().paint("DEBUG"),
                    log::Level::Trace => Blue.bold().paint("TRACE"),
                },
                boldy.paint(target),
                message,
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%SZ"),
            ))
        })
        .chain(std::io::stdout());

    #[cfg(debug_assertions)]
    let level = log::LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let level = log::LevelFilter::Info;

    // dispatching all loggers
    let dispatch = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("backend_lib", level)
        .level_for("backend_bin", level)
        .chain(stdio_cfg);

    #[cfg(feature = "file_logging")]
    let dispatch = dispatch.chain(file_cfg);
    dispatch.apply().with_context(|| "failed to load logger")?;
    Ok(())
}
