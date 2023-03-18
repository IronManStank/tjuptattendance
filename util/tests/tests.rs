use std::sync::Arc;

use util::model::doubandata::DouBanData;
use util::model::poster::Poster;

#[tokio::test]
async fn test_check_answer() {
    let mut data = DouBanData::new(
        "1231233".into(),
        "title".into(),
        "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg".into(),
        None,
    );

    let poster = Poster::new(
        "https://img2.doubanio.com/view/photo/s_ratio_poster/public/p2886492021.jpg".into(),
        Some(bytes::Bytes::new()),
        None,
    )
    .unwrap();

    let poster = Arc::new(poster);

    assert!(!data.have_additional_info());
    assert!(data.check_img_format().unwrap_or(false));

    assert!(!poster.is_quick_check());

    data.set_len().await.unwrap();

    assert!(data.have_additional_info());
    assert!(data.check_img_format().unwrap_or(false));

    assert!(data.is_answer(poster).await.unwrap());
}
