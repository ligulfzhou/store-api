-- 单位管理
create table cates
(
    id        serial,
    name      text    not null default '', -- 类名
    cate_type integer not null default 0,  -- 大类小类， 0 大类， 1小类，再变大，则更小
    parent_id integer not null default 0   -- 父类
);

-- 确认需不需要
create table goods
(
    id       serial,
    goods_no text not null default '' -- 货号
);

--
create table items
(
    id          serial,
    brand       text      not null default '',   -- 品牌
    cates1_id   integer   not null default 0,    -- 大类ID
    cates2_id   integer   not null default 0,    -- 小类ID
    goods_no    text      not null default '',   -- 货号
    color       text      not null default '',   -- 颜色
    name        text      not null default '',   -- 产品名称
    size        text      not null default '',   -- 规格
    unit        text      not null default '',   -- 单位
    barcode     text      not null default '',   -- 条码
    sell_price  integer   not null default 0,    -- 标准售价
    buy_price   integer   not null default 0,    -- 进货价
    create_time TIMESTAMP not null default now() -- 创建时间
);

--     code      text    not null default '', -- 货号

-- 部门
create table departments
(
    id    serial,
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
    id            serial,
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


create table customers
(
    id          serial,
    customer_no text not null default '', -- 客户编号
    name        text not null default '', -- 名称
    address     text not null default '', -- 地址
    phone       text not null default '', -- 电话
    notes       text not null default ''  -- 备注
);
create unique index uniq_customers_customer_no on customers (customer_no);
