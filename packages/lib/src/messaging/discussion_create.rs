use super::push_message;
use crate::PushMessageInput;
use crate::Result;
use piteo_core::database::Db;
use piteo_data::Discussion;
use piteo_data::DiscussionId;
use piteo_data::DiscussionType;
use piteo_data::PersonId;
use piteo_data::SubjectId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateDiscussionInput {
    pub initiator_id: PersonId,
    pub recipient_id: PersonId,
    pub subject_id: Option<SubjectId>,
    pub message: String,
}

pub fn create_discussion_unauthenticated(
    db: &impl Db,
    input: CreateDiscussionInput,
) -> Result<Discussion> {
    input.validate()?;

    let account = db.accounts().by_person_id(&input.recipient_id)?;

    let type_ = get_discussion_type(db, &input.subject_id);

    let discussion = db.discussions().create(Discussion {
        id: DiscussionId::new_v4(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        initiator_id: input.initiator_id,
        subject_id: input.subject_id,
        type_,
    })?;

    push_message(
        db,
        PushMessageInput {
            discussion_id: discussion.id,
            sender_id: input.initiator_id,
            message: input.message,
        },
    )?;

    Ok(discussion)
}

fn get_discussion_type(db: &impl Db, subject_id: &Option<SubjectId>) -> DiscussionType {
    match subject_id {
        Some(subject_id) => {
            if db.candidacies().by_id(subject_id).is_ok() {
                DiscussionType::Candidacy
            } else {
                DiscussionType::default()
            }
        }
        None => DiscussionType::default(),
    }
}
