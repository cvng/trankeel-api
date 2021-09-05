use async_graphql::InputObject;
use piteo_data::AttachableId;
use piteo_data::FileType;

#[derive(InputObject)]
pub struct MailSendInput {
    id: AttachableId,
    r#type: FileType,
}
