#![feature(vec_push_within_capacity)]
use log::LevelFilter;
use rand::random;
use sp_log2::{ColorChoice, CombinedLogger, ConfigBuilder, Format, TermLogger, TerminalMode};

extern crate sp_log2;
#[macro_use]
extern crate tacky_borders_logger;

fn initialize_logger() -> anyhow::Result<()> {
    let mut config_builder = ConfigBuilder::new();

    config_builder.set_format(
        Format::LevelFlag | Format::Time | Format::Thread | Format::Target | Format::FileLocation,
    );

    config_builder.set_formatter(Some(
        "[time:#89dceb] [level:bold] ([thread]) [target:rgb(137 180 250):bold]: [message:bold] [[file:#6c7086]]\n",
    ));

    config_builder.set_time_format_custom("%d/%m/%Y %H:%M:%S,%3f");

    let config = config_builder.build();

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    Ok(())
}

fn generate_custom_vec(data: &mut Vec<u8>) {
    let size = data.capacity();
    data.clear();
    for _ in 0..size {
        let num = random::<u8>();
        let _ = data.push_within_capacity(num);
    }
}

fn main() {
    if let Err(e) = &initialize_logger() {
        error!("logger initialization failed: {e}");
    };

    let size = 4_usize;
    let mut vec = Vec::with_capacity(size);

    generate_custom_vec(&mut vec);
    trace!("{vec:#?}");
    generate_custom_vec(&mut vec);
    debug!("{vec:#?}");
    generate_custom_vec(&mut vec);
    info!("{vec:#?}");
    generate_custom_vec(&mut vec);
    warn!("{vec:#?}");
    generate_custom_vec(&mut vec);
    error!("{vec:#?}");
}
