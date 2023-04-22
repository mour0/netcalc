use std::net::Ipv4Addr;
use super::structs::{IPv4Class, NetHelperError};

pub fn get_class(first_octet: u8) -> Result<IPv4Class,NetHelperError>{
    match first_octet {
        x if /*x >= 0 &&*/ x <= 127 => Ok(IPv4Class::A),
        x if x >= 128 && x <= 191 => Ok(IPv4Class::B),
        x if x >= 192 && x <= 223 => Ok(IPv4Class::C),
        x if x >= 224 && x <= 239 => Ok(IPv4Class::D),
        x if x >= 240 /*&& x <= 255*/ => Ok(IPv4Class::E),
        _ => Err(NetHelperError::UnknownClass),
    }
}

/// Calculates the subnet mask from the provided short notation
/// __in__ 'cidr': short notation Subnet Mask
pub fn cidr_to_sm(cidr: u8) -> Result<[u8; 4], NetHelperError> {
    if cidr > 32 {
        return Err(NetHelperError::InvalidCidr);
    }

    let mask_vec:[u8;4];
    if cidr == 0 {
        mask_vec = [0;4];
        return Ok(mask_vec);
    }
    let mask:u32 = 0xFFFFFFFF << (32 - cidr);
    mask_vec = [((mask >> 24 )).try_into().unwrap(), (mask >> 16 & 0xFF).try_into().unwrap(), (mask >> 8 & 0xFF).try_into().unwrap(), ((mask & 0xFF)).try_into().unwrap()];
    Ok(mask_vec)
}

/// Calculates the wildcard mask from the provided subnet mask 
/// __in__ 'sm': Subnet Mask
/// There are no checks because this function will be called with the output provided by `cidr_to_sm`
pub fn sm_to_wm(sm: [u8; 4]) -> [u8; 4] {
    let wm:[u8;4];
    wm = [!sm[0], !sm[1], !sm[2], !sm[3]];
    wm
}


/// Calculates Network, Subnet Mask (Long), Wildcard Mask, Broadcast, First and Last Hosts for the provided parameters
/// __in__ 'ip_str': String containing the IPv4 Address (ex. 192.168.1.1)
/// __in__ 'cidr': Short notation Subnet Mask
pub fn get_network(ip4_str:&str,cidr:u8) -> Result<[[u8; 4]; 6],NetHelperError>
{
    // Retrieve Network, SM, Wildcard, Brodcast, First Host, Last Host
    let ip: Ipv4Addr = match ip4_str.parse()  {
        Ok(ip) => ip,
        Err(_) => return Err(NetHelperError::InvalidIP),
    };

    let sm = cidr_to_sm(cidr)?;
    let wm = sm_to_wm(sm);

    let network: [u8;4] = [ip.octets()[0] & sm[0], ip.octets()[1] & sm[1], ip.octets()[2] & sm[2], ip.octets()[3] & sm[3]];
    let broadcast: [u8;4] = [network[0] + wm[0], network[1] + wm[1], network[2] + wm[2], network[3] + wm[3]];
    let mut first_host: [u8;4] = [network[0], network[1], network[2], network[3]];
    let mut last_host: [u8; 4] = [broadcast[0], broadcast[1], broadcast[2], broadcast[3]];
    if cidr < 31 {
        first_host[3] = network[3] + 1;
        last_host[3] = broadcast[3] - 1;
    }

    Ok([network,sm,wm,broadcast,first_host,last_host])
}

/// Calculates all the possible hosts from te subnet mask
/// __in__ 'cidr': short notation subnet mask
pub fn possible_host(cidr:u8) -> u32
{
    let addr_num = 2u64.pow(32 - cidr as u32);
    if cidr < 31 {
        (addr_num - 2).try_into().unwrap()
    } else {
        addr_num.try_into().unwrap()
    }
}


/// Check if the network is big enough to hold every host
/// __in__ 'n_host': Vector of hosts per network
/// __in__ 'max cidr': Max CIDR to hold every host
fn is_space_enough(n_host: Vec<u32>,max_cidr:u8) -> Result<Vec<(u32,u8)>,NetHelperError>
{
    let mut nec_cir: u8 = 0;
    let mut sum: u32 = 0;
    let mut vec_tuple: Vec<(u32,u8)> = Vec::new();
    for i in n_host
    {
        // + 2 because of network and broadcast addresses
        while 2u32.pow(nec_cir.into()) < i + 2
        {
            nec_cir += 1;
        }
        sum += possible_host(32 - nec_cir) + 2;
        vec_tuple.push((i,nec_cir));
        nec_cir = 0;
    }

    if sum - 2 <= possible_host(max_cidr) { 
        vec_tuple.sort_by(|a,b| b.0.cmp(&a.0));
        Ok(vec_tuple) 

    }
    else {
        Err(NetHelperError::NoSpace)
    }

}

/// Calculates VLSM 
pub fn vlsm(ip:&str,cidr:u8,n_hosts: Vec<u32>) -> Result<Vec<Vec<String>>,NetHelperError>
{
    if cidr >= 31 || cidr == 0 {
        return Err(NetHelperError::InvalidCidrVLSM);
    }
    
    let v = is_space_enough(n_hosts,cidr)?;

    let mut network = get_network(ip,cidr)?; 

    let mut subnets = Vec::new(); 

    for i in 0..v.len()
    {
        

        let mask_sub = 32 - v[i].1; 
        match get_network(format!("{}.{}.{}.{}",network[0][0],network[0][1],network[0][2],network[0][3]).as_str(),mask_sub)
        {
            Ok(net) => {
                let mut array: Vec<String> = Vec::new(); 
                array.push(format!("{}.{}.{}.{}/{}",net[0][0],net[0][1],net[0][2],net[0][3],mask_sub)); // network
                array.push(format!("{}.{}.{}.{}",net[3][0],net[3][1],net[3][2],net[3][3]));             // broadcast
                array.push(format!("{}.{}.{}.{}",net[4][0],net[4][1],net[4][2],net[4][3]));             // first host
                array.push(format!("{}.{}.{}.{}",net[5][0],net[5][1],net[5][2],net[5][3]));             // last host
                //array.push(format!("{}.{}.{}.{}",net[0][0],net[0][1],net[0][2],net[0][3]));
                array.push(format!("{} (2^{})",v[i].0,v[i].1));                                   // hosts
                subnets.push(array);

                if i != v.len()-1
                {
                    let mut overflow_1: u8 = 0;
                    let mut overflow_2: u8 = 0;
                    let mut overflow_3: u8 = 0;

                    match net[3][3].overflowing_add(1) 
                    {
                        (_,true) => {
                            network[0][3] = 0;
                            overflow_3 = 1;
                        },
                        (x,false) => {
                            network[0][3] = x;
                        }
                    }

                    match net[3][2].overflowing_add(overflow_3) 
                    {
                        (_,true) => {
                            network[0][2] = 0;
                            overflow_2 = 1;
                        },
                        (x,false) => {
                            network[0][2] = x;
                        }
                    }

                    match net[3][1].overflowing_add(overflow_2) 
                    {
                        (_,true) => {
                            network[0][1] = 0;
                            overflow_1 = 1;
                        },
                        (x,false) => {
                            network[0][1] = x;
                        }
                    }

                    match net[3][0].overflowing_add(overflow_1) 
                    {
                        (_,true) => {
                            return Err(NetHelperError::NoSpace); 
                        },
                        (x,false) => {
                            network[0][0] = x;
                        }
                    }
                }

            },
            Err(e) => return Err(e),
        }

    }
    Ok(subnets)
}