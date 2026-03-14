pub mod forum;
pub mod market;
pub mod models;

pub use forum::{
    CategoriesResponse, Category, CategoryLinks, CategoryPermissions, CategoryResponse,
    Conversation, ConversationLinks, ConversationResponse, ConversationsResponse,
    CreatePostRequest, CreateThreadRequest, UpdateUserRequest,
    Forum, ForumLinks, ForumPermissions, ForumResponse, ForumsResponse,
    Post, PostLinks, PostResponse, PostsResponse, SystemInfo,
    Thread, ThreadLinks, ThreadResponse, ThreadsResponse,
    User, UserLinks, UserResponse, AvatarUrls as ForumAvatarUrls,
};

pub use market::{
    BuyItemRequest, CreateItemRequest, Item, ItemFromList, ItemResponse, ItemUser, ItemsResponse,
    SearchParams, StatsResponse, SystemInfo as MarketSystemInfo,
    Tag, TagsResponse, UpdateItemRequest,
};

pub use models::{
    ApiSystemInfo, ErrorResponse, Link, Pagination,
    TokenRequest, TokenResponse, UserSummary,
};

pub use crate::client::{ForumClient, MarketClient};
