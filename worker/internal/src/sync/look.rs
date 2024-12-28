use std::time::Duration;

/// Trait representing a distributed lock client.
pub trait Lock {
	/// Acquires a re-entrant lock on the specified `lock_id`. Blocks indefinitely until the lock is acquired.
	///
	/// # Parameters
	///
	/// - `lock_id`: A `String` representing the resource to lock on.
	fn acquire_lock(&self, lock_id: &str);

	/// Acquires a re-entrant lock on the specified `lock_id`. Blocks for the given duration before giving up.
	///
	/// # Parameters
	///
	/// - `lock_id`: A `String` representing the resource to lock on.
	/// - `time_to_try`: The maximum duration to attempt acquiring the lock.
	/// - `unit`: The unit of time for `time_to_try`. This is replaced by `Duration` in Rust.
	///
	/// # Returns
	///
	/// `true` if the lock was successfully acquired, `false` otherwise.
	fn acquire_lock_with_timeout(&self, lock_id: &str, time_to_try: Duration) -> bool;

	/// Acquires a re-entrant lock on the specified `lock_id` with a provided lease time. Blocks for the given duration
	/// before giving up.
	///
	/// # Parameters
	///
	/// - `lock_id`: A `String` representing the resource to lock on.
	/// - `time_to_try`: The maximum duration to attempt acquiring the lock.
	/// - `lease_time`: The duration for which the lock is held.
	/// - `unit`: The unit of time for `time_to_try` and `lease_time`. This is replaced by `Duration` in Rust.
	///
	/// # Returns
	///
	/// `true` if the lock was successfully acquired, `false` otherwise.
	fn acquire_lock_with_lease(
		&self,
		lock_id: &str,
		time_to_try: Duration,
		lease_time: Duration,
	) -> bool;

	/// Releases a previously acquired lock.
	///
	/// # Parameters
	///
	/// - `lock_id`: A `String` representing the resource to lock on.
	fn release_lock(&self, lock_id: &str);

	/// Explicitly cleans up lock resources if releasing the lock does not do so.
	///
	/// # Parameters
	///
	/// - `lock_id`: A `String` representing the resource to clean up.
	fn delete_lock(&self, lock_id: &str);
}
