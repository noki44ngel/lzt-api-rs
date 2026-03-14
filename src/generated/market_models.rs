//! Generated models from OpenAPI schema

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceModel {
    pub balance: String,
    pub balance_id: i64,
    pub custom_title: serde_json::Value,
    pub full_title: String,
    pub merchant_id: i64,
    pub title: String,
    pub type: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryIDModel {
    // No properties defined
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmationCodeModel {
    pub code_data: serde_json::Value,
    pub item: ItemModel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyModel {
    // No properties defined
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatePeriodModel {
    // No properties defined
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscountModel {
    pub category_id: i64,
    pub discount_id: i64,
    pub discount_percent: i64,
    pub discount_user_id: i64,
    pub max_price: i64,
    pub min_price: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtraModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark_ascended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_hypixel_ban: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_spam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_login: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dota2_mmr: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ea_games: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genshin_currency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honkai_currency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_without_cookies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub the_quarry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplay_games: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warframe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zenless_currency: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceModel {
    pub additional_data: String,
    pub amount: i64,
    pub comment: String,
    pub expires_at: i64,
    pub invoice_date: i64,
    pub invoice_id: i64,
    pub is_test: bool,
    pub merchant_id: i64,
    pub paid_date: i64,
    pub payer_user_id: i64,
    pub payment_id: String,
    pub resend_attempts: i64,
    pub status: String,
    pub url: String,
    pub url_callback: String,
    pub url_success: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemFromListModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_discount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bump_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_bump_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_buy_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_close_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_open_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_resell_item_after_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_stick_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_unstick_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_update_item_stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_validate_account: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_account_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_email_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_login_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_html_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_guarantee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ignored: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sticky: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_origin_phrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsb: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resale_item_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rub_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_get_email_code_button: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_stat_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemIDModel {
    // No properties defined
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemListModel {
    pub has_next_page: bool,
    pub items: Vec<ItemFromListModel>,
    pub page: i64,
    pub per_page: i64,
    pub search_url: String,
    pub sticky_items: Vec<ItemFromListModel>,
    pub system_info: RespSystemInfo,
    pub total_items: i64,
    pub total_items_price: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemModel {
    pub account_link: String,
    pub account_links: Vec<serde_json::Value>,
    pub account_last_activity: i64,
    pub ai_price: i64,
    pub ai_price_check_date: i64,
    pub allow_ask_discount: i64,
    pub auto_buy_price: i64,
    pub auto_buy_price_check_date: i64,
    pub bump_settings: serde_json::Value,
    pub buyer: serde_json::Value,
    pub buyer_avatar_date: i64,
    pub buyer_display_icon_group_id: i64,
    pub buyer_uniq_banner: String,
    pub buyer_user_group_id: i64,
    pub can_ask_discount: bool,
    pub can_change_email_password: bool,
    pub can_change_password: bool,
    pub can_check_ai_price: bool,
    pub can_check_auto_buy_price: bool,
    pub can_check_guarantee: bool,
    pub can_report_item: bool,
    pub can_resell_item: bool,
    pub can_resell_item_after_purchase: bool,
    pub can_share_item: bool,
    pub can_update_item_stats: bool,
    pub can_validate_account: bool,
    pub can_view_account_link: bool,
    pub can_view_account_login_and_temp_email: bool,
    pub can_view_email_login_data: bool,
    pub can_view_item_views: bool,
    pub can_view_login_data: bool,
    pub cart_price: serde_json::Value,
    pub category_id: i64,
    pub content_id: serde_json::Value,
    pub content_type: serde_json::Value,
    pub copy_format_data: serde_json::Value,
    pub custom_fields: serde_json::Value,
    pub delete_date: i64,
    pub delete_reason: String,
    pub delete_user_id: i64,
    pub delete_username: String,
    pub deposit: i64,
    pub description: String,
    pub description_en_html: String,
    pub description_en_plain: String,
    pub description_html: String,
    pub description_plain: String,
    pub description_en: String,
    pub edit_date: i64,
    pub email_provider: String,
    pub email_type: String,
    pub extended_guarantee: i64,
    pub external_auth: Vec<serde_json::Value>,
    pub extra_prices: Vec<serde_json::Value>,
    pub feedback_data: String,
    pub get_email_code_display_login: serde_json::Value,
    pub guarantee: serde_json::Value,
    pub image_preview_links: Vec<String>,
    pub in_cart: serde_json::Value,
    pub information: String,
    pub information_en: String,
    pub is_birthday_today: bool,
    pub is_ignored: bool,
    pub is_personal_account: bool,
    pub is_small_exf: bool,
    pub is_trusted: bool,
    pub is_fave: serde_json::Value,
    pub is_sticky: i64,
    pub item_origin_phrase: String,
    pub item_domain: String,
    pub item_id: i64,
    pub item_origin: String,
    pub item_state: String,
    pub login: String,
    pub login_data: serde_json::Value,
    pub market_custom_title: String,
    pub max_discount_percent: i64,
    pub need_to_require_video_to_view_login_data: bool,
    pub note_text: String,
    pub nsb: i64,
    pub pending_deletion_date: i64,
    pub price: i64,
    pub price_with_seller_fee: f64,
    pub price_with_seller_fee_label: String,
    pub price_currency: String,
    pub published_date: i64,
    pub refreshed_date: i64,
    pub resale_item_origin: String,
    pub rub_price: i64,
    pub seller: serde_json::Value,
    pub show_get_email_code_button: bool,
    pub tags: serde_json::Value,
    pub temp_email: String,
    pub title: String,
    pub title_en: String,
    pub unique_key_exists: bool,
    pub update_stat_date: i64,
    pub user_allow_ask_discount: i64,
    pub view_count: i64,
    pub visitor_is_author: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespSystemInfo {
    pub log_id: i64,
    pub time: i64,
    pub visitor_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    pub active_items_count: i64,
    pub activity_visible: bool,
    pub age: i64,
    pub balance: String,
    pub balances: Vec<serde_json::Value>,
    pub bump_item_period: i64,
    pub can_edit: bool,
    pub can_follow: bool,
    pub can_ignore: bool,
    pub can_post_profile: bool,
    pub can_view_profile: bool,
    pub can_view_profile_posts: bool,
    pub can_warn: bool,
    pub contest_count: i64,
    pub conv_welcome_message: String,
    pub converted_balance: i64,
    pub converted_deposit: i64,
    pub converted_hold: i64,
    pub currency: String,
    pub currency_phrase: String,
    pub custom_account_download_format: String,
    pub custom_fields: serde_json::Value,
    pub custom_title: String,
    pub deposit: i64,
    pub dob: serde_json::Value,
    pub feedback_data: serde_json::Value,
    pub hold: String,
    pub homepage: String,
    pub imap_data: serde_json::Value,
    pub is_admin: bool,
    pub is_banned: bool,
    pub is_followed: bool,
    pub is_ignored: bool,
    pub is_moderator: bool,
    pub is_staff: bool,
    pub is_super_admin: bool,
    pub joined_date: i64,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub location: String,
    pub market_custom_title: String,
    pub max_discount_percent: i64,
    pub message_count: i64,
    pub paid_mail_left: i64,
    pub public_tags: Vec<serde_json::Value>,
    pub register_date: i64,
    pub rendered: serde_json::Value,
    pub restore_count: i64,
    pub restore_data: serde_json::Value,
    pub short_link: String,
    pub sold_items_count: i64,
    pub tags: Vec<serde_json::Value>,
    pub telegram_client: serde_json::Value,
    pub trophy_points: i64,
    pub user_allow_ask_discount: bool,
    pub user_id: i64,
    pub user_title: String,
    pub username: String,
    pub view_url: String,
    pub visible: bool,
    pub warning_points: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YesNoNoMatterScheme {
    // No properties defined
}

