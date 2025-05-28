pub mod activity;
pub mod pam;

// There are two use cases for the activity:
//   1. The user starts the activity and then stops it later.
//   2. The user records an activity that was done in the past.
// How to model the two use cases in the domain with entities?

/// Represents a list of activities.
/// 
/// It is used to record activities that the user did during his working day.
pub struct ActivitiesList;

/// Represents a tracker for activities.
/// 
/// It is used to track an activity that the user is currently doing. Supports
/// starting, stopping, suspending, and resuming activities.
/// 
/// The ActivityTracker is recording the activity after it is finished in the
/// ActivitiesList.
/// 
/// TODO The `ActivityTracker` will be implemented later.
pub struct ActivityTracker;
