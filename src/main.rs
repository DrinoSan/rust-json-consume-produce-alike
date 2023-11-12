use std::io;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use std::{
    fs,
    io::{Read, Write},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use rand::Rng;

use std::f64::consts::PI;

mod Objects;
use request::{Calculation, Shape};
use Objects::request;

fn write_json(tx: Sender<i32>, rx_producer: Receiver<i32>) -> std::io::Result<()> {
    loop {
        thread::sleep(Duration::from_millis(1000));
        match rx_producer.try_recv() {
            Ok(value) => {
                let mut rng = rand::thread_rng();
                let calculation = if rng.gen::<bool>() {
                    Calculation::Area
                } else {
                    Calculation::Perimeter
                };

                let shape: Shape = if rng.gen::<bool>() {
                    Shape::Circle {
                        radius: rng.gen_range(1.0..10.0),
                    }
                } else {
                    Shape::Rectangle {
                        length: rng.gen_range(1.0..10.0),
                        width: rng.gen_range(1.0..10.0),
                    }
                };

                let request = request::Request { calculation, shape };
                if value == 1 {
                    tx.send(0).unwrap();

                    let mut file = fs::File::create("calculation.json")?;

                    let st = serde_json::to_string_pretty(&request).unwrap();
                    file.write(st.as_bytes())?;
                    println!("Producer wrote file");

                    // 1 for written
                    tx.send(1).unwrap();
                }
            }
            Err(TryRecvError::Empty) => {
                println!("Producer Waiting...");
                // You might want to add a short delay here to avoid busy-waiting
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(TryRecvError::Disconnected) => {
                return Err(io::Error::new(io::ErrorKind::Other, "Channel disconnected"));
            }
        }
    }
}

fn read_json(rx: Receiver<i32>, tx_producer: Sender<i32>) -> std::io::Result<()> {
    // 1 for written
    tx_producer.send(1).unwrap();

    loop {
        thread::sleep(Duration::from_millis(1000));
        match rx.try_recv() {
            Ok(value) => {
                if value == 1 {
                    tx_producer.send(0).unwrap();
                    let mut file = fs::File::open("calculation.json")?;

                    // let mut req_buffer: Vec<u8> = vec![];
                    // let bytes_read = file.read_to_end(&mut req_buffer)?;

                    let mut req: String = Default::default();
                    let _ = file.read_to_string(&mut req)?;

                    let calc: request::Request = serde_json::from_str(&req).unwrap();
                    let result = match (calc.calculation, calc.shape) {
                        (Calculation::Perimeter, Shape::Circle { radius }) => PI * 2.0 * radius,
                        (Calculation::Perimeter, Shape::Rectangle { length, width }) => {
                            2.0 * width + 2.0 * length
                        }
                        (Calculation::Area, Shape::Circle { radius }) => {
                            f64::powf(radius, 2.0) * PI
                        }
                        (Calculation::Area, Shape::Rectangle { length, width }) => length * width,
                    };

                    println!(
                        "Consumer CALCULATION of {}",
                        serde_json::to_string_pretty(&req).unwrap()
                    );
                    println!("      Consumer Has result: {}", result);
                    tx_producer.send(1).unwrap();
                }
            }
            Err(TryRecvError::Empty) => {
                println!("Consumer Waiting...");
                // You might want to add a short delay here to avoid busy-waiting
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(TryRecvError::Disconnected) => {
                return Err(io::Error::new(io::ErrorKind::Other, "Channel disconnected"));
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("Calculation for ever");

    let (tx, rx) = mpsc::channel::<i32>();
    let (tx_producer, rx_producer) = mpsc::channel::<i32>();
    let writer = thread::spawn(|| write_json(tx, rx_producer));
    let reader = thread::spawn(|| read_json(rx, tx_producer));

    writer.join().unwrap();
    reader.join().unwrap();

    Ok(())
}
