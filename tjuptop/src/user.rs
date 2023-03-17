//! TJUPT 用户

use anyhow::anyhow;
use reqwest::cookie::CookieStore as reqwestCookieStore;
use reqwest_cookie_store::{CookieStore, CookieStoreRwLock};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    hash::Hash,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::error::Error;

/// TJUPT User
///
/// `name` 相同的 `User` 将会被视为同一个
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) name: String,
    pub(crate) pwd: String,
    #[serde(skip)]
    pub(crate) cookie: CookieStoreRwLock,
    #[serde(skip)]
    pub(crate) cookie_path: Option<PathBuf>,
}

impl User {
    /// cookie file name "`name`_cookie.json"
    fn cookie_file_name(name: &str) -> String {
        format!("{}_cookie.json", name)
    }

    /// 如果 `cookie_dir` 设置为 `None` 则表示不会保存cookie
    pub fn new(name: String, pwd: String, cookie_dir: Option<PathBuf>) -> Self {
        Self {
            cookie_path: cookie_dir.map(|d| d.join(Self::cookie_file_name(&name))),
            name,
            pwd,
            cookie: CookieStoreRwLock::default(),
        }
    }

    /// 修改 cookie_path
    pub fn set_cookie_path(&mut self, cookie_dir: PathBuf) {
        self.cookie_path = Some(cookie_dir.join(Self::cookie_file_name(&self.name)));
    }

    /// 保存cookies
    /// 如果 `cookie_path` 为 `None` 则不会尝试读取 cookie
    pub fn save_cookie(&self) -> Result<(), Error> {
        let Some(ref cookie_path) = self.cookie_path else {
            return Ok(());
        };

        let mut file = File::create(cookie_path)?;
        {
            let lock = self.cookie.read().map_err(|e| anyhow!("内部错误 {e}"))?;
            lock.save_json(&mut file)
                .map_err(|e| anyhow!("无法保存cookie到 {}，{}", cookie_path.display(), e))?;
        }

        Ok(())
    }

    /// 加载 cookies
    pub fn load_cookie(&self) -> Result<(), Error> {
        let Some(ref cookie_path) = self.cookie_path else {
            return Ok(());
        };

        if cookie_path.is_file() {
            let file = File::open(cookie_path).map(BufReader::new)?;
            let cookie = CookieStore::load_json(file)
                .map_err(|e| anyhow!("无法读取cookie {}，{}", cookie_path.display(), e))?;
            {
                let mut lock = self.cookie.write().map_err(|e| anyhow!("内部错误 {e}"))?;
                *lock = cookie;
            }
            Ok(())
        } else {
            Err(Error::CookiesError)
        }
    }

    /// cookie path
    pub fn cookie_path(&self) -> Option<&Path> {
        self.cookie_path.as_deref()
    }
}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl std::ops::Deref for User {
    type Target = CookieStoreRwLock;
    fn deref(&self) -> &Self::Target {
        &self.cookie
    }
}

impl std::ops::DerefMut for User {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cookie
    }
}

impl reqwestCookieStore for User {
    fn cookies(&self, url: &url::Url) -> Option<reqwest::header::HeaderValue> {
        self.cookie.cookies(url)
    }

    fn set_cookies(
        &self,
        cookie_headers: &mut dyn Iterator<Item = &reqwest::header::HeaderValue>,
        url: &url::Url,
    ) {
        self.cookie.set_cookies(cookie_headers, url)
    }
}

impl Drop for User {
    fn drop(&mut self) {
        if let Err(e) = self.save_cookie() {
            eprintln!("{} 保存 cookie 失败 {}", self.name, e);
        }
    }
}

#[cfg(test)]
mod test_user_cookie {
    use super::*;
    use reqwest::ClientBuilder;
    use std::{fs::read_to_string, sync::Arc};
    use tempdir::TempDir;

    /// 测试：cookie 的读取和写入
    #[tokio::test]
    async fn test_cookie() {
        let dir = TempDir::new("test_cookie").unwrap();
        let dir_path = dir.path();

        let mut user = User::new("name".into(), "pwd".into(), None);
        // 设置cookie path
        user.set_cookie_path(dir_path.to_path_buf());
        let cookie_path = user.cookie_path().unwrap().to_path_buf();

        let user_c = Arc::new(user);

        let client = ClientBuilder::new()
            .cookie_provider(user_c.clone())
            .build()
            .unwrap();
        // 访问一些网页，来测试保存cookie
        client.get("https://www.bing.com").send().await.unwrap();
        // 手动保存 cookie
        user_c.save_cookie().unwrap();
        // 检查cookie文件已经生成
        assert!(user_c.cookie_path().unwrap().is_file());
        // cookie 文件 内容存在
        let content = read_to_string(&cookie_path).unwrap();

        assert!(content.lines().count() >= 3);
        assert!(content.contains("bing."));

        // 读取 cookie
        user_c.load_cookie().unwrap();

        // 换一个网页更新 cookie
        client.get("https://www.baidu.com").send().await.unwrap();

        user_c.save_cookie().unwrap();

        let content_two = read_to_string(&cookie_path).unwrap();
        // 此时为两个网页的 cookie 综合
        assert!(content_two.contains("baidu.com"));
        assert!(content_two.contains("bing."));

        drop(user_c);
        drop(client);
        dir.close().unwrap();
    }
}
