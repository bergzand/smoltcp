
use heapless::LinearMap;

use crate::config::IFACE_SLAAC_PREFIX_COUNT;
use crate::time::{Duration, Instant};
use crate::wire::{HardwareAddress, IpAddress};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Prefix {
    preferred_until: Instant,
    valid_until: Instant,
}

/// SLAAC runtime state
#[derive(Debug)]
pub struct SlaacState {
    prefix: LinearMap<IpAddress, Prefix, IFACE_SLAAC_PREFIX_COUNT>,
}

impl SlaacState {
    fn new() -> Self {
        Self {
            prefix: LinearMap::new()
        }
    }


}
