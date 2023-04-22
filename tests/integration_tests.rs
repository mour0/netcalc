use netcalc::utils::{ipv4::{get_class, cidr_to_sm, sm_to_wm, vlsm, get_network, possible_host}, structs::{IPv4Class, NetHelperError}, ipv6::{cidr_to_sm6, sm_to_wm6, get_network6}};

mod constants;
use crate::constants::{IPV4_HOSTS, IPV6_HOSTS, SUBNETMASK, SUBNETMASK6, WILDCARD, WILDCARD6};



#[test]
fn network_class() {
    for i in 0..=255 {
        if i >= 0 && i <= 127 {
            assert_eq!(get_class(i).unwrap(), IPv4Class::A);
        } else if i >= 128 && i <= 191 {
            assert_eq!(get_class(i).unwrap(), IPv4Class::B);
        } else if i >= 192 && i <= 223 {
            assert_eq!(get_class(i).unwrap(), IPv4Class::C);
        } else if i >= 224 && i <= 239 {
            assert_eq!(get_class(i).unwrap(), IPv4Class::D);
        } else if i >= 240 && i <= 255 {
            assert_eq!(get_class(i).unwrap(), IPv4Class::E);
        }
    }
}

#[test]
fn cidr_mask() {
    for i in 0..=40 {
        match cidr_to_sm(i) {
            Ok(sm) => {
                println!("{i}. {:?}", sm);
                assert_eq!(sm, SUBNETMASK[i as usize]);
            },
            Err(e) => {
                println!("{i}. {:?}", e);
            }
        }
    } 
}
#[test]
fn cidr_mask6() {
    for i in 0..=140 {
        match cidr_to_sm6(i) {
            Ok(sm) => {
                println!("{i}. {:?}", sm);
                assert_eq!(sm, SUBNETMASK6[i as usize]);
            },
            Err(e) => {
                println!("{i}. {:?}", e);
            }
        }
    } 
}


#[test]
fn mask_wildcard()
{
    for i in 0..=40 {
        match cidr_to_sm(i)
        {
            Ok(sm) => {
                let wm = sm_to_wm(sm);
                println!("{i}. {:?}", wm);
                assert_eq!(wm, WILDCARD[i as usize]);
            },
            Err(e) => {
                println!("{i}. {:?}", e);
            }
        }
    } 
}

#[test]
fn mask_wildcard6()
{
    for i in 0..=140 {
        match cidr_to_sm6(i)
        {
            Ok(sm) => {
                let wm = sm_to_wm6(sm);
                println!("{i}. {:?}", wm);
                assert_eq!(wm, WILDCARD6[i as usize]);
            },
            Err(e) => {
                println!("{i}. {:?}", e);
            }
        }
    } 
}


