use std::fmt::Display;

/// A process logger, with linear steps.
/// # Examples
/// ```
/// use ivm_common::cli::LinearLogger;
///
/// let logger = LinearLogger::new("my_process", 0);
/// logger.log("Hello, world!");
/// ```
pub struct LinearLogger<'a> {
    process_name: &'a str,
    step: usize,
    n_steps: usize,
}

impl<'a> LinearLogger<'a> {
    pub fn log<D>(&self, message: D)
    where
        D: Display,
    {
        println!("{}@info: {message}", self.process_name);
    }

    fn step<D>(&mut self, description: D)
    where
        D: Display,
    {
        print!(
            "{} ({}/{}): {description}...",
            self.process_name, self.step, self.n_steps
        );
        self.step += 1;
    }

    pub fn step_task<F, T, D>(&mut self, description: D, f: F) -> T
    where
        F: FnOnce() -> T,
        D: Display,
    {
        self.step(description);
        let t = f();
        println!("done");
        t
    }

    pub fn step_result<F, T, E, D>(&mut self, description: &str, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
        D: Display,
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
