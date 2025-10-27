mod router;

use p4rs::{packet_in, Pipeline};
use router::main_pipeline;
use std::net::Ipv6Addr;
use bitvec::prelude::*;

fn construct_lpm_keyset(addr_bytes: &[u8], prefix_len: u8) -> Vec<u8> {
    let mut keyset_data = Vec::new();
    keyset_data.extend_from_slice(addr_bytes);
    keyset_data.push(prefix_len);
    keyset_data
}

fn main() {
    // 1. Create a new pipeline
    let mut pipeline = unsafe { Box::from_raw(router::_main_pipeline_create(16) as *mut router::main_pipeline) as Box<dyn p4rs::Pipeline> };

    // 2. Add a route for 2001:db8::1/128 to port 1
    let dest_addr = "2001:db8::1".parse::<Ipv6Addr>().unwrap();
    let addr_bytes = dest_addr.octets();
    let keyset_data = construct_lpm_keyset(&addr_bytes, 128);
    let parameter_data = 1u16.to_le_bytes().to_vec();

    pipeline.add_table_entry(
        "ingress.router",
        "forward",
        &keyset_data,
        &parameter_data,
        0,
    );

    // 3. Create a test packet (raw bytes) with destination 2001:db8::1
    let raw_packet_data: Vec<u8> = vec![
        // Ethernet Header
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // Destination MAC
        0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // Source MAC
        0x86, 0xDD,                         // EtherType (IPv6)

        // IPv6 Header
        0x60, 0x00, 0x00, 0x00,             // Version, Traffic Class, Flow Label
        0x00, 0x00,                         // Payload Length
        0x00,                               // Next Header
        0x40,                               // Hop Limit
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Source IP (first 8 bytes)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Source IP (last 8 bytes)
        0x20, 0x01, 0x0D, 0xB8, 0x00, 0x00, 0x00, 0x00, // Destination IP (first 8 bytes) - 2001:db8::
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01  // Destination IP (last 8 bytes) - ::1
    ];

    let mut pkt_in = packet_in {
        data: &raw_packet_data,
        index: 0,
    };

    // 4. Process the packet
    let result = pipeline.process_packet(0, &mut pkt_in);

    // 5. Check the output
    assert_eq!(result.len(), 1);
    let (_out_pkt, out_port) = &result[0];
    assert_eq!(*out_port, 1);

    println!("Packet routed successfully to port {}!", out_port);
}
