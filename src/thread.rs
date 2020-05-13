use core::time::Duration;
use time::Time;
use crate::path::{LocalPath, Network};
use crate::{kobzar_env, KobzarEnv};
use crate::harc::Harc;
use core::ops::Deref;

pub type Priority = f32;

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
pub enum ThreadType {
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

pub struct ThreadBuilder<'a> {
    pub local_path: LocalPath<'a>,
    pub ty: ThreadType,
}

/// Thread that is owned by other thread. Owner can affect thread execution or change some
/// the data associated with thread.
pub struct OwnedThread {
    thread: Harc<Thread>,
}

impl Deref for OwnedThread {
    type Target = Thread;

    fn deref(&self) -> &Self::Target {
        self.thread.deref()
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
}

/// General information about thread in the network.
pub struct Thread {}

pub enum ThreadBuildError {
    /// Owner have no rights to create this type of threads.
    NotPermitted,
}

impl<'a> ThreadBuilder<'a> {
    pub fn build(&self) -> Result<OwnedThread, ThreadBuildError> {
        kobzar_env().network_mut().create_thread(self)
    }

    /// Build thread without getting ownership over it. Only general information will be available
    /// and creator will not be able to influence the created thread.
    pub fn build_unowned(&self) -> Result<Harc<Thread>, ThreadBuildError> {
        unimplemented!()
    }
}
