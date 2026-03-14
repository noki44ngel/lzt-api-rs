//! Generated models from OpenAPI schema

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespChatboxMessageModel {
    pub can_report: bool,
    pub date: i64,
    pub is_deleted: bool,
    pub message: String,
    pub message_json: String,
    pub message_raw: String,
    pub message_id: i64,
    pub room: serde_json::Value,
    pub user: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespConversationMessageModel {
    pub conversation_id: i64,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub links: serde_json::Value,
    pub message_body: String,
    pub message_body_html: String,
    pub message_body_plain_text: String,
    pub message_create_date: i64,
    pub message_edit_date: i64,
    pub message_id: i64,
    pub message_is_system: bool,
    pub message_is_unread: i64,
    pub message_need_translate: bool,
    pub permissions: serde_json::Value,
    pub user_is_ignored: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespConversationModel {
    pub alerts: i64,
    pub conversation_create_date: i64,
    pub conversation_id: i64,
    pub conversation_is_deleted: bool,
    pub conversation_is_new: bool,
    pub conversation_is_open: bool,
    pub conversation_last_read_date: i64,
    pub conversation_message_count: i64,
    pub conversation_online_count: i64,
    pub conversation_title: String,
    pub conversation_update_date: i64,
    pub creator_is_ignored: bool,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub is_group: i64,
    pub is_starred: i64,
    pub is_unread: i64,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub recipient: serde_json::Value,
    pub recipients: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespLinkModel {
    pub link_description: String,
    pub link_id: i64,
    pub link_title: String,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespNotificationModel {
    pub content_action: String,
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub links: serde_json::Value,
    pub notification_create_date: i64,
    pub notification_html: String,
    pub notification_id: i64,
    pub notification_is_unread: bool,
    pub notification_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespPostCommentModel {
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub post_comment_body: String,
    pub post_comment_body_html: String,
    pub post_comment_body_plain_text: String,
    pub post_comment_create_date: i64,
    pub post_comment_id: i64,
    pub post_comment_is_deleted: bool,
    pub post_comment_is_published: bool,
    pub post_comment_like_count: i64,
    pub post_comment_update_date: i64,
    pub post_id: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespPostModel {
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub user_is_ignored: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespProfilePostCommentModel {
    pub comment_body: String,
    pub comment_body_html: String,
    pub comment_body_plain_text: String,
    pub comment_create_date: i64,
    pub comment_id: i64,
    pub comment_user_id: i64,
    pub comment_username: String,
    pub comment_username_html: String,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub user_is_ignored: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespProfilePostModel {
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_comment_count: i64,
    pub post_comments_is_disabled: i64,
    pub post_create_date: i64,
    pub post_is_deleted: bool,
    pub post_is_liked: bool,
    pub post_is_published: bool,
    pub post_is_sticked: bool,
    pub post_like_count: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub profile_post_id: i64,
    pub timeline_user: RespUserModel,
    pub timeline_user_id: i64,
    pub timeline_username: String,
    pub user_is_ignored: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespSystemInfo {
    pub time: i64,
    pub visitor_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespThreadModel {
    pub contest: serde_json::Value,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: serde_json::Value,
    pub forum_id: i64,
    pub last_post: serde_json::Value,
    pub links: serde_json::Value,
    pub node_title: String,
    pub permissions: serde_json::Value,
    pub restrictions: serde_json::Value,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_closed: bool,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_starred: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: serde_json::Value,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespUserModel {
    pub balance: String,
    pub banner: String,
    pub birthday: serde_json::Value,
    pub contest_count: i64,
    pub conv_welcome_message: String,
    pub curator_titles: Vec<String>,
    pub currency: String,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub edit_permissions: serde_json::Value,
    pub fields: Vec<serde_json::Value>,
    pub hold: String,
    pub is_banned: i64,
    pub links: serde_json::Value,
    pub permissions: serde_json::Value,
    pub secret_answer_first_letter: String,
    pub secret_answer_rendered: String,
    pub self_permissions: serde_json::Value,
    pub short_link: String,
    pub trophy_count: i64,
    pub user_deposit: i64,
    pub user_email: String,
    pub user_external_authentications: Vec<serde_json::Value>,
    pub user_followers: serde_json::Value,
    pub user_following: serde_json::Value,
    pub user_group_id: i64,
    pub user_groups: Vec<serde_json::Value>,
    pub user_id: i64,
    pub user_is_followed: bool,
    pub user_is_ignored: bool,
    pub user_is_valid: bool,
    pub user_is_verified: bool,
    pub user_is_visitor: bool,
    pub user_last_seen_date: i64,
    pub user_like2_count: i64,
    pub user_like_count: i64,
    pub user_message_count: i64,
    pub user_register_date: i64,
    pub user_timezone_offset: i64,
    pub user_title: String,
    pub user_unread_conversation_count: i64,
    pub user_unread_notification_count: i64,
    pub username: String,
    pub username_html: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomIDModel {
    // No properties defined
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserIDModel {
    // No properties defined
}

