use piteo_data::AttachableId;
use piteo_data::FileType;

#[derive(async_graphql::InputObject)]
pub struct MailSendInput {
    id: AttachableId,
    r#type: FileType,
}
