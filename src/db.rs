// TODO: Move bitflags to use `bitflags` crate.
#![allow(non_upper_case_globals)]
use crate::{error, Result};
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::time::SystemTime;

/// Bitflag indicating no action has been taken
pub const NoAction: u32 = 0b00000000;

/// Bitflag indicating an issue has been incorrectly assigned
/// for at least 24h and an appropriate action has been taken
pub const PullRequestCoreDevAuthorIssueNotAssigned24h: u32 = 0b00000010;

/// Bitflag indicating an issue has been incorrectly assigned
/// for at least 72h and an appropriate action has been taken
pub const PullRequestCoreDevAuthorIssueNotAssigned72h: u32 = 0b00000100;

pub enum DbEntryState {
	Delete,
	Update,
	DoNothing,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IssueProjectState {
	Confirmed,
	Unconfirmed,
	Denied,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueProject {
	pub state: IssueProjectState,
	pub actor_login: String,
	pub project_column_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbEntry {
	#[serde(skip)]
	key: Vec<u8>,
	pub actions_taken: u32,
	pub status_failure_ping: Option<SystemTime>,
	pub issue_not_assigned_ping: Option<SystemTime>,
	pub issue_no_project_ping: Option<SystemTime>,
	pub issue_no_project_npings: u64,
	pub issue_confirm_project_ping: Option<SystemTime>,
	pub issue_project: Option<IssueProject>,
	pub last_confirmed_project: Option<IssueProject>,
}

impl DbEntry {
	pub fn new(key: Vec<u8>) -> DbEntry {
		DbEntry {
			key: key,
			actions_taken: NoAction,
			issue_not_assigned_ping: None,
			issue_no_project_ping: None,
			issue_no_project_npings: 0,
			status_failure_ping: None,
			issue_confirm_project_ping: None,
			issue_project: None,
			last_confirmed_project: None,
		}
	}

	pub fn new_or_with_key(db: &DB, k: Vec<u8>) -> DbEntry {
		let mut db_entry = DbEntry::new(k);
		if let Ok(Some(entry)) = db.get_pinned(&db_entry.key).map(|v| {
			v.map(|value| {
				serde_json::from_str::<DbEntry>(
					String::from_utf8(value.to_vec()).unwrap().as_str(),
				)
				.expect("deserialize entry")
			})
		}) {
			db_entry = entry;
		}
		db_entry
	}

	pub fn delete(&self, db: &DB) -> Result<()> {
		db.delete(&self.key).context(error::Db)
	}

	pub fn update(&self, db: &DB) -> Result<()> {
		db.delete(&self.key).context(error::Db)?;
		db.put(
			&self.key,
			serde_json::to_string(self)
				.expect("serialize db entry")
				.as_bytes(),
		)
		.context(error::Db)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bitflags() {
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h
				& PullRequestCoreDevAuthorIssueNotAssigned72h,
			NoAction
		);
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h
				| PullRequestCoreDevAuthorIssueNotAssigned72h,
			0b0000_0110
		);
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h & NoAction,
			NoAction
		);
		assert_eq!(
			PullRequestCoreDevAuthorIssueNotAssigned24h | NoAction,
			PullRequestCoreDevAuthorIssueNotAssigned24h
		);
	}
}
