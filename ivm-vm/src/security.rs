use crate::{ExecutionContext, ExternMap, VmInstance};

#[derive(Clone)]
pub enum IllegalOperationHandleMethod {
    Panic,
    SilentFail,
}

/// An extern map which checks incoming calls, ensuring that they do not match any of the guards
/// contained.
///
/// If one of these guards match the incoming call, this GuardedExternMap will act according to the
/// inner [IllegalOperationHandleMethod].
pub struct GuardedExternMap<'a> {
    inner: &'a mut dyn ExternMap,
    guards: Vec<usize>,
    inverted: bool,
    handle_method: IllegalOperationHandleMethod,
}

impl<'a> GuardedExternMap<'a> {
    pub fn add_guard(&mut self, guard: usize) {
        self.guards.push(guard);
    }

    pub fn new(
        inner: &'a mut dyn ExternMap,
        guards: Vec<usize>,
        inverted: bool,
        handle_method: IllegalOperationHandleMethod,
    ) -> Self {
        Self {
            inner,
            guards,
            inverted,
            handle_method,
        }
    }

    pub fn empty(
        inner: &'a mut dyn ExternMap,
        inverted: bool,
        handle_method: IllegalOperationHandleMethod,
    ) -> Self {
        Self::new(inner, Vec::new(), inverted, handle_method)
    }
}

impl ExternMap for GuardedExternMap<'_> {
    fn handle(&mut self, ctx: &mut ExecutionContext, call_id: usize, vm: &mut VmInstance) {
        if !self.guards.contains(&call_id) ^ self.inverted {
            self.inner.handle(ctx, call_id, vm);
            return;
        }

        match self.handle_method {
            IllegalOperationHandleMethod::Panic => {
                panic!(
                    "encountered illegal extern call ~ call id {call_id}, @ execution index {}",
                    vm.execution_index
                )
            }
            IllegalOperationHandleMethod::SilentFail => (),
        }
    }
}
