use std::net::{IpAddr};

type IpMaskType = [u8; 4];

static CLASS_A_MASK: IpMaskType = [0xff, 0, 0, 0];
static CLASS_B_MASK: IpMaskType = [0xff, 0xff, 0, 0];
static CLASS_C_MASK: IpMaskType = [0xff, 0xff, 0xff, 0];

pub trait IpMask {
    fn default_mask(&self) -> Option<IpMaskType>;
}

impl IpMask for IpAddr {
    fn default_mask(&self) -> Option<IpMaskType> {
        match to_4(self) {
            Some(octet) => {
                match octet[0] {
                    0x80 => Some(CLASS_A_MASK),
                    0xC0 => Some(CLASS_B_MASK),
                    _ => Some(CLASS_C_MASK)
                }
            }
            None => None
        }
    }
}

fn is_zeros(ip: &[u8]) -> bool {
    ip.iter().all(|&x| x == 0)
}

fn to_4(ip: &IpAddr) -> Option<[u8; 4]> {
    if let IpAddr::V4(ipv4) = ip {
        return Some(ipv4.octets());
    }

    if let IpAddr::V6(ipv46) = ip {
        let mut octets = ipv46.octets();

        if is_zeros(&octets[0..10]) && &octets[10] == &0xff && &octets[11] == &0xff {
            let v: [u8; 4] = [0, 0, 0, 0];
            octets[12..16].clone_from_slice(&v);
            return Some(v);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_is_zeros() {
        assert_eq!(is_zeros(&[0, 0, 12, 0]), false);
        assert_eq!(is_zeros(&[0, 0, 0, 0]), true)
    }

    #[test]
    fn test_to_4() {
        let ipaddr = IpAddr::from_str("").unwrap();
        let to = to_4(&ipaddr);
        println!("{:?}", to);
        assert_eq!(true, true)
    }
}