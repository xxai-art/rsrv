use anypack::url_fn;
use axum::extract::Path;
use xxpg::Q1;

Q1!(sampler_name_by_id : SELECT name FROM img.sampler WHERE id=$1::bigint);

url_fn!(get(Path(id): Path<u64>) {
    sampler_name_by_id(&(id as i64)).await?.get::<_, &str>(0).to_owned()
});
