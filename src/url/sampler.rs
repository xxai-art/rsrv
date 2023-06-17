use anypack::{url_fn, VecAny};
use xxpg::Q;

Q!(
  sampler_id_name :
    SELECT id::bigint::oid,name FROM img.sampler;
);

url_fn!(get() {
    let li = sampler_id_name().await?;
    let mut vec = VecAny::new();
    for i in li {
        let id = i.get::<_, u32>(0);
        let name = i.get::<_, &str>(1).to_string();
        vec.push(id);
        vec.push(name);
    }
    vec
});
