use p4rs::{checksum::Checksum, *};
use colored::*;
use bitvec::prelude::*;
#[usdt::provider]
mod softnpu_provider {
    fn parser_accepted(_: &str) {}
    fn parser_transition(_: &str) {}
    fn parser_dropped() {}
    fn control_apply(_: &str) {}
    fn control_table_hit(_: &str) {}
    fn control_table_miss(_: &str) {}
    fn ingress_dropped(_: &str) {}
    fn ingress_accepted(_: &str) {}
    fn egress_dropped(_: &str) {}
    fn egress_accepted(_: &str) {}
    fn egress_table_hit(_: &str) {}
    fn egress_table_miss(_: &str) {}
    fn action(_: &str) {}
}
#[derive(Debug, Default, Clone)]
pub struct ingress_metadata_t {
    pub port: BitVec<u8, Msb0>,
    pub nat: bool,
    pub nat_id: BitVec<u8, Msb0>,
    pub drop: bool,
}
impl ingress_metadata_t {
    fn valid_header_size(&self) -> usize {
        let mut x: usize = 0;
        x += 16usize;
        x += 16usize;
        x
    }
    fn to_bitvec(&self) -> BitVec<u8, Msb0> {
        let mut x = bitvec![u8, Msb0; 0; self.valid_header_size()];
        let mut off = 0;
        x[off..off + 16usize] |= self.port.to_bitvec();
        off += 16usize;
        x[off..off + 16usize] |= self.nat_id.to_bitvec();
        off += 16usize;
        x
    }
    fn dump(&self) -> String {
        format!(
            "{}: {}\n{}: {}\n{}: {}\n{}: {}", "port".blue(), p4rs::dump_bv(& self.port),
            "nat".blue(), self.nat, "nat_id".blue(), p4rs::dump_bv(& self.nat_id), "drop"
            .blue(), self.drop
        )
    }
}
#[derive(Debug, Default, Clone)]
pub struct ipv6_t {
    pub valid: bool,
    pub version: BitVec<u8, Msb0>,
    pub traffic_class: BitVec<u8, Msb0>,
    pub flow_label: BitVec<u8, Msb0>,
    pub payload_len: BitVec<u8, Msb0>,
    pub next_hdr: BitVec<u8, Msb0>,
    pub hop_limit: BitVec<u8, Msb0>,
    pub src: BitVec<u8, Msb0>,
    pub dst: BitVec<u8, Msb0>,
}
impl Header for ipv6_t {
    fn new() -> Self {
        Self {
            valid: false,
            version: BitVec::<u8, Msb0>::default(),
            traffic_class: BitVec::<u8, Msb0>::default(),
            flow_label: BitVec::<u8, Msb0>::default(),
            payload_len: BitVec::<u8, Msb0>::default(),
            next_hdr: BitVec::<u8, Msb0>::default(),
            hop_limit: BitVec::<u8, Msb0>::default(),
            src: BitVec::<u8, Msb0>::default(),
            dst: BitVec::<u8, Msb0>::default(),
        }
    }
    fn set(&mut self, buf: &[u8]) -> Result<(), TryFromSliceError> {
        self.version = {
            let mut b = buf.view_bits::<Msb0>()[0usize..4usize].to_owned();
            if 4usize - 0usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((4usize - 0usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (0usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(4usize - 0usize, false);
                b
            } else {
                b
            }
        };
        self.traffic_class = {
            let mut b = buf.view_bits::<Msb0>()[4usize..12usize].to_owned();
            if 12usize - 4usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((12usize - 4usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (4usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(12usize - 4usize, false);
                b
            } else {
                b
            }
        };
        self.flow_label = {
            let mut b = buf.view_bits::<Msb0>()[12usize..32usize].to_owned();
            if 32usize - 12usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((32usize - 12usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (12usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(32usize - 12usize, false);
                b
            } else {
                b
            }
        };
        self.payload_len = {
            let mut b = buf.view_bits::<Msb0>()[32usize..48usize].to_owned();
            if 48usize - 32usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((48usize - 32usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (32usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(48usize - 32usize, false);
                b
            } else {
                b
            }
        };
        self.next_hdr = {
            let mut b = buf.view_bits::<Msb0>()[48usize..56usize].to_owned();
            if 56usize - 48usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((56usize - 48usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (48usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(56usize - 48usize, false);
                b
            } else {
                b
            }
        };
        self.hop_limit = {
            let mut b = buf.view_bits::<Msb0>()[56usize..64usize].to_owned();
            if 64usize - 56usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((64usize - 56usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (56usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(64usize - 56usize, false);
                b
            } else {
                b
            }
        };
        self.src = {
            let mut b = buf.view_bits::<Msb0>()[64usize..192usize].to_owned();
            if 192usize - 64usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((192usize - 64usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (64usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(192usize - 64usize, false);
                b
            } else {
                b
            }
        };
        self.dst = {
            let mut b = buf.view_bits::<Msb0>()[192usize..320usize].to_owned();
            if 320usize - 192usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((320usize - 192usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (192usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(320usize - 192usize, false);
                b
            } else {
                b
            }
        };
        Ok(())
    }
    fn size() -> usize {
        320usize
    }
    fn set_valid(&mut self) {
        self.valid = true;
    }
    fn set_invalid(&mut self) {
        self.valid = false;
    }
    fn is_valid(&self) -> bool {
        self.valid
    }
    fn to_bitvec(&self) -> BitVec<u8, Msb0> {
        let mut x = bitvec![u8, Msb0; 0u8; Self::size()];
        if 4usize - 0usize > 8 {
            let mut v = self.version.clone().into_vec();
            if ((4usize - 0usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((4usize - 0usize) % 8);
                }
            }
            v.reverse();
            let n = (4usize - 0usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[0usize..4usize] |= &b[m..];
            }
        } else {
            x[0usize..4usize] |= self.version.to_owned();
        };
        if 12usize - 4usize > 8 {
            let mut v = self.traffic_class.clone().into_vec();
            if ((12usize - 4usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((12usize - 4usize) % 8);
                }
            }
            v.reverse();
            let n = (12usize - 4usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[4usize..12usize] |= &b[m..];
            }
        } else {
            x[4usize..12usize] |= self.traffic_class.to_owned();
        };
        if 32usize - 12usize > 8 {
            let mut v = self.flow_label.clone().into_vec();
            if ((32usize - 12usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((32usize - 12usize) % 8);
                }
            }
            v.reverse();
            let n = (32usize - 12usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[12usize..32usize] |= &b[m..];
            }
        } else {
            x[12usize..32usize] |= self.flow_label.to_owned();
        };
        if 48usize - 32usize > 8 {
            let mut v = self.payload_len.clone().into_vec();
            if ((48usize - 32usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((48usize - 32usize) % 8);
                }
            }
            v.reverse();
            let n = (48usize - 32usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[32usize..48usize] |= &b[m..];
            }
        } else {
            x[32usize..48usize] |= self.payload_len.to_owned();
        };
        if 56usize - 48usize > 8 {
            let mut v = self.next_hdr.clone().into_vec();
            if ((56usize - 48usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((56usize - 48usize) % 8);
                }
            }
            v.reverse();
            let n = (56usize - 48usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[48usize..56usize] |= &b[m..];
            }
        } else {
            x[48usize..56usize] |= self.next_hdr.to_owned();
        };
        if 64usize - 56usize > 8 {
            let mut v = self.hop_limit.clone().into_vec();
            if ((64usize - 56usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((64usize - 56usize) % 8);
                }
            }
            v.reverse();
            let n = (64usize - 56usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[56usize..64usize] |= &b[m..];
            }
        } else {
            x[56usize..64usize] |= self.hop_limit.to_owned();
        };
        if 192usize - 64usize > 8 {
            let mut v = self.src.clone().into_vec();
            if ((192usize - 64usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((192usize - 64usize) % 8);
                }
            }
            v.reverse();
            let n = (192usize - 64usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[64usize..192usize] |= &b[m..];
            }
        } else {
            x[64usize..192usize] |= self.src.to_owned();
        };
        if 320usize - 192usize > 8 {
            let mut v = self.dst.clone().into_vec();
            if ((320usize - 192usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((320usize - 192usize) % 8);
                }
            }
            v.reverse();
            let n = (320usize - 192usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[192usize..320usize] |= &b[m..];
            }
        } else {
            x[192usize..320usize] |= self.dst.to_owned();
        };
        x
    }
}
impl Checksum for ipv6_t {
    fn csum(&self) -> BitVec<u8, Msb0> {
        let mut csum = BitVec::new();
        csum = p4rs::bitmath::add_le(csum.clone(), self.version.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.traffic_class.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.flow_label.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.payload_len.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.next_hdr.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.hop_limit.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.src.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.dst.csum());
        csum
    }
}
impl ipv6_t {
    fn setValid(&mut self) {
        self.valid = true;
    }
    fn setInvalid(&mut self) {
        self.valid = false;
    }
    fn isValid(&self) -> bool {
        self.valid
    }
    fn dump(&self) -> String {
        if self.isValid() {
            format!(
                "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", "version".cyan(),
                p4rs::dump_bv(& self.version), "traffic_class".cyan(), p4rs::dump_bv(&
                self.traffic_class), "flow_label".cyan(), p4rs::dump_bv(& self
                .flow_label), "payload_len".cyan(), p4rs::dump_bv(& self.payload_len),
                "next_hdr".cyan(), p4rs::dump_bv(& self.next_hdr), "hop_limit".cyan(),
                p4rs::dump_bv(& self.hop_limit), "src".cyan(), p4rs::dump_bv(& self.src),
                "dst".cyan(), p4rs::dump_bv(& self.dst)
            )
        } else {
            "∅".to_owned()
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct ethernet_t {
    pub valid: bool,
    pub dst: BitVec<u8, Msb0>,
    pub src: BitVec<u8, Msb0>,
    pub ether_type: BitVec<u8, Msb0>,
}
impl Header for ethernet_t {
    fn new() -> Self {
        Self {
            valid: false,
            dst: BitVec::<u8, Msb0>::default(),
            src: BitVec::<u8, Msb0>::default(),
            ether_type: BitVec::<u8, Msb0>::default(),
        }
    }
    fn set(&mut self, buf: &[u8]) -> Result<(), TryFromSliceError> {
        self.dst = {
            let mut b = buf.view_bits::<Msb0>()[0usize..48usize].to_owned();
            if 48usize - 0usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((48usize - 0usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (0usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(48usize - 0usize, false);
                b
            } else {
                b
            }
        };
        self.src = {
            let mut b = buf.view_bits::<Msb0>()[48usize..96usize].to_owned();
            if 96usize - 48usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((96usize - 48usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (48usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(96usize - 48usize, false);
                b
            } else {
                b
            }
        };
        self.ether_type = {
            let mut b = buf.view_bits::<Msb0>()[96usize..112usize].to_owned();
            if 112usize - 96usize > 8 {
                let mut v = b.into_vec();
                v.reverse();
                if ((112usize - 96usize) % 8) != 0 {
                    if let Some(x) = v.iter_mut().last() {
                        *x <<= (96usize % 8);
                    }
                }
                let mut b = BitVec::<u8, Msb0>::from_vec(v);
                b.resize(112usize - 96usize, false);
                b
            } else {
                b
            }
        };
        Ok(())
    }
    fn size() -> usize {
        112usize
    }
    fn set_valid(&mut self) {
        self.valid = true;
    }
    fn set_invalid(&mut self) {
        self.valid = false;
    }
    fn is_valid(&self) -> bool {
        self.valid
    }
    fn to_bitvec(&self) -> BitVec<u8, Msb0> {
        let mut x = bitvec![u8, Msb0; 0u8; Self::size()];
        if 48usize - 0usize > 8 {
            let mut v = self.dst.clone().into_vec();
            if ((48usize - 0usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((48usize - 0usize) % 8);
                }
            }
            v.reverse();
            let n = (48usize - 0usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[0usize..48usize] |= &b[m..];
            }
        } else {
            x[0usize..48usize] |= self.dst.to_owned();
        };
        if 96usize - 48usize > 8 {
            let mut v = self.src.clone().into_vec();
            if ((96usize - 48usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((96usize - 48usize) % 8);
                }
            }
            v.reverse();
            let n = (96usize - 48usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[48usize..96usize] |= &b[m..];
            }
        } else {
            x[48usize..96usize] |= self.src.to_owned();
        };
        if 112usize - 96usize > 8 {
            let mut v = self.ether_type.clone().into_vec();
            if ((112usize - 96usize) % 8) != 0 {
                if let Some(x) = v.iter_mut().last() {
                    *x >>= ((112usize - 96usize) % 8);
                }
            }
            v.reverse();
            let n = (112usize - 96usize);
            let m = n % 8;
            let mut b = BitVec::<u8, Msb0>::from_vec(v);
            if b.len() > m {
                x[96usize..112usize] |= &b[m..];
            }
        } else {
            x[96usize..112usize] |= self.ether_type.to_owned();
        };
        x
    }
}
impl Checksum for ethernet_t {
    fn csum(&self) -> BitVec<u8, Msb0> {
        let mut csum = BitVec::new();
        csum = p4rs::bitmath::add_le(csum.clone(), self.dst.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.src.csum());
        csum = p4rs::bitmath::add_le(csum.clone(), self.ether_type.csum());
        csum
    }
}
impl ethernet_t {
    fn setValid(&mut self) {
        self.valid = true;
    }
    fn setInvalid(&mut self) {
        self.valid = false;
    }
    fn isValid(&self) -> bool {
        self.valid
    }
    fn dump(&self) -> String {
        if self.isValid() {
            format!(
                "{} {} {} {} {} {}", "dst".cyan(), p4rs::dump_bv(& self.dst), "src"
                .cyan(), p4rs::dump_bv(& self.src), "ether_type".cyan(), p4rs::dump_bv(&
                self.ether_type)
            )
        } else {
            "∅".to_owned()
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct egress_metadata_t {
    pub port: BitVec<u8, Msb0>,
    pub nexthop_v6: BitVec<u8, Msb0>,
    pub nexthop_v4: BitVec<u8, Msb0>,
    pub drop: bool,
    pub broadcast: bool,
}
impl egress_metadata_t {
    fn valid_header_size(&self) -> usize {
        let mut x: usize = 0;
        x += 16usize;
        x += 128usize;
        x += 32usize;
        x
    }
    fn to_bitvec(&self) -> BitVec<u8, Msb0> {
        let mut x = bitvec![u8, Msb0; 0; self.valid_header_size()];
        let mut off = 0;
        x[off..off + 16usize] |= self.port.to_bitvec();
        off += 16usize;
        x[off..off + 128usize] |= self.nexthop_v6.to_bitvec();
        off += 128usize;
        x[off..off + 32usize] |= self.nexthop_v4.to_bitvec();
        off += 32usize;
        x
    }
    fn dump(&self) -> String {
        format!(
            "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}", "port".blue(), p4rs::dump_bv(& self
            .port), "nexthop_v6".blue(), p4rs::dump_bv(& self.nexthop_v6), "nexthop_v4"
            .blue(), p4rs::dump_bv(& self.nexthop_v4), "drop".blue(), self.drop,
            "broadcast".blue(), self.broadcast
        )
    }
}
#[derive(Debug, Default, Clone)]
pub struct headers_t {
    pub ethernet: ethernet_t,
    pub ipv6: ipv6_t,
}
impl headers_t {
    fn valid_header_size(&self) -> usize {
        let mut x: usize = 0;
        if self.ethernet.valid {
            x += ethernet_t::size();
        }
        if self.ipv6.valid {
            x += ipv6_t::size();
        }
        x
    }
    fn to_bitvec(&self) -> BitVec<u8, Msb0> {
        let mut x = bitvec![u8, Msb0; 0; self.valid_header_size()];
        let mut off = 0;
        if self.ethernet.valid {
            x[off..off + ethernet_t::size()] |= self.ethernet.to_bitvec();
            off += ethernet_t::size();
        }
        if self.ipv6.valid {
            x[off..off + ipv6_t::size()] |= self.ipv6.to_bitvec();
            off += ipv6_t::size();
        }
        x
    }
    fn dump(&self) -> String {
        format!(
            "{}: {}\n{}: {}", "ethernet".blue(), self.ethernet.dump(), "ipv6".blue(),
            self.ipv6.dump()
        )
    }
}
pub fn ingress_apply(
    hdr: &mut headers_t,
    ingress: &mut ingress_metadata_t,
    egress: &mut egress_metadata_t,
    router: &p4rs::table::Table<
        1usize,
        std::sync::Arc<
            dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
        >,
    >,
) {
    let matches = router.match_selector(&[p4rs::bitvec_to_biguint(&hdr.ipv6.dst).value]);
    if matches.len() > 0 {
        softnpu_provider::control_table_hit!(|| "ingress_table_router");
        (matches[0].action)(hdr, ingress, egress)
    } else {
        softnpu_provider::control_table_miss!(|| "ingress_table_router");
        ingress_action_drop(hdr, ingress, egress);
    }
}
pub fn ingress_action_forward(
    hdr: &mut headers_t,
    ingress: &mut ingress_metadata_t,
    egress: &mut egress_metadata_t,
    port: BitVec<u8, Msb0>,
) {
    let dump = format!("port={}", port,);
    softnpu_provider::action!(|| (& dump));
    egress.port = port.to_owned().clone();
}
pub fn ingress_action_drop(
    hdr: &mut headers_t,
    ingress: &mut ingress_metadata_t,
    egress: &mut egress_metadata_t,
) {
    let dump = format!("",);
    softnpu_provider::action!(|| (& dump));
}
pub fn ingress_router() -> p4rs::table::Table<
    1usize,
    std::sync::Arc<
        dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
    >,
> {
    let mut router_table: p4rs::table::Table<
        1usize,
        std::sync::Arc<
            dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
        >,
    > = p4rs::table::Table::<
        1usize,
        std::sync::Arc<
            dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
        >,
    >::new();
    let action: std::sync::Arc<
        dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
    > = std::sync::Arc::new(|hdr, ingress, egress| {
        ingress_action_forward(
            hdr,
            ingress,
            egress,
            {
                let mut x = bitvec![mut u8, Msb0; 0; 16usize];
                x.store_le(0u128);
                x
            },
        );
    });
    router_table
        .entries
        .insert(p4rs::table::TableEntry::<
            1usize,
            std::sync::Arc<
                dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
            >,
        > {
            key: [
                p4rs::table::Key::Lpm(p4rs::table::Prefix {
                    addr: bitvec_to_ip6addr(
                        &({
                            let mut x = bitvec![mut u8, Msb0; 0; 128usize];
                            x.store_le(336295007452137374271389969406947753984u128);
                            x
                        }
                            & {
                                let mut x = bitvec![mut u8, Msb0; 0; 128usize];
                                x.store_le(340282346638528859811704183484516925440u128);
                                x
                            }),
                    ),
                    len: 24u8,
                }),
            ],
            priority: 0,
            name: "your name here".into(),
            action,
            action_id: String::new(),
            parameter_data: Vec::new(),
        });
    let action: std::sync::Arc<
        dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
    > = std::sync::Arc::new(|hdr, ingress, egress| {
        ingress_action_forward(
            hdr,
            ingress,
            egress,
            {
                let mut x = bitvec![mut u8, Msb0; 0; 16usize];
                x.store_le(1u128);
                x
            },
        );
    });
    router_table
        .entries
        .insert(p4rs::table::TableEntry::<
            1usize,
            std::sync::Arc<
                dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
            >,
        > {
            key: [
                p4rs::table::Key::Lpm(p4rs::table::Prefix {
                    addr: bitvec_to_ip6addr(
                        &({
                            let mut x = bitvec![mut u8, Msb0; 0; 128usize];
                            x.store_le(336295331970691032698116752562968330240u128);
                            x
                        }
                            & {
                                let mut x = bitvec![mut u8, Msb0; 0; 128usize];
                                x.store_le(340282346638528859811704183484516925440u128);
                                x
                            }),
                    ),
                    len: 24u8,
                }),
            ],
            priority: 0,
            name: "your name here".into(),
            action,
            action_id: String::new(),
            parameter_data: Vec::new(),
        });
    router_table
}
pub fn egress_apply(
    hdr: &mut headers_t,
    ingress: &mut ingress_metadata_t,
    egress: &mut egress_metadata_t,
) {}
pub fn parse_start(
    pkt: &mut packet_in,
    headers: &mut headers_t,
    ingress: &mut ingress_metadata_t,
) -> bool {
    pkt.extract(&mut headers.ethernet);
    pkt.extract(&mut headers.ipv6);
    return true;
}
pub struct main_pipeline {
    pub ingress_router: p4rs::table::Table<
        1usize,
        std::sync::Arc<
            dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
        >,
    >,
    pub parse: fn(
        pkt: &mut packet_in,
        headers: &mut headers_t,
        ingress: &mut ingress_metadata_t,
    ) -> bool,
    pub ingress: fn(
        hdr: &mut headers_t,
        ingress: &mut ingress_metadata_t,
        egress: &mut egress_metadata_t,
        router: &p4rs::table::Table<
            1usize,
            std::sync::Arc<
                dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
            >,
        >,
    ),
    pub egress: fn(
        hdr: &mut headers_t,
        ingress: &mut ingress_metadata_t,
        egress: &mut egress_metadata_t,
    ),
    radix: u16,
}
impl main_pipeline {
    pub fn new(radix: u16) -> Self {
        usdt::register_probes().unwrap();
        Self {
            ingress_router: ingress_router(),
            parse: parse_start,
            ingress: ingress_apply,
            egress: egress_apply,
            radix,
        }
    }
    fn process_packet_headers<'a>(
        &mut self,
        port: u16,
        pkt: &mut packet_in<'a>,
    ) -> Vec<(headers_t, u16)> {
        let mut parsed = headers_t::default();
        let mut ingress_metadata = ingress_metadata_t {
            port: {
                let mut x = bitvec![mut u8, Msb0; 0; 16];
                x.store_le(port);
                x
            },
            ..Default::default()
        };
        let mut egress_metadata = egress_metadata_t::default();
        let accept = (self.parse)(pkt, &mut parsed, &mut ingress_metadata);
        if !accept {
            softnpu_provider::parser_dropped!(|| ());
            return Vec::new();
        }
        let dump = format!("\n{}", parsed.dump());
        softnpu_provider::parser_accepted!(|| (& dump));
        let parsed_size = parsed.valid_header_size() >> 3;
        (self
            .ingress)(
            &mut parsed,
            &mut ingress_metadata,
            &mut egress_metadata,
            &self.ingress_router,
        );
        let ports = if egress_metadata.broadcast {
            let mut ports = Vec::new();
            for p in 0..self.radix {
                if p == port {
                    continue;
                }
                ports.push(p);
            }
            ports
        } else {
            if egress_metadata.port.is_empty() || egress_metadata.drop {
                Vec::new()
            } else {
                vec![egress_metadata.port.load_le()]
            }
        };
        let dump = parsed.dump();
        if ports.is_empty() {
            softnpu_provider::ingress_dropped!(|| (& dump));
            return Vec::new();
        }
        let dump = format!("\n{}", parsed.dump());
        softnpu_provider::ingress_accepted!(|| (& dump));
        let mut result = Vec::new();
        for eport in ports {
            let mut egm = egress_metadata.clone();
            let mut parsed_ = parsed.clone();
            egm.port = {
                let mut x = bitvec![mut u8, Msb0; 0; 16];
                x.store_le(eport);
                x
            };
            (self.egress)(&mut parsed_, &mut ingress_metadata, &mut egm);
            if egm.drop {
                continue;
            }
            result.push((parsed_, eport))
        }
        result
    }
    pub fn add_ingress_router_entry<'a>(
        &mut self,
        action_id: &str,
        keyset_data: &'a [u8],
        parameter_data: &'a [u8],
        priority: u32,
    ) {
        let key = [p4rs::extract_lpm_key(keyset_data, 0usize, 16usize)];
        match action_id {
            "drop" => {
                let action: std::sync::Arc<
                    dyn Fn(
                        &mut headers_t,
                        &mut ingress_metadata_t,
                        &mut egress_metadata_t,
                    ),
                > = std::sync::Arc::new(move |hdr, ingress, egress| {
                    ingress_action_drop(hdr, ingress, egress)
                });
                self.ingress_router
                    .entries
                    .insert(p4rs::table::TableEntry::<
                        1usize,
                        std::sync::Arc<
                            dyn Fn(
                                &mut headers_t,
                                &mut ingress_metadata_t,
                                &mut egress_metadata_t,
                            ),
                        >,
                    > {
                        key,
                        priority,
                        name: "your name here".into(),
                        action,
                        action_id: "drop".to_owned(),
                        parameter_data: parameter_data.to_owned(),
                    });
            }
            "forward" => {
                let port = p4rs::extract_bit_action_parameter(
                    parameter_data,
                    0usize,
                    16usize,
                );
                let action: std::sync::Arc<
                    dyn Fn(
                        &mut headers_t,
                        &mut ingress_metadata_t,
                        &mut egress_metadata_t,
                    ),
                > = std::sync::Arc::new(move |hdr, ingress, egress| {
                    ingress_action_forward(hdr, ingress, egress, port.clone())
                });
                self.ingress_router
                    .entries
                    .insert(p4rs::table::TableEntry::<
                        1usize,
                        std::sync::Arc<
                            dyn Fn(
                                &mut headers_t,
                                &mut ingress_metadata_t,
                                &mut egress_metadata_t,
                            ),
                        >,
                    > {
                        key,
                        priority,
                        name: "your name here".into(),
                        action,
                        action_id: "forward".to_owned(),
                        parameter_data: parameter_data.to_owned(),
                    });
            }
            x => panic!("unknown {} action id {}", "ingress", x),
        }
    }
    pub fn remove_ingress_router_entry<'a>(&mut self, keyset_data: &'a [u8]) {
        let key = [p4rs::extract_lpm_key(keyset_data, 0usize, 16usize)];
        let action: std::sync::Arc<
            dyn Fn(&mut headers_t, &mut ingress_metadata_t, &mut egress_metadata_t),
        > = std::sync::Arc::new(move |hdr, ingress, egress| {});
        self.ingress_router
            .entries
            .remove(
                &p4rs::table::TableEntry::<
                    1usize,
                    std::sync::Arc<
                        dyn Fn(
                            &mut headers_t,
                            &mut ingress_metadata_t,
                            &mut egress_metadata_t,
                        ),
                    >,
                > {
                    key,
                    priority: 0,
                    name: "your name here".into(),
                    action,
                    action_id: String::new(),
                    parameter_data: Vec::new(),
                },
            );
    }
    pub fn get_ingress_router_entries(&self) -> Vec<p4rs::TableEntry> {
        let mut result = Vec::new();
        for e in &self.ingress_router.entries {
            let mut keyset_data = Vec::new();
            for k in &e.key {
                keyset_data.extend_from_slice(&k.to_bytes());
            }
            let x = p4rs::TableEntry {
                action_id: e.action_id.clone(),
                keyset_data,
                parameter_data: e.parameter_data.clone(),
            };
            result.push(x);
        }
        result
    }
}
impl p4rs::Pipeline for main_pipeline {
    fn process_packet<'a>(
        &mut self,
        port: u16,
        pkt: &mut packet_in<'a>,
    ) -> Vec<(packet_out<'a>, u16)> {
        let mut parsed = headers_t::default();
        let mut ingress_metadata = ingress_metadata_t {
            port: {
                let mut x = bitvec![mut u8, Msb0; 0; 16];
                x.store_le(port);
                x
            },
            ..Default::default()
        };
        let mut egress_metadata = egress_metadata_t::default();
        let accept = (self.parse)(pkt, &mut parsed, &mut ingress_metadata);
        if !accept {
            softnpu_provider::parser_dropped!(|| ());
            return Vec::new();
        }
        let dump = format!("\n{}", parsed.dump());
        softnpu_provider::parser_accepted!(|| (& dump));
        let parsed_size = parsed.valid_header_size() >> 3;
        (self
            .ingress)(
            &mut parsed,
            &mut ingress_metadata,
            &mut egress_metadata,
            &self.ingress_router,
        );
        let ports = if egress_metadata.broadcast {
            let mut ports = Vec::new();
            for p in 0..self.radix {
                if p == port {
                    continue;
                }
                ports.push(p);
            }
            ports
        } else {
            if egress_metadata.port.is_empty() || egress_metadata.drop {
                Vec::new()
            } else {
                vec![egress_metadata.port.load_le()]
            }
        };
        let dump = parsed.dump();
        if ports.is_empty() {
            softnpu_provider::ingress_dropped!(|| (& dump));
            return Vec::new();
        }
        let dump = format!("\n{}", parsed.dump());
        softnpu_provider::ingress_accepted!(|| (& dump));
        let mut result = Vec::new();
        for eport in ports {
            let mut egm = egress_metadata.clone();
            let mut parsed_ = parsed.clone();
            egm.port = {
                let mut x = bitvec![mut u8, Msb0; 0; 16];
                x.store_le(eport);
                x
            };
            (self.egress)(&mut parsed_, &mut ingress_metadata, &mut egm);
            if egm.drop {
                continue;
            }
            let bv = parsed_.to_bitvec();
            let buf = bv.as_raw_slice();
            let out = packet_out {
                header_data: buf.to_owned(),
                payload_data: &pkt.data[parsed_size..],
            };
            result.push((out, eport))
        }
        result
    }
    fn add_table_entry(
        &mut self,
        table_id: &str,
        action_id: &str,
        keyset_data: &[u8],
        parameter_data: &[u8],
        priority: u32,
    ) {
        match table_id {
            "ingress.router" => {
                self.add_ingress_router_entry(
                    action_id,
                    keyset_data,
                    parameter_data,
                    priority,
                )
            }
            x => println!("add table entry: unknown table id {}, ignoring", x),
        }
    }
    fn remove_table_entry(&mut self, table_id: &str, keyset_data: &[u8]) {
        match table_id {
            "ingress.router" => self.remove_ingress_router_entry(keyset_data),
            x => println!("remove table entry: unknown table id {}, ignoring", x),
        }
    }
    fn get_table_entries(&self, table_id: &str) -> Option<Vec<p4rs::TableEntry>> {
        match table_id {
            "ingress.router" => Some(self.get_ingress_router_entries()),
            x => None,
        }
    }
    fn get_table_ids(&self) -> Vec<&str> {
        vec!["ingress.router"]
    }
}
unsafe impl Send for main_pipeline {}
#[unsafe(no_mangle)]
pub extern "C" fn _main_pipeline_create(radix: u16) -> *mut std::ffi::c_void {
    let pipeline = main_pipeline::new(radix);
    let boxpipe: Box<dyn p4rs::Pipeline> = Box::new(pipeline);
    Box::into_raw(boxpipe) as *mut std::ffi::c_void
}
