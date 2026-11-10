#![allow(unused_variables)]

use std::{pin::Pin, task::{Context, Poll}};

async fn read_temperature() -> i32 {
    let reading = ReadTemperature { started: false };
    let temperature = reading.await;
    temperature
}

async fn report() -> String {
    println!("Requesting...");
    let c = read_temperature().await;
    format!("Temperature: {}, in Farenheit {}", c, c * 9 / 5 + 32)
}

struct ReadTemperature {
    started: bool,
}

impl Future for ReadTemperature {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // If poll is called for the first time, mark as started
        // If called for the second time, return the temperature immediately

        if !this.started {
            this.started = true;
            println!("Starting to read temperature...");
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        println!("Temperature read successfully.");
        Poll::Ready(24)
    }
}

#[tokio::main]
async fn main() {
    let block = async {
        let kitchen = report().await;
        let living_room = report().await;
        println!("Kitchen: {}\nLiving Room: {}", kitchen, living_room)
    };
    block.await;
}
