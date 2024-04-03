use std::{
    sync::mpsc::{self, Receiver},
    time::Duration,
};

pub fn test_channels() {
    let (transmitter, receiver) = mpsc::channel::<u8>();
    drop(receiver);
    let send_result = transmitter.send(100);

    println!("Send status: {:?}", send_result.is_ok());
}

pub fn test_channels02() {
    let (transmitter, receiver) = mpsc::channel::<u8>();

    let send_result = transmitter.send(100);

    println!("Send status: {:?}", send_result.is_ok());

    receiver.recv();
    transmitter.send(152);
}

pub fn test_channels03() {
    let (transmitter, receiver) = mpsc::channel::<u8>();

    receiver.recv_timeout(Duration::from_millis(300));

    let send_result = transmitter.send(100);

    println!("Send status: {:?}", send_result.is_ok());

    transmitter.send(152);
}

fn main() {
    test_channels();
    test_channels02();
    test_channels03();
}
