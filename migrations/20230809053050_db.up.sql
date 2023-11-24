-- 类别管理
create table cates
(
    id          serial,
    index       integer   not null default 0,    -- 序号
    name        text      not null default '',   -- 类名
    cate_type   integer   not null default 0,    -- 大类小类， 0 大类， 1小类，再变大，则更小
    parent_id   integer   not null default 0,    -- 父类
    create_time TIMESTAMP not null default now() -- 创建时间
);

-- 客户
create table customers
(
    id          serial PRIMARY KEY,
    name        text      not null default '',   -- 名称
    head        text      not null default '',   -- 负责人
    phone       text      not null default '',   -- 电话
    birthday    date,                            -- 生日🎂
    email       text      not null default '',   -- email
    ty_pe       integer   not null default 1,    -- 客户类别 (1: 普通客户，2: VIP客户)
    address     text      not null default '',   -- 地址
    notes       text      not null default '',   -- 备注
    create_time TIMESTAMP not null default now() -- 创建时间
);
create unique index uniq_customers_name on customers (name);
create index idx_customers_type on customers (ty_pe);

-- 产品
create table items
(
    id          serial PRIMARY KEY,
    images      text[] not null default '{}',    -- 商品图片
    name        text      not null default '',   -- 名称
    size        text      not null default '',   -- 规格
    color       text      not null default '',   -- 颜色
    cate1_id    integer   not null default 0,    -- 大类ID
    cate2_id    integer   not null default 0,    -- 小类ID
    unit        text      not null default '',   -- 单位
    price       integer   not null default 0,    -- 标准售价
    cost        integer   not null default 0,    -- 成本
    notes       text      not null default '',   -- 备注
    number      text      not null default '',   -- 编号
    barcode     text      not null default '',   -- 条码
    create_time TIMESTAMP not null default now() -- 创建时间
);
create index idx_items_number on items (number);
create index idx_items_barcode on items (barcode);
create index idx_items_cates on items (cate1_id, cate2_id);

create table item_inout
(
    id                serial PRIMARY KEY,
    account_id        integer   not null default 0,
    item_id           integer   not null default 0,
    count             integer   not null default 0,
    in_true_out_false bool      not null default true,
    via               text      not null default '', -- form/excel/order
    order_id          integer   not null default 0,
    create_time       TIMESTAMP not null default now()
);

create index idx_item_inout_account_create_time on item_inout (account_id);


-- 库存胚
create table embryos
(
    id          serial PRIMARY KEY,
    images      text[] not null default '{}',    -- 商品图片
    name        text      not null default '',   -- 名称
    color       text      not null default '',   -- 颜色
    unit        text      not null default '',   -- 单位
    notes       text      not null default '',   -- 备注
    number      text      not null default '',   -- 编号
    create_time TIMESTAMP not null default now() -- 创建时间
);


create table embryo_inout
(
    id                serial PRIMARY KEY,
    account_id        integer   not null default 0,
    embryo_id         integer   not null default 0,
    count             integer   not null default 0,
    in_true_out_false bool      not null default true,
    via               text      not null default '', -- form/excel
    create_time       TIMESTAMP not null default now()
);


-- 账号
create table accounts
(
    id          serial PRIMARY KEY,
    name        text      not null default '',
    account     text      not null default '',
    password    text      not null default '',
    create_time TIMESTAMP not null default now() -- 创建时间
);

-- todo: test
insert into accounts (name, account, password)
values ('测试账号', 'test', 'test');

-- 订单
create table orders
(
    id          serial PRIMARY KEY,
    account_id  integer   not null default 0,    -- 操作人
    customer_id integer   not null default 0,    --
    order_no    text      not null default '',   --
    create_time TIMESTAMP not null default now() -- 创建时间
);
create index idx_orders_order_no on orders (order_no);

-- 订单商品
create table order_items
(
    id          serial PRIMARY KEY,
    order_id    integer   not null default 0,
    index       integer   not null default 0,
    item_id     integer   not null default 0,
    count       integer   not null default 0,
    price       integer   not null default 0,
    create_time TIMESTAMP not null default now() -- 创建时间
);
create index idx_order_items_order_id on order_items (order_id);
create index idx_order_items_item_id on order_items (item_id);

-- 配置信息
create table global_settings
(
    id       serial PRIMARY KEY,
    units    text[] not null default '{}', -- 产品单位
    accounts text[] not null default '{}'  -- 收款账号
);

-- todo: test
insert into global_settings (units, accounts)
values (ARRAY['个', '串', '只', '支'], ARRAY['现金', '支付宝', '微信', '银行卡']);

create table color_settings
(
    id          serial PRIMARY KEY,
    color       text      not null default '',
    value       integer   not null default 0,
    create_time TIMESTAMP not null default now() -- 创建时间
);
create unique index uniq_color_setting_color on color_settings (color);
insert into color_settings(color, value)
values ('金', 1),
       ('14K金', 1),
       ('18K金', 1),
       ('钢', 2);


create table customer_types
(
    id          serial PRIMARY KEY,
    ty_pe       text      not null default '',
    create_time timestamp not null default now()
);
create unique index uniq_customer_types_type on customer_types (ty_pe);
insert into customer_types (ty_pe)
values ('普通客户'),
       ('VIP客户');