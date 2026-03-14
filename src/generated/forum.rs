use crate::client::ForumClient;
use crate::error::{ApiError, ApiResult};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoriesResponse { pub categories: Vec<Value>, pub categories_total: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryResponse { pub category: Value, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumsResponse { pub forums: Vec<Value>, pub forums_total: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumResponse { pub forum: Value, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadsResponse { pub threads: Vec<Value>, pub threads_total: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadResponse { pub thread: Value, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostsResponse { pub posts: Vec<Value>, pub posts_total: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostResponse { pub post: Value, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse { pub user: Value, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationsResponse { pub conversations: Vec<Value>, pub conversations_total: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationResponse { pub conversation: Value, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse { pub access_token: String, pub token_type: String, pub expires_in: i64, #[serde(skip_serializing_if = "Option::is_none")] pub refresh_token: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] pub scope: Option<String> }

impl ForumClient {
    pub async fn get_categories(&self) -> ApiResult<CategoriesResponse> { self.api().execute_json(self.api().get("/categories")).await }
    pub async fn get_category(&self, category_id: i64) -> ApiResult<CategoryResponse> { self.api().execute_json(self.api().get(&format!("/categories/{}", category_id))).await }
    pub async fn get_forums(&self) -> ApiResult<ForumsResponse> { self.api().execute_json(self.api().get("/forums")).await }
    pub async fn get_forum(&self, forum_id: i64) -> ApiResult<ForumResponse> { self.api().execute_json(self.api().get(&format!("/forums/{}", forum_id))).await }
    pub async fn get_threads(&self, forum_id: i64, page: Option<i64>) -> ApiResult<ThreadsResponse> { let ep = match page { Some(p) => format!("/forums/{}/threads?page={}", forum_id, p), None => format!("/forums/{}/threads", forum_id) }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn get_thread(&self, thread_id: i64) -> ApiResult<ThreadResponse> { self.api().execute_json(self.api().get(&format!("/threads/{}", thread_id))).await }
    pub async fn get_posts(&self, thread_id: i64, page: Option<i64>) -> ApiResult<PostsResponse> { let ep = match page { Some(p) => format!("/threads/{}/posts?page={}", thread_id, p), None => format!("/threads/{}/posts", thread_id) }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn get_user(&self, user_id: i64) -> ApiResult<UserResponse> { self.api().execute_json(self.api().get(&format!("/users/{}", user_id))).await }
    pub async fn get_me(&self) -> ApiResult<UserResponse> { self.api().execute_json(self.api().get("/users/me")).await }
    pub async fn get_conversations(&self, page: Option<i64>) -> ApiResult<ConversationsResponse> { let ep = match page { Some(p) => format!("/conversations?page={}", p), None => "/conversations".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn get_conversation(&self, conversation_id: i64) -> ApiResult<ConversationResponse> { self.api().execute_json(self.api().get(&format!("/conversations/{}", conversation_id))).await }
    pub async fn create_thread(&self, forum_id: i64, title: &str, message: &str) -> ApiResult<ThreadResponse> { let req = serde_json::json!({"forum_id": forum_id, "title": title, "message": message}); self.api().execute_json(self.api().post("/threads").json(&req)).await }
    pub async fn create_post(&self, thread_id: i64, message: &str) -> ApiResult<PostResponse> { let req = serde_json::json!({"thread_id": thread_id, "message": message}); self.api().execute_json(self.api().post("/posts").json(&req)).await }
    pub async fn delete_thread(&self, thread_id: i64) -> ApiResult<Value> { self.api().execute(self.api().delete(&format!("/threads/{}", thread_id))).await?.json().await.map_err(ApiError::from) }
    pub async fn delete_post(&self, post_id: i64) -> ApiResult<Value> { self.api().execute(self.api().delete(&format!("/posts/{}", post_id))).await?.json().await.map_err(ApiError::from) }
    pub async fn follow_forum(&self, forum_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().post(&format!("/forums/{}/follow", forum_id))).await }
    pub async fn unfollow_forum(&self, forum_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().delete(&format!("/forums/{}/follow", forum_id))).await }
    pub async fn bump_thread(&self, thread_id: i64) -> ApiResult<ThreadResponse> { self.api().execute_json(self.api().post(&format!("/threads/{}/bump", thread_id))).await }
    pub async fn star_thread(&self, thread_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().post(&format!("/threads/{}/star", thread_id))).await }
    pub async fn unstar_thread(&self, thread_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().delete(&format!("/threads/{}/star", thread_id))).await }
    pub async fn follow_thread(&self, thread_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().post(&format!("/threads/{}/follow", thread_id))).await }
    pub async fn unfollow_thread(&self, thread_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().delete(&format!("/threads/{}/follow", thread_id))).await }
    pub async fn like_post(&self, post_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().post(&format!("/posts/{}/like", post_id))).await }
    pub async fn unlike_post(&self, post_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().delete(&format!("/posts/{}/like", post_id))).await }
    pub async fn get_oauth_token(&self, grant_type: &str, client_id: &str, client_secret: &str, scope: Vec<&str>) -> ApiResult<TokenResponse> { let req = serde_json::json!({"grant_type": grant_type, "client_id": client_id, "client_secret": client_secret, "scope": scope}); self.api().execute_json(self.api().post("/oauth/token").json(&req)).await }
}
