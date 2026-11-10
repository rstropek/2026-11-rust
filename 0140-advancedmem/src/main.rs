#![allow(dead_code, unused_imports)]

use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex, RwLock}};

#[derive(Debug, Clone, Default)]
struct Scoreboard {
    completed: Vec<String>
}

struct Reporter {
    source: &'static str,
    board: Rc<RefCell<Scoreboard>> // Inner mutabilty pattern
}

struct Reporter2 {
    source: &'static str,
    board: Arc<RwLock<Scoreboard>> // Inner mutabilty pattern
}

impl Reporter {
    fn report(&self, task: &str) {
        let mut board = self.board.borrow_mut();
        board.completed.push(format!("{}: {}", self.source, task));
    }
}

impl Reporter2 {
    fn report(&self, task: &str) {
        let mut board = self.board.write().unwrap();
        board.completed.push(format!("{}: {}", self.source, task));
    }
}

fn main() {
    {
        let board = Rc::new(Scoreboard::default());
        let quiz_board = Rc::clone(&board);
        let lab_board = Rc::clone(&board);

        println!("{}", Rc::ptr_eq(&quiz_board, &lab_board));
        println!("{}", Rc::strong_count(&board));
    }

    {
        let board = RefCell::new(Scoreboard::default());
        {
            let mut access = board.borrow_mut();
            access.completed.push("ownership".into());
            drop(access);
            let mut access_2 = board.borrow_mut();
            access_2.completed.push("ownership".into());
        }
        {
            let access = board.borrow();
            println!("{:?}", access.completed);
        }
    }

    {
        let shared_board = 
            Rc::new(RefCell::new(Scoreboard::default()));

        let reporter_1 = Reporter {
            source: "quiz",
            board: Rc::clone(&shared_board)
        };
        let reporter_2 = Reporter {
            source: "lab",
            board: Rc::clone(&shared_board)
        };

        reporter_1.report("task 1");
        reporter_2.report("task 2");

        println!("{:?}", shared_board.borrow().completed);
    }

    {
        let shared_board = 
            Arc::new(RwLock::new(Scoreboard::default()));

        let reporter_1 = Reporter2 {
            source: "quiz",
            board: Arc::clone(&shared_board)
        };
        let reporter_2 = Reporter2 {
            source: "lab",
            board: Arc::clone(&shared_board)
        };

        reporter_1.report("task 1");
        reporter_2.report("task 2");

        println!("{:?}", shared_board.read().unwrap().completed);
    }
}
