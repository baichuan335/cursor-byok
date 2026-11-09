//! Classifies Cursor actions and selects their message delivery behavior.

use crate::{
    cursor::protocol::proto::agent::v1 as pb,
    model::{CanonicalMessage, RunId},
    store::BlobId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageDelivery {
    Ignore,
    InsertMessages,
    BreakMessages,
}

#[derive(Clone, Debug)]
pub struct CompiledMessages {
    pub event_id: String,
    pub target_run_id: Option<RunId>,
    pub messages: Vec<CanonicalMessage>,
    pub delivery: MessageDelivery,
}

impl CompiledMessages {
    pub fn ignored(event_id: impl Into<String>) -> Self {
        Self {
            event_id: event_id.into(),
            target_run_id: None,
            messages: Vec::new(),
            delivery: MessageDelivery::Ignore,
        }
    }
}

/// Cursor subagent follow-ups can omit a message ID. Use the ordered transport
/// identity, not the text or a random UUID, so retries deduplicate and repeated
/// follow-ups with identical text remain distinct. Root message validation stays strict.
pub(crate) fn identify_subagent_message(
    action: &mut pb::ConversationAction,
    request_id: &str,
    seqno: i64,
) {
    let Some(pb::conversation_action::Action::UserMessageAction(action)) = action.action.as_mut()
    else {
        return;
    };
    let Some(user) = action
        .user_message
        .as_mut()
        .filter(|user| user.message_id.is_empty())
    else {
        return;
    };
    let transport_id = BlobId::digest(request_id.as_bytes()).to_base64();
    user.message_id = format!("subagent-user:{transport_id}:{seqno}");
    tracing::debug!(seqno, "assigned missing Cursor subagent user message id");
}

pub fn delivery(action: &pb::conversation_action::Action) -> MessageDelivery {
    use pb::conversation_action::Action;
    match action {
        Action::BackgroundTaskCompletionAction(_)
        | Action::BackgroundShellAction(_)
        | Action::BackgroundSubagentAction(_)
        | Action::AsyncAskQuestionCompletionAction(_)
        | Action::SubscriptionNotificationAction(_)
        | Action::GoalContinuationAction(_) => MessageDelivery::InsertMessages,
        Action::UserMessageAction(_) | Action::InjectContextAction(_) => {
            MessageDelivery::BreakMessages
        }
        Action::CancelAction(_)
        | Action::CancelSubagentAction(_)
        | Action::ResumeAction(_)
        | Action::SummarizeAction(_)
        | Action::ShellCommandAction(_)
        | Action::StartPlanAction(_)
        | Action::ExecutePlanAction(_) => MessageDelivery::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action() -> pb::ConversationAction {
        pb::ConversationAction {
            action: Some(pb::conversation_action::Action::UserMessageAction(
                pb::UserMessageAction {
                    user_message: Some(pb::UserMessage {
                        text: "Continue".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }
    }

    fn user_id(action: &pb::ConversationAction) -> &str {
        let Some(pb::conversation_action::Action::UserMessageAction(action)) =
            action.action.as_ref()
        else {
            unreachable!()
        };
        &action.user_message.as_ref().unwrap().message_id
    }

    #[test]
    fn subagent_message_identity_is_stable_for_retries_and_scoped_to_the_transport() {
        let mut first = action();
        identify_subagent_message(&mut first, "request-a", 7);
        let mut retry = action();
        identify_subagent_message(&mut retry, "request-a", 7);
        assert_eq!(user_id(&first), user_id(&retry));
        let mut next = action();
        identify_subagent_message(&mut next, "request-a", 8);
        assert_ne!(user_id(&first), user_id(&next));
        let mut other = action();
        identify_subagent_message(&mut other, "request-b", 7);
        assert_ne!(user_id(&first), user_id(&other));
        identify_subagent_message(&mut first, "request-b", 9);
        assert_eq!(
            user_id(&first),
            user_id(&retry),
            "an assigned ID is preserved"
        );
    }
}
