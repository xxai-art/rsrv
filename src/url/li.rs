use anypack::{url_fn, VecAny};
use xxpg::Q;

Q!(
  img_li:
      SELECT task.id,hash::bytea,w::bigint::oid,h::bigint::oid,star::bigint,laugh::bigint FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 600
);

url_fn!(post() {
    dbg!(&SQL_IMG_LI.0.get().unwrap().columns());
    let li = img_li().await?;
    let mut vec = VecAny::new();
    for i in li {
        // let mut t = VecAny::new();
        // let id:i64 = i.get(0);
        // t.push(id);
        // let hash:Vec<u8> = i.get(1);
        // t.push(hash);
        // let w:u32 = i.get(2);
        // t.push(w);
        // let h:u32 = i.get(3);
        // t.push(h);
        // let star:i64 = i.get(4);
        // t.push(star);
        // let reply:i64 = i.get(5);
        // t.push(reply);
        // vec.push(t);
    }
    vec
});
