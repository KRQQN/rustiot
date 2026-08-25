// Persistent storage for Wi-Fi credentials in the dedicated `wifi_cfg` partition.
// Partition table (from boot log):
//   wifi_cfg  →  Offset 0x310000, Length 0x1000 (4 KB)

use core::{mem::size_of, ptr, slice, str};
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use heapless::String;

const MAGIC: u32 = 0x5749_4649;

// must match BLE side limits
const SSID_MAX: usize = 32;
const PASS_MAX: usize = 64;

const WIFI_CFG_OFFSET: u32 = 0x310000;
const SECTOR_SIZE: u32 = 0x1000;

#[derive(Clone)]
pub struct WifiCredentials {
    pub ssid: String<SSID_MAX>,
    pub password: String<PASS_MAX>,
}

// Struct that is written to flash
#[repr(C)]
struct StoredWifi {
    magic: u32,
    ssid_len: u8,
    pass_len: u8,
    ssid: [u8; SSID_MAX],
    password: [u8; PASS_MAX],
    // todo?: add a CRC32 here later for extra integrity checking
}

impl StoredWifi {
    fn from_credentials(creds: &WifiCredentials) -> Self {
        let mut s = Self {
            magic: MAGIC,
            ssid_len: creds.ssid.len() as u8,
            pass_len: creds.password.len() as u8,
            ssid: [0; SSID_MAX],
            password: [0; PASS_MAX],
        };

        s.ssid[..creds.ssid.len()].copy_from_slice(creds.ssid.as_bytes());
        s.password[..creds.password.len()].copy_from_slice(creds.password.as_bytes());
        s
    }

    fn to_credentials(&self) -> Option<WifiCredentials> {
        if self.magic != MAGIC {
            return None;
        }

        let ssid = str::from_utf8(&self.ssid[..self.ssid_len as usize]).ok()?;
        let pass = str::from_utf8(&self.password[..self.pass_len as usize]).ok()?;

        let mut s = String::new();
        let mut p = String::new();
        s.push_str(ssid).ok()?;
        p.push_str(pass).ok()?;

        Some(WifiCredentials {
            ssid: s,
            password: p,
        })
    }
}

// Returns `None` when nothing valid is stored
pub fn load_wifi_credentials<S>(storage: &mut S) -> Option<WifiCredentials>
where
    S: ReadNorFlash,
{
    let mut buf = [0u8; size_of::<StoredWifi>()];

    storage.read(WIFI_CFG_OFFSET, &mut buf).ok()?;

    let stored: StoredWifi = unsafe { ptr::read(buf.as_ptr() as *const _) };

    stored.to_credentials()
}

pub fn save_wifi_credentials<S>(storage: &mut S, creds: &WifiCredentials) -> Result<(), S::Error>
where
    S: NorFlash,
{
    let stored = StoredWifi::from_credentials(creds);

    // Turn the struct into a byte slice
    let bytes =
        unsafe { slice::from_raw_parts(&stored as *const _ as *const u8, size_of::<StoredWifi>()) };

    // Erase the whole 4 KB sector before write
    storage.erase(WIFI_CFG_OFFSET, WIFI_CFG_OFFSET + SECTOR_SIZE)?;
    storage.write(WIFI_CFG_OFFSET, bytes)?;

    Ok(())
}

pub fn clear_wifi_credentials<S>(storage: &mut S) -> Result<(), S::Error>
where
    S: NorFlash,
{
    storage.erase(WIFI_CFG_OFFSET, WIFI_CFG_OFFSET + SECTOR_SIZE)
}
