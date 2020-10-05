use std::rc::Rc;

struct DB {
    db_url: String,
}

impl DB {
    fn new(db_url: String) -> DB {
        DB { db_url }
    }
    fn get_user(&self) {}
}

struct Core {
    db: Rc<DB>,
    user: UserService,
}

impl Core {
    fn new(db_url: String) -> Core {
        let db = DB::new(db_url);
        let db_ref = Rc::new(db);
        let user = UserService::new(Rc::clone(&db_ref));
        Core { db: db_ref, user }
    }
}

struct UserService {
    db: Rc<DB>,
}

impl UserService {
    fn new(db: Rc<DB>) -> UserService {
        UserService { db }
    }
    fn work(&self) {
        self.db.get_user();
    }
}

fn main() {
    let core = Core::new("db-url".to_string());
    core.db.get_user();
    core.user.work();
}
