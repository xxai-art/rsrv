use anypack::sync_url_fn;
use tower_cookies::Cookies;
use xkv::fred::interfaces::HashesInterface;

use crate::R;

// use anypack::url_fn;
// use xxpg::Q;
//
// Q!(
//     img_li:
//     SELECT task.id,hash::bytea,w,h,star,laugh FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 600
// );

const TOKEN_LEN: usize = 8;

static mut SK: [u8; 32] = [0; 32];

sync_url_fn!(post(cookies: Cookies) {
    let sk:Vec<u8> = R.get().unwrap().hget("conf","SK").await?;
    dbg!(sk.len() );
    if let Some(c) = cookies.get("I") {
        let c = xxai::cookie_decode(&c.value())?;
        let token = &c[..TOKEN_LEN];
        let client = &c[TOKEN_LEN..];
        dbg!(token, client);
    }
    1
});
