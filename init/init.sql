CREATE TABLE IF NOT EXISTS seen (
uid BIGINT UNSIGNED NOT NULL,
cid TINYINT UNSIGNED NOT NULL,
rid BIGINT UNSIGNED NOT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, cid, rid)
);

CREATE TABLE IF NOT EXISTS log_n (
aid TINYINT UNSIGNED NOT NULL,
cid TINYINT UNSIGNED NOT NULL,
rid BIGINT UNSIGNED NOT NULL,
n BIGINT UNSIGNED NOT NULL,
ts TIMESTAMP(0) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (aid, cid, rid)
);

# 推荐的点击链
CREATE TABLE IF NOT EXISTS rec_chain (
uid BIGINT UNSIGNED NOT NULL,
aid TINYINT UNSIGNED NOT NULL,
cid TINYINT UNSIGNED NOT NULL,
rid BIGINT UNSIGNED NOT NULL,
rcid TINYINT UNSIGNED NOT NULL,
rrid BIGINT UNSIGNED NOT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, aid, cid, rid, rcid, rrid)
);

CREATE TABLE IF NOT EXISTS log (
uid BIGINT UNSIGNED NOT NULL,
aid TINYINT UNSIGNED NOT NULL,
cid TINYINT UNSIGNED NOT NULL,
rid BIGINT UNSIGNED NOT NULL,
q BIGINT UNSIGNED NOT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, aid, cid, rid, q)
);

CREATE TABLE IF NOT EXISTS q (
id TIMESTAMP(0) NOT NULL,
q STRING NOT NULL,
TIME INDEX (id),
PRIMARY KEY (q)
);
