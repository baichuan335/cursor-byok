//! Exercises subagent follow-ups through the ordered Cursor transport without model API calls.
#[path = "support/fake_provider.rs"]
mod fake_provider;
#[path = "support/fixtures.rs"]
mod fixtures;

use std::{collections::HashMap, sync::Arc, time::Duration};

use cursor_server::{
    cursor::{
        prompting::{PromptAssets, PromptCompiler},
        protocol::{connect, proto::agent::v1 as pb},
        TransportCommand, TransportParent, TransportRegistry,
    },
    model::{ContentPart, ConversationId, MessageContent},
    provider::{FinishReason, ModelEvent},
};
use prost::Message;

fn response(id: &str) -> Vec<ModelEvent> {
    vec![
        ModelEvent::Start {
            model_call_id: id.into(),
        },
        ModelEvent::TextStart,
        ModelEvent::TextDelta("Acknowledged.".into()),
        ModelEvent::TextEnd,
        ModelEvent::Done(FinishReason::Stop),
    ]
}

fn user_action(id: &str) -> pb::ConversationAction {
    pb::ConversationAction {
        action: Some(pb::conversation_action::Action::UserMessageAction(
            pb::UserMessageAction {
                user_message: Some(pb::UserMessage {
                    text: "Continue inspecting the code.".into(),
                    message_id: id.into(),
                    mode: pb::AgentMode::Agent as i32,
                    ..Default::default()
                }),
                request_context: Some(pb::RequestContext::default()),
                ..Default::default()
            },
        )),
        ..Default::default()
    }
}

fn run(child: bool, id: &str) -> pb::AgentClientMessage {
    pb::AgentClientMessage {
        message: Some(pb::agent_client_message::Message::RunRequest(
            pb::AgentRunRequest {
                conversation_id: Some("resume-conversation".into()),
                subagent_type_name: child.then(|| "explore".into()),
                requested_model: Some(pb::RequestedModel {
                    model_id: "model".into(),
                    ..Default::default()
                }),
                action: Some(user_action(id)),
                ..Default::default()
            },
        )),
    }
}

fn runtime_user(id: &str) -> pb::AgentClientMessage {
    pb::AgentClientMessage {
        message: Some(pb::agent_client_message::Message::ConversationAction(
            user_action(id),
        )),
    }
}

#[tokio::test]
async fn subagent_followups_without_message_ids_keep_distinct_checkpoint_messages() {
    check_followups(false).await;
}

#[tokio::test]
async fn parent_headers_identify_subagents_without_a_type_name() {
    check_followups(true).await;
}

async fn check_followups(via_parent: bool) {
    let (_directory, store) = fixtures::temp_store().await;
    let provider = fake_provider::FakeProvider::default();
    provider.push(response("first"));
    provider.push(response("second"));
    provider.push(response("third"));
    let assets =
        PromptAssets::load(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("prompt/cursor"))
            .unwrap();
    let registry = TransportRegistry::new(
        store.clone(),
        Arc::new(provider.clone()),
        PromptCompiler::new(assets),
    );
    let handle = registry
        .get_or_create("subagent-resume-wire")
        .await
        .unwrap();
    if via_parent {
        handle
            .set_parent(TransportParent {
                request_id: "parent-request".into(),
                tool_call_id: "parent-task".into(),
            })
            .unwrap();
    }
    let mut output = handle.subscribe();
    let initial = run(!via_parent, "");
    handle
        .command(TransportCommand::Append {
            seqno: 0,
            message: Box::new(initial.clone()),
        })
        .await
        .unwrap();
    // A retry of the same transport sequence must not produce another turn.
    handle
        .command(TransportCommand::Append {
            seqno: 0,
            message: Box::new(initial),
        })
        .await
        .unwrap();
    let mut seqno = 1;
    let mut turns = 0;
    let mut users = Vec::new();
    let mut blobs = HashMap::<Vec<u8>, Vec<u8>>::new();
    loop {
        let frame = tokio::time::timeout(Duration::from_secs(8), output.recv())
            .await
            .unwrap()
            .unwrap();
        let (flags, payload) = connect::decode_frames(&frame).unwrap().pop().unwrap();
        if flags & connect::END_STREAM_FLAG != 0 {
            assert_eq!(
                payload.as_ref(),
                b"{}",
                "subagent resume must complete without a protocol error"
            );
            break;
        }
        let server = pb::AgentServerMessage::decode(payload).unwrap();
        match server.message {
            Some(pb::agent_server_message::Message::KvServerMessage(kv)) => {
                if let Some(pb::kv_server_message::Message::SetBlobArgs(set)) = kv.message {
                    blobs.insert(set.blob_id, set.blob_data);
                }
                let ack = pb::AgentClientMessage {
                    message: Some(pb::agent_client_message::Message::KvClientMessage(
                        pb::KvClientMessage {
                            id: kv.id,
                            message: Some(pb::kv_client_message::Message::SetBlobResult(
                                pb::SetBlobResult { error: None },
                            )),
                        },
                    )),
                };
                handle
                    .command(TransportCommand::Append {
                        seqno,
                        message: Box::new(ack),
                    })
                    .await
                    .unwrap();
                seqno += 1;
            }
            Some(pb::agent_server_message::Message::ConversationCheckpointUpdate(state))
                if state.pending_tool_calls.is_empty() =>
            {
                let Some(turn_id) = state.turns.last() else {
                    continue;
                };
                let Some(encoded) = blobs.get(turn_id) else {
                    continue;
                };
                let turn = pb::ConversationTurnStructure::decode(encoded.as_slice()).unwrap();
                let Some(pb::conversation_turn_structure::Turn::AgentConversationTurn(turn)) =
                    turn.turn
                else {
                    continue;
                };
                let Some(encoded) = blobs.get(&turn.user_message) else {
                    continue;
                };
                let user = pb::UserMessage::decode(encoded.as_slice()).unwrap();
                assert!(!user.message_id.is_empty());
                if users.contains(&user.message_id) || turn.steps.is_empty() {
                    continue;
                }
                users.push(user.message_id);
                turns += 1;
                if turns < 3 {
                    let followup = runtime_user(if turns == 2 { "client-provided-id" } else { "" });
                    handle
                        .command(TransportCommand::Append {
                            seqno,
                            message: Box::new(followup.clone()),
                        })
                        .await
                        .unwrap();
                    handle
                        .command(TransportCommand::Append {
                            seqno,
                            message: Box::new(followup),
                        })
                        .await
                        .unwrap();
                    seqno += 1;
                }
            }
            Some(pb::agent_server_message::Message::ExecServerMessage(exec)) => {
                panic!("unexpected exec during resume: {}", exec.id)
            }
            _ => {}
        }
    }
    assert_eq!(provider.requests().len(), 3);
    assert_eq!(turns, 3);
    assert_ne!(
        users[0], users[1],
        "identical text in separate follow-ups must not collapse"
    );
    assert_eq!(
        users[2], "client-provided-id",
        "existing IDs must not be overwritten"
    );
    let messages = store
        .load_current_messages(&ConversationId::new("resume-conversation"))
        .await
        .unwrap();
    let runtime = messages
        .iter()
        .filter(|m| m.runtime_event_id.is_some())
        .collect::<Vec<_>>();
    assert_eq!(runtime.len(), 3);
    assert!(runtime.iter().all(|m| matches!(&m.content, MessageContent::Parts { parts } if parts.iter().any(|p| matches!(p, ContentPart::Text { text } if text.contains("Continue inspecting the code."))))));
}

