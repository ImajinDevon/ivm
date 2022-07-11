pub trait SecurityGuard {
    fn should_allow(&self, req: &OperationRequest) -> bool;
}

/// An external call guard.
///
/// See [InvertedExtcGuard] for an inverted implementation.
pub struct ExtcGuard {
    call_id: usize,
}

impl ExtcGuard {
    pub fn new(call_id: usize) -> Self {
        Self { call_id }
    }
}

impl SecurityGuard for ExtcGuard {
    fn should_allow(&self, req: &OperationRequest) -> bool {
        let OperationRequest::ExternCall(call_id) = req;
        *call_id != self.call_id
    }
}

/// An inverted external call guard.
///
/// See [ExtcGuard] for a non-inverted implementation.
#[derive(Default)]
pub struct InvertedExtcGuard {
    exceptions: Vec<usize>,
}

impl InvertedExtcGuard {
    pub fn new(exceptions: Vec<usize>) -> Self {
        Self { exceptions }
    }
}

impl SecurityGuard for InvertedExtcGuard {
    fn should_allow(&self, req: &OperationRequest) -> bool {
        let OperationRequest::ExternCall(call_id) = req;
        self.exceptions.contains(call_id)
    }
}

/// Created, then sent to the [SecurityManager] when the VM requests to perform an operation.
pub enum OperationRequest {
    /// During execution, within a [crate::VmInstance], an extern call was encountered.
    ExternCall(usize),
}

/// Provides protection against unwanted operations, such as extern calls, and (soon to be) more.
///
/// Before the VM performs an operation, a request will be sent to the SecurityManger, (if present),
/// where it will decide if this operation should be performed.
///
/// # Examples
/// ```
/// use ivm_vm::security::{ExtcGuard, OperationRequest, SecurityManager};
///
/// let mut security_manager = SecurityManager::new();
/// security_manager.add_extc_guard(ExtcGuard::new(1234));
///
/// assert!(!security_manager.is_allowed(OperationRequest::ExternCall(1234)));
/// ```
#[derive(Default)]
pub struct SecurityManager {
    guards: Vec<Box<dyn SecurityGuard>>,
}

impl SecurityManager {
    pub fn add_guard(&mut self, guard: Box<dyn SecurityGuard>) {
        self.guards.push(guard);
    }

    pub fn add_extc_guard(&mut self, guard: ExtcGuard) {
        self.guards.push(Box::new(guard));
    }

    /// Get if the provided [OperationRequest] is allowed to be executed.
    pub fn is_allowed(&self, operation: OperationRequest) -> bool {
        self.guards.iter().all(|g| g.should_allow(&operation))
    }

    /// Creates an empty SecurityManager.
    ///
    /// Equivalent to [SecurityManager::default].
    pub fn new() -> Self {
        Self::default()
    }
}
