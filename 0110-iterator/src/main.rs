#![allow(dead_code)]

struct TemperaturSensor {
    config_value: i32, // Imagine some configuration data about the sensor
}

impl TemperaturSensor {
    fn new() -> Self {
        Self { config_value: 42 }
    }
}

impl Iterator for TemperaturSensor {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(rand::random_range(10..=40))
    }
}

fn main() {
    let temperatures = vec![18, 21, 24, 27, 30];
    let mut alerts = Vec::new();
    for &c in &temperatures {
        if c > 24 {
            alerts.push(c * 9 / 5 + 32);
            if alerts.len() == 2 {
                break;
            }
        }
    }

    println!("{:?}", alerts);

    let mut readings = temperatures.iter();
    println!("{:?}", readings.next());
    println!("{:?}", readings.next());

    for c in TemperaturSensor::new()
        .skip(5)
        .take(10)
        .filter(|&x| x > 24)
        .map(|x| x * 9 / 5 + 32)
    {
        println!("Temperature reading: {}", c);
    }

    let filtered_farenheit: Vec<_> = TemperaturSensor::new()
        .skip(5)
        .take(10)
        .filter(|&x| x > 24)
        .map(|x| x * 9 / 5 + 32)
        .collect();
    println!("{:?}", filtered_farenheit.iter().sum::<i32>());

    struct Customer {
        name: String,
        age: u32,
    }
    let customers = vec![
        Customer { name: "Alice".to_string(), age: 30 },
        Customer { name: "Bob".to_string(), age: 25 },
    ];

    for c in customers.into_iter() {
        println!("Customer: {}, Age: {}", c.name, c.age);
    }
}
