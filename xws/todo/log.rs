use std::collections::HashSet;

use anyhow::Result;
use anypack::Any;
use awp::ok;
use axum::body::Bytes;
use client::Client;
use sonic_rs::Value;
use tokio::sync::OnceCell;
use x0::fred::types::Script;
use xxai::z85_decode_u64_li;

use crate::{
  db::rec::rec_by_action,
  C::{
    action::{CLICK, FAV, FAV_RM},
    cid::CID_IMG,
  },
};

static LOG_CID: [u8; 1] = [CID_IMG];
static REC_ACTION: [u8; 2] = [CLICK, FAV];

static QID: OnceCell<Script> = OnceCell::const_new();

pub async fn post(mut client: Client, body: Bytes) -> awp::any!() {
  if let Some(uid) = client.uid().await? {
    let ts = sts::ms();
    let req: Vec<Value> = sonic_rs::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
    let level = req[0].as_u64().unwrap_or(0) as u8; // 内容分级
    let all: Vec<Vec<String>> = req[1..]
      .iter()
      .map(|i| {
        if let Some(li) = i.as_array() {
          li.iter()
            .map(|v| v.as_str().unwrap_or("").to_owned())
            .collect()
        } else {
          vec![]
        }
      })
      .collect();

    let mut to_insert = Vec::with_capacity(all.iter().map(|v| v.len()).sum());
    let mut rec_action = Vec::new();
    let mut rec_chain = Vec::new();
    let mut exist = HashSet::new();

    for li in all {
      if !li.is_empty() {
        let q = xxai::str::low_short(&li[0]);
        let (qid, new) = qid(&q).await?;
        if new {
          trt::spawn!({
            gt::QE(format!("INSERT INTO q (id,q) VALUES ({qid},$1)"), &[&q]).await?;
          });
        }
        for cid_rid_li in &li[1..] {
          let cid_rid_li = z85_decode_u64_li(cid_rid_li)?;
          if !cid_rid_li.is_empty() {
            let action = cid_rid_li[0] as u8;
            macro_rules! to_insert {
              ($cid:ident,$rid:ident) => {{
                let cid = $cid;
                let rid = $rid;
                to_insert.push(format!("({uid},{action},{cid},{rid},{qid},{ts})"));
              }};
            }
            if action == FAV_RM {
              cid_rid_li[1..].chunks(2).for_each(|i| {
                let cid = i[0] as u8;
                if LOG_CID.contains(&cid) {
                  let rid = i[1];
                  let key = (cid, rid);
                  exist.remove(&key);
                  to_insert!(cid, rid);
                  rec_action.retain(|li: &Vec<(u8, u64)>| li[0] != key);
                }
              });
            } else if REC_ACTION.contains(&action) {
              let cid_rid_li = &cid_rid_li[1..];
              let len = cid_rid_li.len();
              if len >= 2 {
                let cid = cid_rid_li[0] as u8;
                if LOG_CID.contains(&cid) {
                  let rid = cid_rid_li[1];
                  to_insert!(cid, rid);
                  let key = (cid, rid);
                  if !exist.contains(&key) {
                    exist.insert(key);
                    let mut action_li = vec![key];
                    if len >= 5 {
                      let mut n_level = cid_rid_li[2];

                      for i in cid_rid_li[3..].chunks(2).map(|i| (i[0] as u8, i[1])) {
                        action_li.push(i);
                        if n_level > 0 {
                          let (pcid, prid) = i;
                          n_level -= 1;
                          rec_chain
                            .push(format!("({uid},{action},{cid},{rid},{pcid},{prid},{ts})"));
                        }
                      }
                    }
                    rec_action.push(action_li);
                  }
                }
              }
            } else {
              cid_rid_li[1..].chunks(2).for_each(|i| {
                let cid = i[0] as u8;
                if LOG_CID.contains(&cid) {
                  let rid = i[1];
                  to_insert!(cid, rid);
                }
              });
            }
          }
        }
      }
    }

    macro_rules! insert {
      ($li:ident,$sql:expr) => {
        if !$li.is_empty() {
          trt::spawn!({
            let li = $li.join(",");
            gt::QE($sql + &li, &[]).await?;
          });
        }
      };
    }

    insert!(
      to_insert,
      "INSERT INTO log (uid,aid,cid,rid,q,ts) VALUES ".to_owned()
    );
    insert!(
      rec_chain,
      "INSERT INTO rec_chain (uid,aid,cid,rid,pcid,prid,ts) VALUES ".to_owned()
    );
    ok!(rec_by_action(level, rec_action))
  } else {
    Ok(Any::Null)
  }
}
