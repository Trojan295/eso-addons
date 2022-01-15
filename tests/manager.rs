extern crate tempfile;

use std::error::Error;

use eso_addons::addons::Manager;

#[test]
fn addon_manager_install_remove_addon() -> Result<(), Box<dyn Error>> {
    let addon_dir = tempfile::tempdir()?;
    let manager = Manager::new(addon_dir.path());

    let addon = manager
        .download_addon("https://www.esoui.com/downloads/download2275-LibDebugLogger.html")?;

    let addon_list = manager.get_addons()?;
    assert!(
        addon_list.addons.len() == 1,
        "Installed mods: {:?}",
        addon_list.addons
    );

    manager.delete_addon(&addon)?;

    let addon_list = manager.get_addons()?;
    assert!(
        addon_list.addons.len() == 0,
        "Installed mods: {:?}",
        addon_list.addons
    );

    Ok(())
}

#[test]
fn addon_manager_supports_nested_modules() -> Result<(), Box<dyn Error>> {
    let addon_dir = tempfile::tempdir()?;
    let manager = Manager::new(addon_dir.path());

    manager.download_addon("https://www.esoui.com/downloads/download1360-CombatMetrics")?;

    let addon_list = manager.get_addons()?;
    assert!(
        addon_list.errors.len() == 0,
        "failed to list addons: {:?}",
        addon_list.errors
    );
    assert!(
        addon_list.addons.len() == 2,
        "Installed mods: {:?}",
        addon_list.addons
    );

    Ok(())
}
