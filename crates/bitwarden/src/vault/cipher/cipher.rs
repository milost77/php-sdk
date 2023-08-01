use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{CipherString, Decryptable, Encryptable},
    error::Result,
};

use super::{
    attachment, card, field, identity,
    local_data::{LocalData, LocalDataView},
    login, password_history, secure_note,
};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
pub enum CipherType {
    Login = 1,
    SecureNote = 2,
    Card = 3,
    Identity = 4,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
pub enum CipherRepromptType {
    None = 0,
    Password = 1,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cipher {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    pub name: CipherString,
    pub notes: CipherString,

    pub r#type: CipherType,
    pub login: Option<login::Login>,
    pub identity: Option<identity::Identity>,
    pub card: Option<card::Card>,
    pub secure_note: Option<secure_note::SecureNote>,

    pub favorite: bool,
    pub reprompt: CipherRepromptType,
    pub organization_use_totp: bool,
    pub edit: bool,
    pub view_password: bool,
    pub local_data: Option<LocalData>,

    pub attachments: Vec<attachment::Attachment>,
    pub fields: Vec<field::Field>,
    pub password_history: Vec<password_history::PasswordHistory>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherView {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    pub name: String,
    pub notes: String,

    pub r#type: CipherType,
    pub login: Option<login::LoginView>,
    pub identity: Option<identity::IdentityView>,
    pub card: Option<card::CardView>,
    pub secure_note: Option<secure_note::SecureNoteView>,

    pub favorite: bool,
    pub reprompt: CipherRepromptType,
    pub organization_use_totp: bool,
    pub edit: bool,
    pub view_password: bool,
    pub local_data: Option<LocalDataView>,

    pub attachments: Vec<attachment::AttachmentView>,
    pub fields: Vec<field::FieldView>,
    pub password_history: Vec<password_history::PasswordHistoryView>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherListView {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    pub name: String,
    pub sub_title: String,

    pub r#type: CipherType,

    pub favorite: bool,
    pub reprompt: CipherRepromptType,
    pub edit: bool,
    pub view_password: bool,

    /// The number of attachments
    pub attachments: usize,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

impl Encryptable<Cipher> for CipherView {
    fn encrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<Cipher> {
        let org_id = &self.organization_id;
        Ok(Cipher {
            id: self.id,
            organization_id: self.organization_id,
            folder_id: self.folder_id,
            collection_ids: self.collection_ids,
            name: self.name.encrypt(enc, org_id)?,
            notes: self.notes.encrypt(enc, org_id)?,
            r#type: self.r#type,
            login: self.login.encrypt(enc, org_id)?,
            identity: self.identity.encrypt(enc, org_id)?,
            card: self.card.encrypt(enc, org_id)?,
            secure_note: self.secure_note.encrypt(enc, org_id)?,
            favorite: self.favorite,
            reprompt: self.reprompt,
            organization_use_totp: self.organization_use_totp,
            edit: self.edit,
            view_password: self.view_password,
            local_data: self.local_data.encrypt(enc, org_id)?,
            attachments: self.attachments.encrypt(enc, org_id)?,
            fields: self.fields.encrypt(enc, org_id)?,
            password_history: self.password_history.encrypt(enc, org_id)?,
            creation_date: self.creation_date,
            deleted_date: self.deleted_date,
            revision_date: self.revision_date,
        })
    }
}

impl Decryptable<CipherView> for Cipher {
    fn decrypt(&self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<CipherView> {
        let org_id = &self.organization_id;
        Ok(CipherView {
            id: self.id,
            organization_id: self.organization_id,
            folder_id: self.folder_id,
            collection_ids: self.collection_ids.clone(),
            name: self.name.decrypt(enc, org_id)?,
            notes: self.notes.decrypt(enc, org_id)?,
            r#type: self.r#type,
            login: self.login.decrypt(enc, org_id)?,
            identity: self.identity.decrypt(enc, org_id)?,
            card: self.card.decrypt(enc, org_id)?,
            secure_note: self.secure_note.decrypt(enc, org_id)?,
            favorite: self.favorite,
            reprompt: self.reprompt,
            organization_use_totp: self.organization_use_totp,
            edit: self.edit,
            view_password: self.view_password,
            local_data: self.local_data.decrypt(enc, org_id)?,
            attachments: self.attachments.decrypt(enc, org_id)?,
            fields: self.fields.decrypt(enc, org_id)?,
            password_history: self.password_history.decrypt(enc, org_id)?,
            creation_date: self.creation_date,
            deleted_date: self.deleted_date,
            revision_date: self.revision_date,
        })
    }
}
