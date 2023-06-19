use anypack::sync_url_fn;
use axum::extract::Host;
use tower_cookies::Cookies;

use crate::R;

// use anypack::url_fn;
// use xxpg::Q;
//
// Q!(
//     img_li:
//     SELECT task.id,hash::bytea,w,h,star,laugh FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 600
// );

sync_url_fn!(post(host: Host, cookies: Cookies) {
    client_id(host, &cookies);

    1
});
