use crate::client::MarketClient;
use crate::error::{ApiError, ApiResult};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchParams { pub page: Option<i64>, pub pmin: Option<f64>, pub pmax: Option<f64>, pub title: Option<String>, pub order_by: Option<String>, pub tag_id: Option<Vec<i64>>, pub origin: Option<String> }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemsResponse { pub items: Vec<Value>, #[serde(rename = "totalItems")] pub total_items: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemResponse { pub item: Value, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagsResponse { pub tags: Vec<Value>, pub total: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatsResponse { #[serde(rename = "total_items")] pub total_items: i64, #[serde(rename = "total_sold")] pub total_sold: i64, #[serde(rename = "total_views")] pub total_views: i64, #[serde(skip_serializing_if = "Option::is_none")] pub system_info: Option<Value> }

impl MarketClient {
    fn build_query(&self, params: &SearchParams) -> String {
        let mut q = String::from("?");
        if let Some(v) = params.page { q.push_str(&format!("page={}&", v)); }
        if let Some(v) = params.pmin { q.push_str(&format!("pmin={}&", v)); }
        if let Some(v) = params.pmax { q.push_str(&format!("pmax={}&", v)); }
        if let Some(v) = &params.title { q.push_str(&format!("title={}&", urlencoding::encode(v))); }
        q
    }
    pub async fn search_items(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/{}", self.build_query(&p)), None => "/".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn get_item(&self, item_id: i64) -> ApiResult<ItemResponse> { self.api().execute_json(self.api().get(&format!("/{}", item_id))).await }
    pub async fn search_steam(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/steam{}", self.build_query(&p)), None => "/steam".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn search_origin(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/origin{}", self.build_query(&p)), None => "/origin".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn search_epic(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/epic{}", self.build_query(&p)), None => "/epic".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn search_uplay(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/uplay{}", self.build_query(&p)), None => "/uplay".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn search_battlenet(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/battlenet{}", self.build_query(&p)), None => "/battlenet".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn search_socialclub(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/socialclub{}", self.build_query(&p)), None => "/socialclub".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn search_eaapp(&self, params: Option<SearchParams>) -> ApiResult<ItemsResponse> { let ep = match params { Some(p) => format!("/eaapp{}", self.build_query(&p)), None => "/eaapp".to_string() }; self.api().execute_json(self.api().get(&ep)).await }
    pub async fn get_tags(&self) -> ApiResult<TagsResponse> { self.api().execute_json(self.api().get("/tags")).await }
    pub async fn get_stats(&self) -> ApiResult<StatsResponse> { self.api().execute_json(self.api().get("/stats")).await }
    pub async fn create_item(&self, category_id: i64, title: &str, description: &str, price: i64, currency: &str) -> ApiResult<ItemResponse> { let req = serde_json::json!({"category_id": category_id, "title": title, "description": description, "price": price, "currency": currency}); self.api().execute_json(self.api().post("/").json(&req)).await }
    pub async fn update_item(&self, item_id: i64, title: Option<&str>, price: Option<i64>) -> ApiResult<ItemResponse> { let mut req = serde_json::Map::new(); if let Some(v) = title { req.insert("title".to_string(), Value::String(v.to_string())); } if let Some(v) = price { req.insert("price".to_string(), Value::Number(v.into())); } self.api().execute_json(self.api().put(&format!("/{}", item_id)).json(&req)).await }
    pub async fn delete_item(&self, item_id: i64) -> ApiResult<Value> { self.api().execute(self.api().delete(&format!("/{}", item_id))).await?.json().await.map_err(ApiError::from) }
    pub async fn buy_item(&self, item_id: i64) -> ApiResult<Value> { self.api().execute_json(self.api().post(&format!("/{}", item_id))).await }
}
