use std::convert::TryFrom;

use hdk::{
    self,
    AGENT_ADDRESS,
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    error::{
        ZomeApiResult,
        ZomeApiError,
    },
    holochain_core_types::{
        cas::content::Address,
        entry::Entry,
        dna::entry_types::Sharing,
        json::JsonString,
    },
};

use crate::anchors::Anchor;

const HANDLE: &str = "handle";
pub struct Handle;

impl Handle {
    pub fn definition() -> ValidatingEntryType {
        entry!(
            name: HANDLE,
            description: "A user handle for posting meows",
            sharing: Sharing::Public,
            native_type: Address,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_handle_anchor: Address, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                Handle::agent_link_definition()
            ]
        )
    }

    fn agent_link_definition() -> ValidatingLinkDefinition {
        from!(
            "%agent_id",
            tag: HANDLE,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    }

    fn create(handle: &str) -> ZomeApiResult<Address> {
        hdk::commit_entry(&Entry::App(HANDLE.into(),
            Anchor::create(HANDLE, handle)?.into()
        ))
    }

    fn exists(handle: &str) -> ZomeApiResult<bool> {
        Anchor::exists(HANDLE, handle)
    }
}

pub fn handle_app_property(key: String) -> JsonString {
    match app_property(&key) {
        Ok(value) => json!({ "value": value }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_use_handle(handle: String) -> JsonString {
    match use_handle(&handle) {
        Ok(address) => json!({ "value": address }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_get_handle(address: String) -> JsonString {
    match get_handle(&address.into()) {
        Ok(handle) => json!({ "value": handle }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

pub fn handle_log_out() -> JsonString {
    match log_out() {
        Ok(logged_out) => json!({ "value": logged_out }).into(),
        Err(hdk_err) => json!({ "error": hdk_err }).into(),
    }
}

// incomplete
fn app_property(key: &str) -> ZomeApiResult<String> {
    match key {
        "Agent_Address" => Ok(AGENT_ADDRESS.to_string()),
        "Agent_Handle" => get_handle(&AGENT_ADDRESS),
        _ => Err(ZomeApiError::ValidationFailed(
            format!("No App Property with key: {}", key)
        )),
    }
}

// incomplete
pub fn use_handle(handle: &str) -> ZomeApiResult<Address> {
    if Handle::exists(handle)? {
        return Err(ZomeApiError::ValidationFailed("handle_in_use".into()));
    }
    let handle_addr = Handle::create(handle)?;
    hdk::link_entries(&AGENT_ADDRESS, &handle_addr, HANDLE)?;
    Ok(handle_addr)
}

fn get_handle(addr: &Address) -> ZomeApiResult<String> {
    let mut addr = addr;
    let links = hdk::get_links(addr, HANDLE)?;
    let addrs = links.addresses();
    if !addrs.is_empty() {
        addr = &addrs[0];
    }
    if let Some(entry) = hdk::get_entry(&addr)? {
        if let Entry::App(entry_type, value) = entry {
            if entry_type.to_string() == HANDLE {
                return Ok(Anchor::get(
                    &Address::try_from(value)?)?
                    .get_text().to_string()
                );
            }
        }
    }
    Err(ZomeApiError::ValidationFailed("get_handle called on non-handle address".into()))
}

fn log_out() -> ZomeApiResult<String> {
    Err(ZomeApiError::Internal("cannot delete link entries yet".into()))
}
