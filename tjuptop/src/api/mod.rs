//! 豆瓣 API & 自定义 API

pub mod myapi;

use util::{model::Answer, CLIENT};

use crate::error::Error;

/// 通过豆瓣 API 获得 Answer
///
/// ## 注意：
/// - 返回的 `Answers` 不携带 `img_len`
pub async fn ask_douban_api(title: &str) -> Result<Vec<Answer>, Error> {
    Ok(CLIENT
        .get("https://movie.douban.com/j/subject_suggest")
        .query(&[("q", title)])
        .send()
        .await?
        .json()
        .await?)
}

/// 尝试将无 `img_len` 的 `Answer` 转化为有 `img_len` 的 Answer。并且，保持原有顺序
pub async fn get_img_len_try(lst: Vec<Answer>) -> Vec<Answer> {
    let mut answer = Vec::with_capacity(6);
    for i in lst.into_iter() {
        answer.push(i.get_len_try().await);
    }
    answer
}

#[cfg(test)]
mod test_api {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_douban_api() {
        let url = "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg";
        let lst = ask_douban_api("三体").await.unwrap();
        assert!(lst.len() >= 5);
        let mut lst_with_len = get_img_len_try(lst).await.into_iter();
        let santi = lst_with_len.next().unwrap();
        assert_eq!(santi.img_len(), 17075);
        assert_eq!(santi.title(), "三体");
        assert_eq!(santi.id(), "26647087");
        assert_eq!(santi.img_url(), url);
        assert_eq!(lst_with_len.next().unwrap().id(), "34444648");
    }
}
