use super::ActivityCategory;
use super::ActivitySecondaryType;
use super::ActivityStatus;
use super::ActivityType;
use super::ActivityUserActionKind;
use crate::api::prime::prelude::*;

/// Represents an activity within the account, including its metadata and associated user actions.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Activity {
    /// A unique id for the account activity.
    pub id: String,
    /// A reference for orders and transactions, n/a for other category types.
    pub reference_id: String,
    /// The general category of the activity.
    pub category: ActivityCategory,
    /// The type of activity.
    pub r#type: ActivityType,
    /// The secondary type of activity, providing additional classification.
    pub secondary_type: ActivitySecondaryType,
    /// The current status of the activity.
    pub status: ActivityStatus,
    /// Id of the user who created the activity.
    #[serde(default, with = "maybe_str")]
    pub created_by: Option<Uuid>,
    /// Title of the activity.
    pub title: String,
    /// Detailed description of the activity.
    pub description: String,
    /// Actions related to the activity by different users.
    pub user_actions: Vec<ActivityUserAction>,
    /// Metadata associated with the transactions of the activity.
    pub transactions_metadata: Option<ActivityMetadata>,
    /// Metadata associated with the account of the activity.
    pub account_metadata: Option<ActivityMetadata>,
    // /// Metadata associated with the orders of the activity. (Not documented in the API).
    // pub orders_metadata: Option<_>,
    /// List of symbols/currencies included in the activity.
    pub symbols: Vec<String>,
    /// Time when the activity was created.
    pub created_at: DtCoinbasePrime,
    /// Time of the latest status update of the account activity.
    pub updated_at: DtCoinbasePrime,
    // /// The hierarchy type of the activity. (Not documented in the API).
    // pub hierarchy_type: String,
}

/// Represents a user action associated with an activity.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ActivityUserAction {
    /// The type of action taken by the user.
    pub action: ActivityUserActionKind,
    /// The ID of the user who executed the action.
    pub user_id: String,
    /// The timestamp when the action was taken.
    pub timestamp: String,
}

/// Metadata associated with various aspects of an activity, such as transactions and accounts.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ActivityMetadata {
    /// Details about the consensus process for an activity.
    pub consensus: ActivityConsensus,
}

/// Represents the consensus details of an activity.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ActivityConsensus {
    /// The deadline for the approval of an activity.
    pub approval_deadline: String,
    /// Indicates whether the activity has passed the consensus threshold.
    pub has_passed_consensus: bool,
}
