CREATE SCHEMA IF NOT EXISTS it_service;

-- 用户表
CREATE TABLE IF NOT EXISTS it_service.itsd_users
(
    id       Serial4 PRIMARY KEY,
    username text NOT NULL,
    password text NOT NULL,
    email    text NOT NULL,
    role     text NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS itsd_users_username_uindex ON it_service.itsd_users (username);
CREATE UNIQUE INDEX IF NOT EXISTS itsd_users_email_uindex ON it_service.itsd_users (email);
CREATE UNIQUE INDEX IF NOT EXISTS itsd_users_id_uindex ON it_service.itsd_users (id);
COMMENT ON COLUMN it_service.itsd_users.id IS 'User ID';
COMMENT ON COLUMN it_service.itsd_users.username IS 'Username';
COMMENT ON COLUMN it_service.itsd_users.password IS 'Password';
COMMENT ON COLUMN it_service.itsd_users.email IS 'Email';
COMMENT ON COLUMN it_service.itsd_users.role IS 'Role';
COMMENT ON TABLE it_service.itsd_users IS 'Users';

-- 工单表
CREATE TABLE IF NOT EXISTS it_service.itsd_tickets
(
    id              SERIAL8 PRIMARY KEY,
    title           TEXT NOT NULL,
    description     TEXT,
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
    id         serial4 PRIMARY KEY,
    ticket_id  int  NOT NULL,
    message    text NOT NULL,
    created_at timestamp DEFAULT now()
);

COMMENT ON COLUMN it_service.itsd_ticket_logs.id IS 'Ticket Log ID';
COMMENT ON COLUMN it_service.itsd_ticket_logs.ticket_id IS 'Ticket ID';
COMMENT ON COLUMN it_service.itsd_ticket_logs.message IS 'Ticket Log Message';
COMMENT ON COLUMN it_service.itsd_ticket_logs.created_at IS 'Ticket Log Created At';
COMMENT ON TABLE it_service.itsd_ticket_logs IS 'Ticket Logs';

