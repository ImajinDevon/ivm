pub struct LinearProcess<'a> {
    process_name: &'a str,
    step: usize,
    n_steps: usize,
}

impl<'a> LinearProcess<'a> {
    pub fn log(&self, message: &str) {
        println!("{}@info: {message}", self.process_name);
    }

    fn step(&mut self, description: &str) {
        print!(
            "{} ({}/{}): {description}...",
            self.process_name, self.step, self.n_steps
        );
        self.step += 1;
    }

    pub fn step_task<F, T>(&mut self, description: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.step(description);
        let t = f();
        println!("done");
        t
    }

    pub fn step_result<F, T, E>(&mut self, description: &str, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        self.step(description);
        let result = f();
        println!("{}", if result.is_ok() { "done" } else { "failure" });
        result
    }

    pub fn new(process_name: &'a str, n_steps: usize) -> Self {
        Self {
            process_name,
            step: 1,
            n_steps,
        }
    }
}
