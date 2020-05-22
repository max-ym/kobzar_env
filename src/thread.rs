use core::time::Duration;
use time::Time;
use crate::path::{LocalPath, Network, InstanceId, Interface};
use crate::{kobzar_env, KobzarEnv, Uid};
use core::ops::Deref;
use crate::rsc::{Variable, Handle};
use alloc::rc::Rc;

/// Priority identifies relative importance of the thread over other one. This helps scheduler to
/// make correct decisions over which threads should be executed next and what time they should
/// run.
pub type Priority = f32;

/// State of the thread.
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

/// Performance policy defines the way CPU time is allocated for given thread.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum PerformancePolicy {
    /// Request best performance possible for given thread.
    Performance,

    /// No special requests.
    Normal,
}

/// Detailed information about how and when to run this task.
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
        kobzar_env().network_mut().allow_run(self)
    }

    /// Request pausing of this thread to prevent further execution until run is requested.
    pub fn request_pause(&mut self) {
        kobzar_env().network_mut().request_pause(self)
    }

    /// Notify thread to cease.
    pub fn request_cease(&mut self) {
        kobzar_env().network_mut().request_cease(self)
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
        kobzar_env().network_mut().brutal_kill(self)
    }

    /// Sleep for at least given duration.
    pub fn sleep(&mut self, duration: Duration) {
        kobzar_env().network_mut().sleep(self, duration)
    }

    /// Try changing the performance policy. Err with most supported policy will be returned if
    /// thread tries to use one it has no permissions for.
    pub fn set_performance_policy(&mut self, policy: PerformancePolicy)
                                  -> Result<(), PerformancePolicy> {
        match kobzar_env().network().set_performance_policy(self, policy) {
            Ok(_) => {
                self.thread.performance = policy;
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    /// Get current performance policy.
    pub fn performance_policy(&self) -> PerformancePolicy {
        self.thread.performance
    }

    /// Get current thread handle.
    pub fn current() -> &'static mut OwnedThread {
        kobzar_env().network().current_thread()
    }
}

/// Publicity defines who can initiate communication with selected thread.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Publicity {
    /// All external threads can initiate communication with selected thread.
    Public,

    /// Only threads from the same or descendant packages can initiate communication.
    Package,

    /// No threads from the same package can initiate communication but descendant threads can.
    Descendant,

    /// No threads can initiate communication with selected thread.
    Private,
}

/// General information about thread in the network.
#[derive(Clone)]
pub struct Thread {
    instance: Rc<InstanceId>,
    state: State,
    publicity: Publicity,
    performance: PerformancePolicy,

    has_powersave_notif: bool,
    has_powersave_disable_notif: bool,
}

impl Thread {
    /// State of the thread when the snapshot was taken.
    pub fn state(&self) -> State {
        self.state
    }
}

impl Handle for Thread {
    fn uid(&self) -> Uid {
        self.instance.uid()
    }
}

impl Variable for Thread {}

/// Information that is required to build a thread.
pub struct ThreadBuilder<'a, 'b> {
    pub local_path: LocalPath<'a>,
    pub ty: Type,
    pub publicity: Publicity,

    pub imp: &'b Interface,
}

pub enum ThreadBuildError {
    /// Creator has no rights to create this type of threads.
    ThreadCreationNotPermitted,

    /// Performance policy requested is not permitted for creator.
    PerformancePolicyNotPermitted {
        most_supported: PerformancePolicy,
    },

    /// Implementation for requested interface was not found.
    NotFound,
}

impl<'a, 'b> ThreadBuilder<'a, 'b> {
    pub fn build(&self) -> Result<OwnedThread, ThreadBuildError> {
        kobzar_env().network_mut().create_thread(self)
    }
}
