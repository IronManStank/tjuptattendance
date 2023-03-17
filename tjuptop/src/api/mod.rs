//! 豆瓣 API & 自定义 API

pub mod doubanapi;
pub mod myapi;

#[cfg(test)]
mod test_api {
    use super::*;
    use pretty_assertions::assert_eq;

    /// 测试：豆瓣API获取图片信息
    #[tokio::test]
    async fn test_douban_api() {
        let url = "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg";
        let lst = doubanapi::DouBanApi::get_answer("三体").await.unwrap();
        assert!(lst.len() >= 5);
        let mut lst_with_len = doubanapi::DouBanApi::get_more_info_try(lst)
            .await
            .into_iter();
        let santi = lst_with_len.next().unwrap();
        assert_eq!(santi.img_len(), 17075);
        assert_eq!(santi.title(), "三体");
        assert_eq!(santi.id(), "26647087");
        assert_eq!(santi.img_url(), url);
        assert_eq!(lst_with_len.next().unwrap().id(), "34444648");
    }
}
