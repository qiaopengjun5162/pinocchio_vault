-- 1. 存储存款事件
CREATE TABLE IF NOT EXISTS deposits (
    id TEXT PRIMARY KEY,
    owner TEXT,
    vault TEXT,
    amount BIGINT,
    block_number BIGINT
);

-- 2. 存储取款事件
CREATE TABLE IF NOT EXISTS withdraws (
    id TEXT PRIMARY KEY,
    owner TEXT,
    vault TEXT,
    block_number BIGINT
);

-- 3. 进度追踪表
CREATE TABLE IF NOT EXISTS cursors (
    id         TEXT PRIMARY KEY,
    cursor     TEXT,
    block_num  BIGINT,
    block_id   TEXT
);

-- 4. 区块历史记录 (Sink 工具处理分叉必须表)
CREATE TABLE IF NOT EXISTS substreams_history (
    id         TEXT PRIMARY KEY,
    at         TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    block_num  BIGINT,
    block_id   TEXT,
    parent_id  TEXT,
    module_hash TEXT
);
