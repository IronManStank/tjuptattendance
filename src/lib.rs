mod error;
pub use error::Error;

mod util;
pub use util::{
    doubandata::{DouBanData, RawDouBanData, API},
    get_east_eight_now,
    user::User,
    OFFSET,
};

mod attimpl;