#[test]
fn vlsm_test() {
    match vlsm("192.168.1.1", 24, vec![30,20,1])
    {
        Ok(a) => {
            assert_eq!(a,[["192.168.1.0/27", "192.168.1.31", "192.168.1.1", "192.168.1.30", "30 (2^5)"], ["192.168.1.32/27", "192.168.1.63", "192.168.1.33", "192.168.1.62", "20 (2^5)"], ["192.168.1.64/30", "192.168.1.67", "192.168.1.65", "192.168.1.66", "1 (2^2)"]]);
        },
        Err(e) => {
            println!("{}", e);
        }
    };
    match vlsm("192.168.1.1", 16, vec![1,1555,30])
    {
        Ok(a) => {
            assert_eq!(a, [["192.168.0.0/21", "192.168.7.255", "192.168.0.1", "192.168.7.254", "1555 (2^11)"], ["192.168.8.0/27", "192.168.8.31", "192.168.8.1", "192.168.8.30", "30 (2^5)"], ["192.168.8.32/30", "192.168.8.35", "192.168.8.33", "192.168.8.34", "1 (2^2)"]]);
        },
        Err(e) => {
            println!("{}", e);
        }
    };

    match vlsm("192.168.1.1", 24, vec![100,27,27,27,27])
    {
        Ok(a) => {
            assert_eq!(a, [["192.168.1.0/25", "192.168.1.127", "192.168.1.1", "192.168.1.126", "100 (2^7)"], ["192.168.1.128/27", "192.168.1.159", "192.168.1.129", "192.168.1.158", "27 (2^5)"], ["192.168.1.160/27", "192.168.1.191", "192.168.1.161", "192.168.1.190", "27 (2^5)"], ["192.168.1.192/27", "192.168.1.223", "192.168.1.193", "192.168.1.222", "27 (2^5)"], ["192.168.1.224/27", "192.168.1.255", "192.168.1.225", "192.168.1.254", "27 (2^5)"]]);
        },
        Err(e) => {
            println!("{}", e);
        }
    };

    match vlsm("192.168.1.1", 24, vec![100,27,27,27,27,27])
    {
        Ok(a) => {
            println!("{:?}",a);
        },
        Err(e) => {
            assert_eq!(e, NetHelperError::NoSpace);
        }
    };

    match vlsm("255.255.255.255", 8, vec![16_777_215])
    {
        Ok(a) => {
            println!("{:?}", a);
        },
        Err(e) => {
            assert_eq!(e, NetHelperError::NoSpace);
        }
    };

    match vlsm("192.168.1.1", 24, vec![126,126])
    {
        Ok(a) => {
            assert_eq!(a,[["192.168.1.0/25", "192.168.1.127", "192.168.1.1", "192.168.1.126", "126 (2^7)"], ["192.168.1.128/25", "192.168.1.255", "192.168.1.129", "192.168.1.254", "126 (2^7)"]]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
 
}

#[test]
fn network() {

    match get_network("192.168.1.1", 24)
    {
        Ok(a) => {
            assert_eq!(a[0],[192,168,1,0]);
            assert_eq!(a[1],[255,255,255,0]);
            assert_eq!(a[2],[0,0,0,255]);
            assert_eq!(a[3],[192,168,1,255]);
            assert_eq!(a[4],[192,168,1,1]);
            assert_eq!(a[5],[192,168,1,254]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    match get_network("192.168.1.1", 30)
    {
        Ok(a) => {
            assert_eq!(a[0],[192,168,1,0]);
            assert_eq!(a[1],[255,255,255,252]);
            assert_eq!(a[2],[0,0,0,3]);
            assert_eq!(a[3],[192,168,1,3]);
            assert_eq!(a[4],[192,168,1,1]);
            assert_eq!(a[5],[192,168,1,2]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
    
    match get_network("192.168.1.1", 1)
    {
        Ok(a) => {
            assert_eq!(a[0],[128,0,0,0]);
            assert_eq!(a[1],[128,0,0,0]);
            assert_eq!(a[2],[127,255,255,255]);
            assert_eq!(a[3],[255,255,255,255]);
            assert_eq!(a[4],[128,0,0,1]);
            assert_eq!(a[5],[255,255,255,254]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    match get_network("0.0.0.0", 1)
    {
        Ok(a) => {
            assert_eq!(a[0],[0,0,0,0]);
            assert_eq!(a[1],[128,0,0,0]);
            assert_eq!(a[2],[127,255,255,255]);
            assert_eq!(a[3],[127,255,255,255]);
            assert_eq!(a[4],[0,0,0,1]);
            assert_eq!(a[5],[127,255,255,254]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    match get_network("0.0.0.0", 30) {
        Ok(a) => {
            assert_eq!(a[0],[0,0,0,0]);
            assert_eq!(a[1],[255,255,255,252]);
            assert_eq!(a[2],[0,0,0,3]);
            assert_eq!(a[3],[0,0,0,3]);
            assert_eq!(a[4],[0,0,0,1]);
            assert_eq!(a[5],[0,0,0,2]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    match get_network("255.255.255.255", 1) {
        Ok(a) => {
            assert_eq!(a[0],[128,0,0,0]);
            assert_eq!(a[1],[128,0,0,0]);
            assert_eq!(a[2],[127,255,255,255]);
            assert_eq!(a[3],[255,255,255,255]);
            assert_eq!(a[4],[128,0,0,1]);
            assert_eq!(a[5],[255,255,255,254]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
    match get_network("255.255.255.255", 30)
    {
        Ok(a) => {
            assert_eq!(a[0],[255,255,255,252]);
            assert_eq!(a[1],[255,255,255,252]);
            assert_eq!(a[2],[0,0,0,3]);
            assert_eq!(a[3],[255,255,255,255]);
            assert_eq!(a[4],[255,255,255,253]);
            assert_eq!(a[5],[255,255,255,254]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    let a = get_network("255.255.256.3", 30);
    assert_eq!(a.is_err(), true, "{}", a.unwrap_err());
}

#[test]
fn network6()
{
    match get_network6("2001:0db8:85a3:0000:0000:8a2e:0370:7334", 64) {
        Ok(a) => {
            assert_eq!(a[0],[0x2001,0xdb8,0x85a3,0,0,0,0,0]);
            assert_eq!(a[1],[0xFFFF,0xFFFF,0xFFFF,0xFFFF,0,0,0,0]);
            assert_eq!(a[2],[0,0,0,0,0xFFFF,0xFFFF,0xFFFF,0xFFFF]);
            assert_eq!(a[3],[0x2001,0x0db8,0x85a3,0,0,0,0,0]);
            assert_eq!(a[4],[0x2001,0x0db8,0x85a3,0,0xffff,0xffff,0xffff,0xffff]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    match get_network6("2001:0db8:85a3:0000:0000:8a2e:0370:7334", 128) {
        Ok(a) => {
            assert_eq!(a[0],[0x2001,0xdb8,0x85a3,0,0,0x8a2e,0x0370,0x7334]);
            assert_eq!(a[1],[0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF]);
            assert_eq!(a[2],[0,0,0,0,0,0,0,0]);
            assert_eq!(a[3],[0x2001,0xdb8,0x85a3,0,0,0x8a2e,0x0370,0x7334]);
            assert_eq!(a[4],[0x2001,0xdb8,0x85a3,0,0,0x8a2e,0x0370,0x7334]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
    
    match get_network6("2001:0db8:85a3:0000:0000:8a2e:0370:7334", 1) {
        Ok(a) => {
            assert_eq!(a[0],[0,0,0,0,0,0,0,0]);
            assert_eq!(a[1],[0x8000,0,0,0,0,0,0,0]);
            assert_eq!(a[2],[0x7FFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF]);
            assert_eq!(a[3],[0,0,0,0,0,0,0,0]);
            assert_eq!(a[4],[0x7fff,0xffff,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF,0xFFFF]);
        },
        Err(e) => {
            println!("{}", e);
        }
    }

}

#[test]
fn test_host4()
{
    for i in 0..=32 {
        let a = possible_host(i);
        println!("{i}. {a}");
        assert_eq!(a, IPV4_HOSTS[i as usize]);
    }
}