use async_hwi::HWI;

#[cfg(feature = "specter")]
use async_hwi::specter::{Specter, SpecterSimulator};

#[cfg(feature = "ledger")]
use async_hwi::ledger::{Ledger, LedgerSimulator};

#[tokio::main]
pub async fn main() {
    let list = list_hardware_wallets().await;
    eprintln!(
        "{} device{} connected",
        list.len(),
        if list.len() > 1 { "s" } else { "" }
    );

    for hw in list {
        eprintln!(
            "{} (fingerprint: {})",
            hw.device_kind(),
            hw.get_master_fingerprint().await.unwrap()
        );
    }
}

pub async fn list_hardware_wallets() -> Vec<Box<dyn HWI + Send>> {
    let mut hws = Vec::new();

    #[cfg(feature = "specter")]
    if let Ok(device) = SpecterSimulator::try_connect().await {
        hws.push(device.into());
    }

    #[cfg(feature = "specter")]
    if let Ok(device) = Specter::try_connect_serial().await {
        hws.push(device.into());
    }

    #[cfg(feature = "ledger")]
    if let Ok(device) = LedgerSimulator::try_connect().await {
        hws.push(device.into());
    }

    #[cfg(feature = "ledger")]
    if let Ok(device) = Ledger::try_connect_hid() {
        hws.push(device.into());
    }

    hws
}