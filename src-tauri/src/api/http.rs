use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    Client,
};

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use http_cache_reqwest::{Cache, CacheMode, CACacheManager, HttpCache, HttpCacheOptions};
use tauri::api::path::cache_dir;


#[derive(Debug)]
pub struct Context {
    pub client: ClientWithMiddleware,
}

impl Context {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
            AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
            )
            .expect("Failed to create user agent header"),
        );
        headers.insert(ACCEPT, HeaderValue::from_str("text/html,application/xhtml+xml,\
        application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;\
        v=b3;q=0.7").expect("Failed to create accept header"));
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_str("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6")
                .expect("Failed to create accept language header"),
        );
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_str("gzip, deflate, br, zstd")
                .expect("Failed to create accept encoding header"),
        );
        headers.insert("sec-ch-ua", HeaderValue::from_str("\"Chromium\";v=\"94\", \";Not A Brand\";v=\"99\"")
            .expect("Failed to create sec-ch-ua header"));
        headers.insert("sec-ch-ua-mobile", HeaderValue::from_str("?0")
            .expect("Failed to create sec-ch-ua-mobile header"));
        headers.insert("sec-ch-ua-platform", HeaderValue::from_str("\"macOS\"")
            .expect("Failed to create sec-ch-ua-platform header"));
        headers.insert("sec-fetch-dest", HeaderValue::from_str("empty")
            .expect("Failed to create sec-fetch-dest header"));
        headers.insert("sec-fetch-mode", HeaderValue::from_str("cors")
            .expect("Failed to create sec-fetch-mode header"));
        headers.insert("sec-fetch-site", HeaderValue::from_str("same-origin")
            .expect("Failed to create sec-fetch-site header"));
        
        // 获取系统缓存目录
        let cache_path = cache_dir()
            .map(|p| p.join("wallhaven_rs").join("http-cache"))
            .unwrap_or_else(|| {
                // 如果获取失败，使用临时目录
                std::env::temp_dir().join("wallhaven_rs").join("http-cache")
            });
        
        // 确保缓存目录存在
        if let Err(e) = std::fs::create_dir_all(&cache_path) {
            eprintln!("创建缓存目录失败: {}", e);
        }
        
        println!("HTTP 缓存目录: {:?}", cache_path);
        
        Self {
            client: ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::ForceCache,
                    manager: CACacheManager {
                        path: cache_path,
                    },
                    options: HttpCacheOptions::default(),
                }))
                .build()
        }
    }
    
    pub fn http_client(&self) -> &ClientWithMiddleware {
        &self.client
    }
}


impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
