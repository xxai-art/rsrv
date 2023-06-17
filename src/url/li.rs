use anypack::{url_fn, VecAny};
use xxpg::Q;

Q!(
  img_li:
      SELECT task.id::bigint,hash::bytea,w::bigint::oid,h::bigint::oid,star::bigint,laugh::bigint FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 600
);

url_fn!(post() {
    let li = img_li().await?;
    let mut vec = VecAny::new();
    for i in li {
        let id:i64 = i.get(0);
        vec.push(id as u64);
        let hash:Vec<u8> = i.get(1);
        vec.push(hash);
        let w:u32 = i.get(2);
        vec.push(w);
        let h:u32 = i.get(3);
        vec.push(h);
        let star:i64 = i.get(4);
        vec.push(star as u64);
        let reply:i64 = i.get(5);
        vec.push(reply as u64);
    }
    vec
});
