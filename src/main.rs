use just_core::result::BoxedResult;

const JUST_PREFIX: &str = "just-";

fn init_log() {
    use simplelog::*;
    use std::fs::File;
    use std::path::Path;

    let log_path = Path::new("..").join("log");

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default())
            .expect("Could not enable terminal logging"),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(log_path).expect("Could not create log"),
        ),
    ])
    .expect("Could not enable logging");
}

fn main() -> BoxedResult<()> {
    use clap::{App, AppSettings};
    use just_core::kernel::Kernel;
    use just_core::system::cmd_run;
    use log::{debug, error};

    init_log();

    let kernel = Kernel::load();

    let matches = App::new("just")
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
        ])
        .get_matches();

    match matches.subcommand() {
        (sub_cmd, Some(sub_args)) => {
            let exe_name = format!("{}{}.exe", JUST_PREFIX, sub_cmd);
            let exe_path = kernel.path.bin_path.join(&exe_name);
            let mut ext_args: Vec<&str> = Vec::new();
            ext_args.extend(sub_args.values_of("").unwrap_or_default());

            debug!("{:?} mit {:?}", exe_path, ext_args);

            if exe_path.exists() {
                cmd_run(exe_path, &ext_args)
            } else {
                error!("{} is not a existing command", sub_cmd);

                Ok(())
            }
        }
        _ => Ok(()),
    }
}
