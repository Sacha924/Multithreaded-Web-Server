use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Self { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            // On va avoir un pb car on veut un serveur qui parallélize les requete mais on peut pasappeler thread spawn car on a pas le code qu on peut mettre à l'intérieur
            //Le moyen de contourner ca c d instancier les thread avec le worker.
            workers.push(Worker::new(id));  
            //Notre constructeur prend un paramètre et va nous retourner une structure qui contient 2 params
        }
        Self { workers  }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}