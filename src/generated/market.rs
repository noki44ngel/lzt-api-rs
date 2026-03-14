use crate::client::MarketClient;
use crate::error::{ApiError, ApiResult};
use serde_json::Value;

// ==================== Response Types ====================

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ItemsResponse {
    pub items: Vec<ItemFromList>,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "totalItemsPrice")]
    pub total_items_price: Value,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    pub page: i64,
    #[serde(rename = "searchUrl")]
    pub search_url: String,
    #[serde(rename = "stickyItems")]
    pub sticky_items: Vec<ItemFromList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ItemResponse {
    pub item: Item,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ItemFromList {
    #[serde(rename = "item_id")]
    pub item_id: i64,
    #[serde(rename = "item_state")]
    pub item_state: String,
    #[serde(rename = "category_id")]
    pub category_id: i64,
    #[serde(rename = "published_date")]
    pub published_date: i64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub price: i64,
    #[serde(rename = "update_stat_date")]
    pub update_stat_date: i64,
    #[serde(rename = "refreshed_date")]
    pub refreshed_date: i64,
    #[serde(rename = "view_count")]
    pub view_count: i64,
    #[serde(rename = "is_sticky")]
    pub is_sticky: i64,
    #[serde(rename = "item_origin")]
    pub item_origin: String,
    #[serde(rename = "extended_guarantee")]
    pub extended_guarantee: i64,
    pub nsb: i64,
    #[serde(rename = "allow_ask_discount")]
    pub allow_ask_discount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "item_id")]
    pub item_id: i64,
    #[serde(rename = "item_state")]
    pub item_state: String,
    #[serde(rename = "category_id")]
    pub category_id: i64,
    #[serde(rename = "published_date")]
    pub published_date: i64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub price: i64,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "view_count")]
    pub view_count: i64,
    #[serde(rename = "item_origin")]
    pub item_origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ItemUser>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ItemUser {
    pub user_id: i64,
    pub username: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Tag {
    pub tag_id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TagsResponse {
    pub tags: Vec<Tag>,
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct StatsResponse {
    #[serde(rename = "total_items")]
    pub total_items: i64,
    #[serde(rename = "total_sold")]
    pub total_sold: i64,
    #[serde(rename = "total_views")]
    pub total_views: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<SystemInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SystemInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_time: Option<i64>,
}

// ==================== Request Types ====================

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct SearchParams {
    pub page: Option<i64>,
    pub pmin: Option<f64>,
    pub pmax: Option<f64>,
    pub title: Option<String>,
    pub order_by: Option<String>,
    pub tag_id: Option<Vec<i64>>,
    pub origin: Option<String>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CreateItemRequest {
    pub category_id: i64,
    pub title: String,
    pub description: String,
    pub price: i64,
    pub currency: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct UpdateItemRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct BuyItemRequest {
    pub item_id: i64,
}

// ==================== Market API Client ====================

impl MarketClient {
    // ==================== Search Methods ====================

    pub async fn search_items(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn get_item(&self, item_id: i64) -> ApiResult<ItemResponse> {
        let builder = self.api().get(&format!("/{}", item_id));
        self.api().execute_json(builder).await
    }

    pub async fn search_steam(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/steam".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn search_origin(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/origin".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn search_epic(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/epic".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn search_uplay(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/uplay".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn search_battlenet(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/battlenet".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn search_socialclub(
        &self,
        params: Option<SearchParams>,
    ) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/socialclub".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    pub async fn search_eaapp(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> {
        let mut endpoint = "/eaapp".to_string();
        if let Some(p) = params {
            endpoint.push_str(&Self::build_query(&p));
        }
        let builder = self.api().get(&endpoint);
        self.api().execute_json(builder).await
    }

    // ==================== Tags & Stats ====================

    pub async fn get_tags(&self) -> ApiResult<TagsResponse> {
        let builder = self.api().get("/tags");
        self.api().execute_json(builder).await
    }

    pub async fn get_stats(&self) -> ApiResult<StatsResponse> {
        let builder = self.api().get("/stats");
        self.api().execute_json(builder).await
    }

    // ==================== CRUD Operations ====================

    pub async fn create_item(&self, request: &CreateItemRequest) -> ApiResult<ItemResponse> {
        let builder = self.api().post("/");
        let builder = builder.json(request);
        self.api().execute_json(builder).await
    }

    pub async fn update_item(
        &self,
        item_id: i64,
        request: &UpdateItemRequest,
    ) -> ApiResult<ItemResponse> {
        let builder = self.api().put(&format!("/{}", item_id));
        let builder = builder.json(request);
        self.api().execute_json(builder).await
    }

    pub async fn delete_item(&self, item_id: i64) -> ApiResult<Value> {
        let builder = self.api().delete(&format!("/{}", item_id));
        self.api()
            .execute(builder)
            .await?
            .json()
            .await
            .map_err(ApiError::from)
    }

    pub async fn buy_item(&self, request: &BuyItemRequest) -> ApiResult<Value> {
        let builder = self.api().post(&format!("/{}", request.item_id));
        self.api().execute_json(builder).await
    }

    // ==================== Helper Methods ====================

    fn build_query(params: &SearchParams) -> String {
        let mut query = String::from("?");
        if let Some(page) = params.page {
            query.push_str(&format!("page={}&", page));
        }
        if let Some(min) = params.pmin {
            query.push_str(&format!("pmin={}&", min));
        }
        if let Some(max) = params.pmax {
            query.push_str(&format!("pmax={}&", max));
        }
        if let Some(title) = &params.title {
            query.push_str(&format!("title={}&", urlencoding::encode(title)));
        }
        query
    }
}
