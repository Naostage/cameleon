use cameleon::{
    genapi::DefaultGenApiCtxt,
    u3v::{enumerate_cameras, ControlHandle, StreamHandle},
    Camera,
};

fn main() {
    // Enumerates cameras connected to the host.
    let mut cameras: Vec<Camera<ControlHandle, StreamHandle, DefaultGenApiCtxt>> =
        enumerate_cameras().unwrap();

    if cameras.is_empty() {
        println!("no camera found!");
        return;
    }

    let mut camera = cameras.pop().unwrap();

    // Open the camera.
    camera.open().unwrap();
    // Load `GenApi` context.
    camera.load_context().unwrap();

    // Start streaming.
    let payload_rx = camera.start_streaming(10).unwrap();
    let mut payload_count = 0;
    while payload_count < 10 {
        match payload_rx.try_recv() {
            Ok(payload) => {
                println!(
                    "payload received! block_id: {:?}, timestamp: {:?}",
                    payload.id(),
                    payload.timestamp()
                );
                if let Some(image_info) = payload.image_info() {
                    println!("{:?}\n", image_info);
                }
                payload_count += 1;

                // Send back payload to streaming loop to reuse the buffer.
                payload_rx.send_back(payload);
            }
            Err(_err) => {
                continue;
            }
        }
    }

    camera.close().ok();
}
