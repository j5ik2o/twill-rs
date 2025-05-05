/// The enum type representing the commit status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommittedStatus {
  /// Committed.
  Committed,
  /// Uncommitted.
  Uncommitted,
}

impl CommittedStatus {
  /// Returns whether the status is committed or not.
  pub fn is_committed(&self) -> bool {
    match self {
      CommittedStatus::Committed => true,
      CommittedStatus::Uncommitted => false,
    }
  }

  /// Returns whether the status is uncommitted or not.
  pub fn is_uncommitted(&self) -> bool {
    !self.is_committed()
  }

  /// Returns the OR operation result with another CommittedStatus.
  pub fn or(&self, other: &CommittedStatus) -> CommittedStatus {
    if self.is_committed() || other.is_committed() {
      CommittedStatus::Committed
    } else {
      CommittedStatus::Uncommitted
    }
  }
}

impl From<bool> for CommittedStatus {
  fn from(value: bool) -> Self {
    if value {
      CommittedStatus::Committed
    } else {
      CommittedStatus::Uncommitted
    }
  }
}
