mod errors;
pub use errors::{Error, Result};
mod consts;
use std::collections::BTreeMap;

pub use consts::DEFAULT_DOMAINS;
use serde::{Deserialize, Serialize};

pub fn defaults_export(domain: impl std::fmt::Display) -> Result<plist::Value> {
    let domain = domain.to_string();
    let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let path = iocore::Path::raw("/tmp")
        .join(format!("{}-{}", &domain, &ts))
        .try_canonicalize();

    let (exit_code, _, err) = iocore::shell_command_string_output(
        format!("defaults export {} {}", &domain, &path),
        "/tmp",
    )?;
    if exit_code != 0 {
        return Err(Error::IOError(format!(
            "defaults export {} failed[{}]: {}",
            &domain, exit_code, err
        )));
    }
    let bytes = match path.read_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            path.delete_unchecked();
            return Err(e.into());
        },
    };
    let plist = plist::from_bytes::<plist::Value>(&bytes)?;
    Ok(plist)
}

pub fn defaults_delete_domain(domain: impl std::fmt::Display) -> Result<(String, plist::Value)> {
    let domain = domain.to_string();
    let plist = defaults_export(&domain)?;
    match iocore::shell_command_string_output(format!("defaults delete {}", &domain), "/tmp")? {
        (0, _, _) => Ok((domain, plist)),
        (exit_code, _, err) => Err(Error::IOError(format!(
            "defaults delete {} failed[{}]: {}",
            &domain, exit_code, err
        ))),
    }
}
pub fn defaults_delete_domains(
    domains: Vec<String>,
) -> DeleteDefaultsMacOSResult {
    let mut errors = BTreeMap::<String, Error>::new();
    let mut domain_map = BTreeMap::<String, plist::Value>::new();

    for domain in domains {
        match defaults_delete_domain(&domain) {
            Ok((domain, plist)) => {
                domain_map.insert(domain, plist);
            },
            Err(e) => {
                errors.insert(domain.clone(), e);
            },
        }
        save_domain_map(&domain, domain_map.clone());
    }
    DeleteDefaultsMacOSResult { domain_map, errors }
}
pub fn save_domain_map(domain: impl std::fmt::Display, domains: BTreeMap<String, plist::Value>) {
    let domain = domain.to_string();
    let data = serde_json::to_string_pretty(&domains).unwrap_or_default();
    if data.is_empty() {
        return;
    }
    let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let path = iocore::Path::raw("/tmp")
        .join(format!("{}-{}", &domain, &ts))
        .try_canonicalize();

    path.write_unchecked(data.as_bytes());
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDefaultsMacOSResult {
    pub domain_map: BTreeMap<String, plist::Value>,
    pub errors: BTreeMap<String, Error>,
}
