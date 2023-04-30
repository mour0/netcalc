use std::net::Ipv6Addr;

use super::structs::NetHelperError;

/// IPv6
/// Calculates the subnet mask from the provided short notation
/// __in__ 'cidr': short subnet mask notation
pub fn cidr_to_sm6(cidr: u8) -> Result<[u16; 8], NetHelperError> {
    if cidr > 128 {
        return Err(NetHelperError::InvalidCidr);
    }

    let mask_vec:[u16;8];
    if cidr == 0 {
        mask_vec = [0;8];
        return Ok(mask_vec);
    }

    let mask: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF << (128 - cidr);
    mask_vec = [((mask >> 112 )).try_into().unwrap(), (mask >> 96 & 0xFFFF).try_into().unwrap(), (mask >> 80 & 0xFFFF).try_into().unwrap(), ((mask >> 64 & 0xFFFF)).try_into().unwrap(), (mask >> 48 & 0xFFFF).try_into().unwrap(), (mask >> 32 & 0xFFFF).try_into().unwrap(), (mask >> 16 & 0xFFFF).try_into().unwrap(), ((mask & 0xFFFF)).try_into().unwrap()];
    Ok(mask_vec)
}

/// IPv6
/// Calculates the wildcard mask from the provided subnet mask 
/// __in__ 'sm': Subnet Mask
pub fn sm_to_wm6(sm: [u16; 8]) -> [u16; 8] {
    let wm:[u16;8];
    wm = [!sm[0], !sm[1], !sm[2], !sm[3], !sm[4], !sm[5], !sm[6], !sm[7]];
    wm
}

/// IPv6
/// Calculates Network, Subnet Mask (Long), Wildcard Mask, Broadcast, First and Last Hosts for the provided parameters
/// __in__ 'ip_str': String containing the IPv6 Address 
/// __in__ 'cidr': Short notation Subnet Mask
pub fn get_network6(ip6_str:&str,cidr:u8) -> Result<[[u16; 8]; 5], NetHelperError>
{
    let ip: Ipv6Addr = match ip6_str.parse()  {
        Ok(ip) => ip,
        Err(_) => return Err(NetHelperError::InvalidIP),
    };
        
    let sm = match cidr_to_sm6(cidr) {
        Ok(sm) => sm,
        Err(err) => return Err(err),
    };
    let wm = sm_to_wm6(sm);

    let network: [u16;8] = [ip.segments()[0] & sm[0], ip.segments()[1] & sm[1], ip.segments()[2] & sm[2], ip.segments()[3] & sm[3], ip.segments()[4] & sm[4], ip.segments()[5] & sm[5], ip.segments()[6] & sm[6], ip.segments()[7] & sm[7]];
    let first_host: [u16;8] = [network[0], network[1], network[2], network[3], network[4], network[5], network[6], network[7]];
    let last_host: [u16;8] = [network[0] + wm[0], network[1] + wm[1], network[2] + wm[2], network[3] + wm[3], network[4] + wm[4], network[5] + wm[5], network[6] + wm[6], network[7] + wm[7]]; 

    Ok([network,sm,wm,first_host,last_host])
}

/// IPv6
/// Calculates all the possible hosts from the subnet mask
/// __in__ 'cidr': short notation subnet mask
pub fn possible_host6(cidr:u8) -> u128
{
    // If cidr==128 => u128::MAX, missing 1 host
    // Temp before using bigint
    2u128.saturating_pow(128 - cidr as u32)
}