#![allow(dead_code)]

struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

trait Billable {
    fn total(&self) -> f32;
}

trait Pointworthy {
    fn points(&self) -> i32;

    fn get_point_factor(&self) -> f32 {
        0.1
    }
}

impl<T> Pointworthy for T
where
    T: Billable,
{
    fn points(&self) -> i32 {
        (self.total() / self.get_point_factor()) as i32
    }
}

impl<T, const C: usize> Pointworthy for [T; C]
where 
    T: Pointworthy,
{
    fn points(&self) -> i32 {
        self.iter().map(|item| item.points()).sum()
    }
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

impl Billable for () {
    fn total(&self) -> f32 {
        0.0
    }
}

fn create_billable() -> impl Billable {
    ConsultingWork {
        what: String::from("Training"),
        hours: 10.0,
        rate: 50.0,
    }
}

fn create_billable_dyn() -> Box<dyn Billable> {
    if rand::random() {
        Box::new(ConsultingWork {
            what: String::from("Consulting"),
            hours: 5.0,
            rate: 100.0,
        })
    } else {
        Box::new(100.0)
    }
}

fn print_billable(something_billable: &impl Billable) {
    println!("Total: {}", something_billable.total());
}

fn print_billable_dyn(something_billable: &dyn Billable) {
    println!("Total: {}", something_billable.total());
}

fn print_pointworthy_dyn(something_pointworthy: &dyn Pointworthy) {
    println!("Points: {}", something_pointworthy.points());
}

fn main() {
    let something_billable = create_billable();
    print_billable(&something_billable);

    let number_billable: f32 = 100.0;
    print_billable(&number_billable);

    let unit_billable = ();
    print_billable(&unit_billable);

    let something_billable_dyn = create_billable_dyn();
    print_billable_dyn(something_billable_dyn.as_ref());

    let billables: Vec<Box<dyn Billable>> = vec![
        Box::new(ConsultingWork {
            what: String::from("Training"),
            hours: 10.0,
            rate: 50.0,
        }),
        Box::new(100.0),
        Box::new(()),
    ];
    for item in billables {
        print_billable_dyn(item.as_ref());
    }

    let consulting_work_pointworthy = Box::new(ConsultingWork {
        what: String::from("Consulting"),
        hours: 5.0,
        rate: 100.0,
    });
    print_pointworthy_dyn(consulting_work_pointworthy.as_ref());

    // ConsultingWork is Pointworthy via the blanket impl (because it is Billable),
    // so [ConsultingWork; 2] is Pointworthy via the array impl.
    let pointworthy_items = [
        ConsultingWork {
            what: String::from("Training"),
            hours: 10.0,
            rate: 50.0,
        },
        ConsultingWork {
            what: String::from("Consulting"),
            hours: 5.0,
            rate: 100.0,
        },
    ];
    print_pointworthy_dyn(&pointworthy_items);
}
