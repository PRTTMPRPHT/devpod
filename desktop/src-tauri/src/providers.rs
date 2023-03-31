use crate::{
    commands::{
        list_providers::ListProvidersCommand, use_provider::UseProviderCommand,
        DevpodCommandConfig, DevpodCommandError, delete_provider::DeleteProviderCommand,
    },
    system_tray::{SystemTray, SystemTrayClickHandler, ToSystemTraySubmenu},
};
use chrono::DateTime;
use log::{trace, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{
    AppHandle, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Wry, 
};

#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProvidersState {
    default_provider: Option<String>,
    providers: Providers,
}
impl ProvidersState {
    pub const IDENTIFIER_PREFIX: &str = "providers";

    fn item_id(id: &String) -> String {
        format!("{}-{}", Self::IDENTIFIER_PREFIX, id)
    }
}
impl ProvidersState {
    pub fn load() -> Result<ProvidersState, DevpodCommandError> {
        trace!("loading providers");
        let list_providers_cmd = ListProvidersCommand::new();

        list_providers_cmd.exec()
    }
}

impl ProvidersState {
    const ADD_PROVIDER_ID: &str = "add_provider";
}

impl ToSystemTraySubmenu for ProvidersState {
    fn to_submenu(&self) -> tauri::SystemTraySubmenu {
        let mut providers_menu = SystemTrayMenu::new();
        let mut providers: Vec<_> = self.providers.iter().collect();
        providers.sort_by_key(|(key, _)| *key);

        providers_menu = providers_menu
            .add_item(CustomMenuItem::new(Self::ADD_PROVIDER_ID, "Add Provider"))
            .add_native_item(SystemTrayMenuItem::Separator);
        for (provider_name, _value) in providers {
            let mut item = CustomMenuItem::new(Self::item_id(provider_name), provider_name);
            if Some(provider_name.to_string()) == self.default_provider {
                item = item.selected();
            }

            providers_menu = providers_menu.add_item(item);
        }
        SystemTraySubmenu::new("Providers", providers_menu)
    }

    fn on_tray_item_clicked(&self, tray_item_id: &str) -> Option<SystemTrayClickHandler> {
        // Make sure providers contain clicked item.
        let provider = self
            .providers
            .iter()
            .find(|(el_id, _)| Self::item_id(el_id) == tray_item_id);

        // Don't proceed if default provider is the same as the selected.
        if self.default_provider == provider.map(|(name, _)| name.clone()) {
            return None;
        }

        if let Some(provider) = provider {
            let provider_name = provider.0;
            let use_provider_cmd = UseProviderCommand::new(provider_name);
            let updated = use_provider_cmd.exec();

            if updated.is_err() {
                return None;
            }

            let provider_name = provider_name.to_string();
            return Some(Box::new(move |app_handle, app_state| {
                let tray_handle = app_handle.tray_handle();
                let providers = &mut *app_state.providers.lock().unwrap();
                providers.default_provider = Some(provider_name.to_string());

                let workspaces = &*app_state.workspaces.lock().unwrap();
                let new_menu = SystemTray::new()
                    .build_with_submenus(vec![Box::new(workspaces), Box::new(providers)]);

                tray_handle
                    .set_menu(new_menu)
                    .expect("should be able to set menu");
            }));
        }

        None
    }
}

type Providers = HashMap<String, Provider>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct Provider {
    options: Option<HashMap<String, ProviderOption>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct ProviderOption {
    value: Option<String>,
    local: Option<bool>,
    retrieved: Option<DateTime<chrono::Utc>>,
}

pub fn check_dangling_provider(app: &AppHandle<Wry>) {
    use tauri_plugin_store::{with_store, StoreCollection};
    use tauri::Manager;

    let stores = app.state::<StoreCollection<Wry>>();
    let file_name = ".providers.dat"; // WARN: needs to match the file name defined in typescript
    let dangling_provider_key = "danglingProvider"; // WARN: needs to match the key defined in typescript
    let path = app.path_resolver().app_data_dir();
    if path.is_none() {
        return;
    }

    let mut path = path.expect("AppDataDir should exist");
    path.push(file_name);

    let _ = with_store(app.app_handle(), stores, path, |store| {
        store
            .get(dangling_provider_key)
            .and_then(|dangling_provider| {
                serde_json::from_value::<String>(dangling_provider.clone()).ok()
            })
            .and_then(|dangling_provider| {
                info!(
                    "Found dangling provider: {}, attempting to delete",
                    dangling_provider
                );
                if DeleteProviderCommand::new(dangling_provider.clone())
                    .exec()
                    .is_ok()
                {
                    if let Ok(_) = store.delete(dangling_provider_key) {
                        info!(
                            "Successfully deleted dangling provider: {}",
                            dangling_provider
                        );
                        let _ = store.save();
                    };
                }

                Some(())
            });

        Ok(())
    });
}