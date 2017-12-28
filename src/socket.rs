use ws;
use std::sync::Mutex;
use std::thread;
use math::Vec3;

lazy_static! {
    pub static ref CLIENTS: Mutex<Vec<ws::Sender>> = Mutex::new(Vec::new());
}

pub fn listen() {
    thread::spawn(move || {
        ws::listen("0.0.0.0:3012", |out| {
            CLIENTS.lock().unwrap().push(out);

            move |_msg| Ok(())
        }).unwrap();
    });
}

pub fn send(msg: &str) -> ws::Result<()> {
    for client in CLIENTS.lock().unwrap().iter() {
        client.send(msg)?;
    }

    Ok(())
}

pub fn send_distance(distance: Option<f64>) -> ws::Result<()> {
    let msg = json!({
        "type": "distance",
        "distance": distance
    });
    send(&msg.to_string())
}

pub fn send_acceleration(accel: Vec3) -> ws::Result<()> {
    let msg = json!({
        "type": "acceleration",
        "x": accel[0],
        "y": accel[1],
        "z": accel[2]
    });
    send(&msg.to_string())
}
