use atuin_config::store::AliasStore;
use eyre::Result;

pub async fn init(store: AliasStore, disable_up_arrow: bool, disable_ctrl_r: bool) -> Result<()> {
    let base = include_str!("../../../shell/atuin.bash");

    let aliases = store.aliases().await?;

    let aliases = atuin_config::shell::bash::build(&aliases[..]);

    let (bind_ctrl_r, bind_up_arrow) = if std::env::var("ATUIN_NOBIND").is_ok() {
        (false, false)
    } else {
        (!disable_ctrl_r, !disable_up_arrow)
    };

    println!("__atuin_bind_ctrl_r={bind_ctrl_r}");
    println!("__atuin_bind_up_arrow={bind_up_arrow}");
    println!("{base}");
    println!("{aliases}");

    Ok(())
}