#[tokio::test]
async fn a_running_subagent_accepts_a_followup_without_a_message_id() {
    let (_directory, store) = fixtures::temp_store().await;
    let provider = fake_provider::FakeProvider::default();
    provider.push_pending();
    provider.push(response("after-followup"));
    let assets =
        PromptAssets::load(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("prompt/cursor"))
            .unwrap();
    let registry = TransportRegistry::new(
        store.clone(),
        Arc::new(provider.clone()),
        PromptCompiler::new(assets),
    );
    let handle = registry
        .get_or_create("active-subagent-wire")
        .await
        .unwrap();
    let mut output = handle.subscribe();
    handle
        .command(TransportCommand::Append {
            seqno: 0,
            message: Box::new(run(true, "first-message")),
        })
        .await
        .unwrap();
    let mut seqno = 1;
    let mut sent_followup = false;
    let mut poll = tokio::time::interval(Duration::from_millis(10));
    loop {
        let frame = tokio::time::timeout(Duration::from_secs(8), async {
            loop {
                tokio::select! {
                    frame = output.recv() => return frame,
                    _ = poll.tick(), if !sent_followup => {
                        if !provider.requests().is_empty() {
                            handle.command(TransportCommand::Append {
                                seqno,
                                message: Box::new(runtime_user("")),
                            }).await.unwrap();
                            seqno += 1;
                            sent_followup = true;
                        }
                    }
                }
            }
        })
        .await
        .unwrap()
        .unwrap();
        let (flags, payload) = connect::decode_frames(&frame).unwrap().pop().unwrap();
        if flags & connect::END_STREAM_FLAG != 0 {
            assert_eq!(payload.as_ref(), b"{}");
            break;
        }
        let server = pb::AgentServerMessage::decode(payload).unwrap();
        if let Some(pb::agent_server_message::Message::KvServerMessage(kv)) = server.message {
            handle
                .command(TransportCommand::Append {
                    seqno,
                    message: Box::new(pb::AgentClientMessage {
                        message: Some(pb::agent_client_message::Message::KvClientMessage(
                            pb::KvClientMessage {
                                id: kv.id,
                                message: Some(pb::kv_client_message::Message::SetBlobResult(
                                    pb::SetBlobResult { error: None },
                                )),
                            },
                        )),
                    }),
                })
                .await
                .unwrap();
            seqno += 1;
        }
    }
    assert_eq!(provider.requests().len(), 2);
    let messages = store
        .load_current_messages(&ConversationId::new("resume-conversation"))
        .await
        .unwrap();
    assert_eq!(
        messages
            .iter()
            .filter(|m| m.runtime_event_id.is_some())
            .count(),
        2
    );
}

#[tokio::test]
async fn a_root_request_without_message_id_is_still_rejected() {
    let (_directory, store) = fixtures::temp_store().await;
    let provider = fake_provider::FakeProvider::default();
    let assets =
        PromptAssets::load(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("prompt/cursor"))
            .unwrap();
    let registry = TransportRegistry::new(
        store,
        Arc::new(provider.clone()),
        PromptCompiler::new(assets),
    );
    let handle = registry.get_or_create("invalid-root-wire").await.unwrap();
    let mut output = handle.subscribe();
    handle
        .command(TransportCommand::Append {
            seqno: 0,
            message: Box::new(run(false, "")),
        })
        .await
        .unwrap();
    loop {
        let frame = tokio::time::timeout(Duration::from_secs(5), output.recv())
            .await
            .unwrap()
            .unwrap();
        let (flags, payload) = connect::decode_frames(&frame).unwrap().pop().unwrap();
        if flags & connect::END_STREAM_FLAG != 0 {
            let error: serde_json::Value = serde_json::from_slice(&payload).unwrap();
            assert_eq!(error["error"]["code"], "invalid_argument");
            assert!(error["error"]["message"]
                .as_str()
                .unwrap()
                .contains("has no message_id"));
            break;
        }
    }
    assert!(provider.requests().is_empty());
}
