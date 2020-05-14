use core::time::Duration;
use time::Time;
use crate::path::{LocalPath, Network, InstanceId};
use crate::{kobzar_env, KobzarEnv, Uid};
use core::ops::Deref;
use crate::rsc::{Variable, Handle};
use alloc::sync::Arc;

pub type Priority = f32;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum State {
    Running,
    Paused,
    Ceased,
    Killed,

    PausedRunRequested,
    RunningPauseRequested,
    RunningCeaseRequested,
    PausedCeaseRequested,
}

/// System notification that thread can subscribe to.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Notification {
    /// Powersave mode turned on.
    Powersave,

    /// Powersave mode turned off.
    PowersaveDisabled,
}

impl Notification {
    /// Subscribe current thread to given notification.
    pub fn subscribe(self) -> Self {
        OwnedThread::current().subscribe_to(self);
        self
    }
}

impl State {
    pub fn is_running(&self) -> bool {
        State::Running == *self
    }

    pub fn is_paused(&self) -> bool {
        State::Paused == *self
    }

    pub fn is_pause_requested(&self) -> bool {
        State::RunningPauseRequested == *self
    }

    pub fn is_run_requested(&self) -> bool {
        State::PausedRunRequested == *self
    }

    pub fn is_killed(&self) -> bool {
        State::Killed == *self
    }

    pub fn is_ceased(&self) -> bool {
        State::Ceased == *self
    }

    pub fn is_cease_requested(&self) -> bool {
        State::PausedCeaseRequested == *self || State::RunningCeaseRequested == *self
    }

    pub fn is_dead(&self) -> bool {
        self.is_killed() || self.is_ceased()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum PerformancePolicy {
    /// Request best performance possible for given thread.
    Performance,

    /// No special requests.
    Normal,
}

#[derive(Clone, PartialEq)]
pub struct TaskDetail {
    /// Estimated time left for execution to end. It may go negative
    /// if task exceeded it's time during execution.
    pub estimate_left: Duration,

    /// Margin for error for task execution time.
    pub margin: Duration,

    /// Priority of this task over others of the same type.
    pub priority: Priority,
}

/// Thread type with associated information. According to the information available scheduler
/// decides time allocation, execution ordering.
#[derive(Clone, PartialEq)]
pub enum Type {
    /// Thread that is executed as task. It has associated estimate time of completion.
    TimerTask(TaskDetail),

    /// Task which purpose is to cache some information that might be necessary for later
    /// use. These tasks will be executed if otherwise the system will be idle.
    /// They might not be executed at all if system tries to save energy or otherwise
    /// is not willing to allocate extra time.
    CachingTask {
        basic: TaskDetail,

        /// Indicator of probability that the cached info will be needed by the system.
        /// The value should be in range from 0.0 to 1.0.
        probability: f32,

        /// Time in the network until which the data should be cached.
        /// If scheduler fails to execute the task until this time the task will
        /// be asked to provide new time, change task type or cease.
        until: Time,
    },

    /// Typical application thread that runs in parallel with other threads.
    Parallel,
}

/// Thread that is owned by other thread. Owner can affect thread execution or change some
/// the data associated with thread.
pub struct OwnedThread {
    thread: Thread,
}

impl Deref for OwnedThread {
    type Target = Thread;

    fn deref(&self) -> &Self::Target {
        &self.thread
    }
}

impl OwnedThread {
    /// Allow execution of this thread.
    pub fn allow_run(&mut self) {
        unimplemented!()
    }

    /// Request pausing of this thread to prevent further execution until run is requested.
    pub fn request_pause(&mut self) {
        unimplemented!()
    }

    /// Notify thread to cease.
    pub fn request_cease(&mut self) {
        unimplemented!()
    }

    /// Kill thread immediately. Thread may be secured from killing. On startup each
    /// thread decides whether it needs guard to prevent killing. Even thread owner cannot
    /// kill this thread. Owned thread can release or re-acquire guard at any time. If thread
    /// is killed any resources owned by the thread will be lost. Any shared resources which
    /// were modified will possibly get corrupted. It's best to use [`request_cease`] instead.
    ///
    /// Err is returned if thread is guarded.
    // TODO verify killing policies for efficiency
    pub unsafe fn brute_kill(&mut self) -> Result<(), ()> {
        unimplemented!()
    }

    /// Sleep for at least given duration.
    pub fn sleep(&mut self, duration: Duration) {
        unimplemented!()
    }

    /// Try changing the performance policy. Err with most supported policy will be returned if
    /// thread tries to use one it has no permissions for.
    pub fn set_performance_policy(&mut self, policy: PerformancePolicy)
                                  -> Result<(), PerformancePolicy> {
        unimplemented!()
    }

    /// Get current performance policy.
    pub fn performance_policy(&self) -> PerformancePolicy {
        unimplemented!()
    }

    pub fn subscribe_to(&mut self, notif: Notification) {
        unimplemented!()
    }

    pub fn unsubscribe_from(&mut self, notif: Notification) {
        unimplemented!()
    }

    pub fn is_subscriber(&self, notif: Notification) -> bool {
        unimplemented!()
    }

    /// Get current thread handle.
    pub fn current() -> Self {
        unimplemented!()
    }
}

/// General information about thread in the network.
#[derive(Clone)]
pub struct Thread {
    instance: Arc<InstanceId>,
    state: State,

    has_powersave_notif: bool,
    has_powersave_disable_notif: bool,
}

impl Thread {
    pub fn thread_state(&self) -> State {
        unimplemented!()
    }
}

impl Handle for Thread {
    fn instance(&self) -> &InstanceId {
        &self.instance
    }
}

impl Variable for Thread {}

pub struct ThreadBuilder<'a> {
    pub local_path: LocalPath<'a>,
    pub ty: Type,
}

pub enum ThreadBuildError {
    /// Creator has no rights to create this type of threads.
    ThreadCreationNotPermitted,

    /// Performance policy requested is not permitted for creator.
    PerformancePolicyNotPermitted {
        most_supported: PerformancePolicy,
    },
}

impl<'a> ThreadBuilder<'a> {
    pub fn build(&self) -> Result<OwnedThread, ThreadBuildError> {
        kobzar_env().network_mut().create_thread(self)
    }

    /// Build thread without getting ownership over it. Only general information will be available
    /// and creator will not be able to influence the created thread.
    pub fn build_unowned(&self) -> Result<Thread, ThreadBuildError> {
        unimplemented!()
    }
}
