#![allow(dead_code, unused_variables, unreachable_patterns)]

struct Person {
    first_name: String,
    last_name: String,
}

enum RoomStatus {
    Available,
    Occupied(Person),
    UnderMaintenance,
}

impl RoomStatus {
    fn is_available(&self) -> bool {
        matches!(self, RoomStatus::Available)
    }
}

fn main() {
    let available_room = RoomStatus::Available;
    let occupied_room = RoomStatus::Occupied(Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    });
    let maintenance_room = RoomStatus::UnderMaintenance;

    match &available_room {
        RoomStatus::Available => println!("The room is available."),
        RoomStatus::Occupied(p) => println!("The room is occupied by {}", p.last_name),
        RoomStatus::UnderMaintenance => println!("The room is under maintenance."),
        _ => println!("Unknown room status."),
    }

    if available_room.is_available() {
        println!("The room is available (checked with if).");
    }

    if let RoomStatus::Occupied(person) = &occupied_room {
        println!("The room is occupied by {}", person.last_name);
    }

    // The same with match
    match &occupied_room {
        RoomStatus::Occupied(person) => println!("The room is occupied by {}", person.last_name),
        _ => (),
    }
}
