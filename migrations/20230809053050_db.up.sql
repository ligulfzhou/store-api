-- 单位管理
-- create table cates
-- (
--     id          serial,
--     index       integer not null default 0,
--     name        text    not null default '', -- 类名
--     cate_type   integer not null default 0,  -- 大类小类， 0 大类， 1小类，再变大，则更小
--     parent_name text    not null default ''  -- 父类
-- );

-- 客户
create table customers
(
    id          serial PRIMARY KEY,
    customer_no text      not null default '',   -- 客户编号
    ty_pe       integer   not null default 1,    -- 客户类别 (1: 普通客户，2: VIP客户)
    name        text      not null default '',   -- 名称
    head        text      not null default '',   -- 负责人
    address     text      not null default '',   -- 地址
    email       text      not null default '',   -- email
    birthday    date,                            -- 生日🎂
    qq          text      not null default '',   -- qq
    phone       text      not null default '',   -- 电话
    notes       text      not null default '',   -- 备注
    create_time TIMESTAMP not null default now() -- 创建时间
);
create index idx_customers_type on customers (ty_pe);


-- 类别
create table cates
(
    id        serial PRIMARY KEY,
    index     integer not null default 0,
    name      text    not null default '', -- 类名
    sub_cates text[] not null default '{}' -- 子类
);

-- todo: for test...
insert into cates (index, name, sub_cates)
values (1, '大类1', array['小类1', '小类2']);

-- 确认需不需要
-- create table goods
-- (
--     id       serial,
--     goods_no text not null default '' -- 货号
-- );

--
create table items
(
    id           serial PRIMARY KEY,
    images       text[] not null default '{}',           -- 商品图片
    cates1       text             not null default '',   -- 大类ID
    cates2       text             not null default '',   -- 小类ID
    brand        text             not null default '',   -- 品牌
    supplier     text             not null default '',   -- 供应商
    material     text             not null default '',   -- 材质
    pcs          integer          not null default 0,    -- pcs件数
    weight       double precision not null default 0.0,  -- 重量
    goods_no     text             not null default '',   -- 货号
    color        text             not null default '',   -- 颜色
    name         text             not null default '',   -- 产品名称
    english_name text             not null default '',   -- 英文名
    size         text             not null default '',   -- 规格
    unit         text             not null default '',   -- 单位
    barcode      text             not null default '',   -- 条码
    description  text             not null default '',   -- 描述
    notes        text             not null default '',   -- 备注
    sell_price   integer          not null default 0,    -- 标准售价
    buy_price    integer          not null default 0,    -- 进货价
    create_time  TIMESTAMP        not null default now() -- 创建时间
);

--     code      text    not null default '', -- 货号

-- 部门
create table departments
(
    id    serial PRIMARY KEY,
    name  text not null default '',       -- 部门名称
    steps integer[] not null default '{}' -- 流程位续
);

insert into departments (name, steps)
values ('业务部', '{1}');
insert into departments (name, steps)
values ('仓库部', '{2}');

-- 账号
create table accounts
(
    id            serial PRIMARY KEY,
    name          text    not null default '',
    account       text    not null default '',
    password      text    not null default '',
    department_id integer not null default 0
);
insert into accounts (name, account, password, department_id)
values ('业务test', 'test', 'test', 1);
insert into accounts (name, account, password, department_id)
values ('业务小红', 'yewuxiaobai', 'yewuxiaobai', 1);
insert into accounts (name, account, password, department_id)
values ('仓库小黄', 'cangkuxiaohuang', 'cangkuxiaohuang', 2);


create table orders
(
    id          serial PRIMARY KEY,
    account_id  integer   not null default 0,    -- 操作人
    customer_id integer   not null default 0,    --
    order_no    text      not null default '',   --
    create_time TIMESTAMP not null default now() -- 创建时间
);
create index idx_orders_order_no on orders (order_no);

create table order_items
(
    id          serial PRIMARY KEY,
    index integer not null default 0,
    item_id integer not null default 0,
    count integer not null default 0,
    price integer not null default 0
)



