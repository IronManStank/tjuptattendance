use crate::{tjurl, Error, User};
use anyhow::anyhow;
use chrono::prelude::*;
use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, ClientBuilder, Response};
use reqwest_cookie_store::CookieStoreMutex;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

lazy_static! {
    static ref HEADER: HeaderMap = {
        let mut header = HeaderMap::new();
        header.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                    AppleWebKit/537.36 (KHTML, like Gecko) \
                    Chrome/100.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        header
    };
}

#[derive(Debug)]
pub struct AttBot {
    pub(crate) user: User,
    pub(crate) client: Client,
    pub(crate) cookie: Arc<CookieStoreMutex>,
    /// 如果设置了路径则进行保存，如果是None则不保存
    pub(crate) cookie_path: Option<PathBuf>,
}

impl AttBot {
    pub fn new(user: User, cookie_path: Option<PathBuf>) -> Self {
        let cookie = Arc::new(CookieStoreMutex::default());
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .cookie_store(true)
            .cookie_provider(cookie.clone())
            .default_headers(HEADER.clone())
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .unwrap();

        Self {
            client,
            cookie,
            user,
            cookie_path,
        }
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn clinet(&self) -> &Client {
        &self.client
    }

    pub fn clear_cookie(&self) -> Result<(), Error> {
        let mut lock = self.cookie.lock().map_err(|e| anyhow!("{e}"))?;
        lock.clear();
        Ok(())
    }

    pub fn cookie_path(&self) -> &Option<PathBuf> {
        &self.cookie_path
    }

    pub(crate) fn save_cookie_to<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let mut f = File::create(path)?;
        let lock = self.cookie.lock().map_err(|e| anyhow!("{e}"))?;
        lock.save_json(&mut f)
            .map_err(|e| anyhow!("无法保存cookie {}，Err: {}", path.display(), e))?;
        Ok(())
    }

    pub fn save_cookie(&self) -> Result<(), Error> {
        if let Some(ref path) = self.cookie_path {
            self.save_cookie_to(path)?;
        }
        Ok(())
    }

    pub(crate) fn load_cookie_from<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let f = File::open(path).map(std::io::BufReader::new)?;
        let cookie = reqwest_cookie_store::CookieStore::load_json(f).map_err(|e| anyhow!("{e}"))?;
        let mut lock = self.cookie.lock().map_err(|e| anyhow!("{e}"))?;
        *lock = cookie;
        Ok(())
    }

    pub fn load_cookie(&self) -> Result<(), Error> {
        if let Some(ref path) = self.cookie_path {
            self.load_cookie_from(path)?;
        }
        Ok(())
    }

    /// 仅 POST 登录请求，并判断是否登录成功。
    /// 登陆前可能需要 GET 一下登录页。
    /// 重定向到签到页面
    pub async fn login_post(&self) -> Result<Response, Error> {
        let res = self
            .client
            .post(tjurl::TAKELOGIN)
            .form(&[
                ("username", self.user.name()),
                ("password", self.user.pwd()),
                ("logout", "7days"),
                ("returnto", "attendance.php"),
            ])
            .send()
            .await?;

        // 这里无法通过 statuscode 重定向判断
        if res.status().is_success() && !res.url().as_str().contains("login.php") {
            // 登录成功
            Ok(res)
        } else {
            Err(Error::UserVerification)
        }
    }

    /// 立即签到 普通模式
    pub async fn att_now_normal(&self) -> Result<(), Error> {
        todo!()
    }

    #[allow(unused)]
    /// 按照时间点
    pub async fn att_top(&self, t: NaiveDateTime) -> Result<(), Error> {
        todo!()
    }
}

impl Drop for AttBot {
    fn drop(&mut self) {
        if let Err(e) = self.save_cookie() {
            eprintln!("{} 保存 cookie 失败 Err: {}", self.user.name(), e);
        }
    }
}
