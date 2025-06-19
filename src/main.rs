use walktermnet::interfaces::get_mac;
use walktermnet::l2::{mac_to_string, receive_frames, send_frame, string_to_mac, Frame};
use walktermnet::l3::{ipv6_string_to_bytes, NetPacket};

use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let iface_name = "enp0s31f6"; // Adjust this to your actual interface

    let src_mac = get_mac(iface_name)?;
    let dst_mac =
        string_to_mac("ff:ff:ff:ff:ff:ff").ok_or("Invalid destination MAC address format")?;

    let src_ip = ipv6_string_to_bytes("0101::f").ok_or("Invalid source IPv6 address")?;
    let dst_ip = ipv6_string_to_bytes("0101::1").ok_or("Invalid destination IPv6 address")?;

    let l3_packet = NetPacket::new(src_ip, dst_ip, b"Hello from L3".to_vec());
    let l3_bytes = l3_packet.to_bytes();

    let mut frame = Frame::new();
    frame.src_mac = src_mac.into();
    frame.dest_mac = dst_mac;
    frame.data = l3_bytes;

    let iface_name_send = iface_name.to_string();
    let send_task = task::spawn(async move { send_frame(&iface_name_send, &frame).await });

    let iface_name_recv = iface_name.to_string();
    let recv_task = task::spawn(async move {
        receive_frames(&iface_name_recv, |frame| {
            println!("Got frame from {}", mac_to_string(&frame.src_mac));
            if let Some(net_packet) = NetPacket::from_bytes(&frame.data) {
                println!(
                    "Payload from {} to {}: {:?}",
                    net_packet.src_ip_to_string(),
                    net_packet.dst_ip_to_string(),
                    String::from_utf8_lossy(&net_packet.data)
                );
            } else {
                println!("Could not parse L3 data");
            }
        })
        .await
    });

    // Wait for both tasks
    send_task.await??;
    recv_task.await??;

    Ok(())
}
