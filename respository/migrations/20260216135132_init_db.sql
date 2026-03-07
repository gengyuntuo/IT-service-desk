CREATE SCHEMA IF NOT EXISTS it_service;

-- 用户表
DROP TABLE IF EXISTS it_service.itsd_users;
CREATE TABLE IF NOT EXISTS it_service.itsd_users
(
    id              Serial4 PRIMARY KEY,
    username        TEXT UNIQUE NOT NULL,
    password_hashed TEXT        NOT NULL,
    email           TEXT UNIQUE NOT NULL,
    role            TEXT        NOT NULL,
    is_active       BOOLEAN     NOT NULL     DEFAULT TRUE,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at      TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX IF NOT EXISTS itsd_users_username_uindex ON it_service.itsd_users (username);
CREATE UNIQUE INDEX IF NOT EXISTS itsd_users_email_uindex ON it_service.itsd_users (email);
CREATE UNIQUE INDEX IF NOT EXISTS itsd_users_id_uindex ON it_service.itsd_users (id);
COMMENT ON COLUMN it_service.itsd_users.id IS 'User ID';


-- 工单表
-- DROP TABLE IF EXISTS it_service.itsd_tickets;
CREATE TABLE IF NOT EXISTS it_service.itsd_tickets
(
    id              SERIAL8 PRIMARY KEY,
    title           TEXT NOT NULL,
    description     TEXT,
    extra_data      TEXT,
    attachments     TEXT,
    category        TEXT NOT NULL,
    status          TEXT NOT NULL,
    priority        TEXT NOT NULL,
    apply_user_id   INT8 NOT NULL,
    approve_user_id INT8 NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at      TIMESTAMP WITH TIME ZONE DEFAULT now(),
    finished_at     TIMESTAMP WITH TIME ZONE
);

-- 状态：Draft(草稿), Pending(审批中), Approved(通过), Rejected(驳回), Canceled(撤回)。

CREATE INDEX IF NOT EXISTS itsd_tickets_id_index ON it_service.itsd_tickets (id);
CREATE INDEX IF NOT EXISTS itsd_tickets_status_index ON it_service.itsd_tickets (status);
CREATE INDEX IF NOT EXISTS itsd_tickets_priority_index ON it_service.itsd_tickets (priority);

COMMENT ON TABLE it_service.itsd_tickets IS '申请工单表';

-- 工单日志表
CREATE TABLE IF NOT EXISTS it_service.itsd_ticket_logs
(
    id          serial4 PRIMARY KEY,
    ticket_id   bigint NOT NULL,
    message     text   NOT NULL,
    operator_id int    NOT NULL,
    created_at  timestamp DEFAULT now()
);

COMMENT ON COLUMN it_service.itsd_ticket_logs.id IS 'Ticket Log ID';
COMMENT ON COLUMN it_service.itsd_ticket_logs.ticket_id IS 'Ticket ID';
COMMENT ON COLUMN it_service.itsd_ticket_logs.message IS 'Ticket Log Message';
COMMENT ON COLUMN it_service.itsd_ticket_logs.created_at IS 'Ticket Log Created At';
COMMENT ON TABLE it_service.itsd_ticket_logs IS 'Ticket Logs';

CREATE TABLE IF NOT EXISTS it_service.itsd_cloud_accounts
(
    id          SERIAL4 PRIMARY KEY,
    name        TEXT    NOT NULL,
    description TEXT,
    provider    TEXT    NOT NULL,
    region      TEXT    NOT NULL,
    access_key  TEXT    NOT NULL,
    secret_key  TEXT    NOT NULL,
    is_active   BOOLEAN NOT NULL         DEFAULT TRUE,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE IF NOT EXISTS it_service.itsd_cloud_resources
(
    id             SERIAL8 PRIMARY KEY,
    cloud_account  INT8 NOT NULL,
    resource_id    TEXT NOT NULL,
    resource_type  TEXT NOT NULL,
    resource_name  TEXT NOT NULL,
    resource_state TEXT NOT NULL,
    resource_data  TEXT,
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT now()
);
