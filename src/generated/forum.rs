use crate::client::ForumClient;
use crate::error::{ApiError, ApiResult};
use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CategoriesResponse {
    pub categories: Vec<Category>,
    pub categories_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CategoryResponse {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Category {
    pub category_id: i64,
    pub category_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_description: Option<String>,
    pub links: CategoryLinks,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<CategoryPermissions>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CategoryLinks {
    pub permalink: String,
    pub detail: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CategoryPermissions {
    pub view: bool,
    pub edit: bool,
    pub delete: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ForumsResponse {
    pub forums: Vec<Forum>,
    pub forums_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ForumResponse {
    pub forum: Forum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Forum {
    pub forum_id: i64,
    pub forum_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_description: Option<String>,
    pub links: ForumLinks,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ForumPermissions>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ForumLinks {
    pub permalink: String,
    pub detail: String,
    pub threads: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ForumPermissions {
    pub view: bool,
    pub edit: bool,
    pub delete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ThreadsResponse {
    pub threads: Vec<Thread>,
    pub threads_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ThreadResponse {
    pub thread: Thread,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Thread {
    pub thread_id: i64,
    pub title: String,
    pub user_id: i64,
    pub username: String,
    pub reply_count: i64,
    pub view_count: i64,
    pub links: ThreadLinks,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ThreadLinks {
    pub permalink: String,
    pub detail: String,
    pub posts: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
    pub posts_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Post {
    pub post_id: i64,
    pub user_id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub post_date: i64,
    pub links: PostLinks,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PostLinks {
    pub permalink: String,
    pub detail: String,
    pub user: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<AvatarUrls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<UserLinks>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AvatarUrls {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserLinks {
    pub permalink: String,
    pub detail: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ConversationsResponse {
    pub conversations: Vec<Conversation>,
    pub conversations_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ConversationResponse {
    pub conversation: Conversation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Conversation {
    pub conversation_id: i64,
    pub title: String,
    pub starter_user_id: i64,
    pub reply_count: i64,
    pub last_message_date: i64,
    pub links: ConversationLinks,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ConversationLinks {
    pub permalink: String,
    pub detail: String,
    pub messages: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SystemInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_time: Option<i64>,
}

// Request types for POST/PUT
#[derive(serde::Serialize, Debug, Clone)]
pub struct CreateThreadRequest {
    pub forum_id: i64,
    pub title: String,
    pub message: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CreatePostRequest {
    pub thread_id: i64,
    pub message: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PostResponse {
    pub post: Post,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

impl ForumClient {
    // GET methods
    pub async fn get_categories(&self) -> ApiResult<CategoriesResponse> {
        let builder = self.api().get("/categories");
        self.api().execute_json(builder).await
    }

    pub async fn get_category(&self, category_id: i64) -> ApiResult<CategoryResponse> {
        let builder = self.api().get(&format!("/categories/{}", category_id));
        self.api().execute_json(builder).await
    }

    pub async fn get_forums(&self) -> ApiResult<ForumsResponse> {
        let builder = self.api().get("/forums");
        self.api().execute_json(builder).await
    }

    pub async fn get_forum(&self, forum_id: i64) -> ApiResult<ForumResponse> {
        let builder = self.api().get(&format!("/forums/{}", forum_id));
        self.api().execute_json(builder).await
    }

    pub async fn get_threads(&self, forum_id: i64, page: Option<i64>) -> ApiResult<ThreadsResponse> {
        let endpoint = match page {
            Some(p) => format!("/forums/{}/threads?page={}", forum_id, p),
            None => format!("/forums/{}/threads", forum_id),
        };
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn get_thread(&self, thread_id: i64) -> ApiResult<ThreadResponse> {
        let builder = self.api().get(&format!("/threads/{}", thread_id));
        self.api().execute_json(builder).await
    }

    pub async fn get_posts(&self, thread_id: i64, page: Option<i64>) -> ApiResult<PostsResponse> {
        let endpoint = match page {
            Some(p) => format!("/threads/{}/posts?page={}", thread_id, p),
            None => format!("/threads/{}/posts", thread_id),
        };
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn get_user(&self, user_id: i64) -> ApiResult<UserResponse> {
        let builder = self.api().get(&format!("/users/{}", user_id));
        self.api().execute_json(builder).await
    }

    pub async fn get_me(&self) -> ApiResult<UserResponse> {
        let builder = self.api().get("/users/me");
        self.api().execute_json(builder).await
    }

    pub async fn get_conversations(&self, page: Option<i64>) -> ApiResult<ConversationsResponse> {
        let endpoint = match page {
            Some(p) => format!("/conversations?page={}", p),
            None => "/conversations".to_string(),
        };
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn get_conversation(&self, conversation_id: i64) -> ApiResult<ConversationResponse> {
        let builder = self.api().get(&format!("/conversations/{}", conversation_id));
        self.api().execute_json(builder).await
    }

    // POST methods
    pub async fn create_thread(&self, request: &CreateThreadRequest) -> ApiResult<ThreadResponse> {
        let builder = self.api().post("/threads");
        let builder = builder.json(request);
        self.api().execute_json(builder).await
    }

    pub async fn create_post(&self, request: &CreatePostRequest) -> ApiResult<PostResponse> {
        let builder = self.api().post("/posts");
        let builder = builder.json(request);
        self.api().execute_json(builder).await
    }

    // PUT methods
    pub async fn update_user(&self, user_id: i64, request: &UpdateUserRequest) -> ApiResult<UserResponse> {
        let builder = self.api().put(&format!("/users/{}", user_id));
        let builder = builder.json(request);
        self.api().execute_json(builder).await
    }

    // DELETE methods
    pub async fn delete_thread(&self, thread_id: i64) -> ApiResult<Value> {
        let builder = self.api().delete(&format!("/threads/{}", thread_id));
        self.api().execute(builder).await?.json().await.map_err(ApiError::from)
    }

    pub async fn delete_post(&self, post_id: i64) -> ApiResult<Value> {
        let builder = self.api().delete(&format!("/posts/{}", post_id));
        self.api().execute(builder).await?.json().await.map_err(ApiError::from)
    }

    pub async fn delete_conversation(&self, conversation_id: i64) -> ApiResult<Value> {
        let builder = self.api().delete(&format!("/conversations/{}", conversation_id));
        self.api().execute(builder).await?.json().await.map_err(ApiError::from)
    }
}
