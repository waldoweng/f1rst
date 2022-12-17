use log::{Log, info};
use std::sync::atomic::{AtomicI64, Ordering};

pub type RunFn = fn(T);

pub type ScenarioFn = fn(T) -> RunFn;

// T is a type passed to Scenario functions to manage test state and support formatted test logs.
// A Test ends when its Scenario function returns or calls any of the methods FailNow, Fatal, FatalF.
pub struct T {
    // "iteration " + iteration number or "setup"
    pub Iteration: String,
    pub Scenario: String,
    Failed: AtomicI64,
    TeardownFailed: AtomicI64,
    Require: bool,
    TeardownStack: bool,
    TearingDown: bool,
}

impl T {
    pub fn new(iter: &str, scenario_name: &str) -> Self {
        Self {
            Iteration: String::from(iter),
            Scenario: String::from(scenario_name),
            Failed: AtomicI64::new(0),
            TeardownFailed: AtomicI64::new(0),
            Require: false,
            TeardownStack: false,
            TearingDown: false,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.Scenario
    }
}

impl T {
    // FailNow marks the function as having failed and stops its execution by calling runtime.Goexit (which then runs all deferred calls in the current goroutine).
    // Execution will continue at the next Scenario iteration. FailNow must be called from the goroutine running the Scenario,
    // not from other goroutines created during the Scenario. Calling FailNow does not stop those other goroutines.
    // TODO
    pub fn fail_now(&self) {
        if self.TearingDown {
            self.TeardownFailed.fetch_add(1, Ordering::SeqCst);
        } else {
            self.Failed.fetch_add(1, Ordering::SeqCst);
        }
        std::process::exit(1)
    }

    // Fail marks the function as having failed but continues execution.
    pub fn fail(self) {
        if self.TearingDown {
            self.TeardownFailed.fetch_add(1, Ordering::SeqCst);
        } else {
            self.Failed.fetch_add(1, Ordering::SeqCst);
        }
    }
}

impl T {
    pub fn errorf(&self, format :&str, args ...) {
        info!(format, )
    }
}
