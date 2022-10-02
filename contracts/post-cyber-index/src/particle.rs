/// Copy from https://github.com/cybercongress/cw-cyber/packages/cyber-std/src/particle.rs

use crate::error::ContractError;
use cid::multihash::{Code, MultihashDigest};
use cid::{Cid, Version};
use std::ops::Add;
use std::str::FromStr;

pub fn prepare_particle(input: String) -> Result<Cid, ContractError> {
    if input.len() == 0 || input.len() > 256 {
        return Err(ContractError::InvalidParticleData {});
    }

    // unixfs/dagnode/proto shortcut
    // wrap input bytes as a dagnode unixfs file
    let length: u8 = input.len() as u8;
    let mut raw: Vec<u8> = vec![10, length.add(6) as u8, 8, 2, 18, length];
    raw.append(&mut input.as_bytes().to_vec());
    raw.append(&mut vec![24, length]);

    let h = Code::Sha2_256.digest(&raw.as_slice());
    let particle = Cid::new_v0(h).unwrap();

    Ok(particle)
}

pub fn check_particle(input: String) -> Result<Cid, ContractError> {
    let particle: Cid;
    let try_particle = Cid::from_str(&input.clone());
    if try_particle.is_ok() {
        particle = try_particle.unwrap();
        if particle.version() != Version::V0 {
            return Err(ContractError::InvalidParticleVersion {});
        }
    } else {
        return Err(ContractError::InvalidParticle {});
    }

    Ok(particle)
}
