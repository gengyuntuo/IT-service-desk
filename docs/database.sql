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
    id          Serial4 PRIMARY KEY,
    title       text NOT NULL,
    description text NOT NULL,
    status      text NOT NULL,
    priority    text NOT NULL,
    created_at  timestamp DEFAULT now(),
    updated_at  timestamp DEFAULT now(),
    user_id     int  NOT NULL
);

CREATE INDEX IF NOT EXISTS itsd_tickets_user_id_index ON it_service.itsd_tickets (user_id);
CREATE INDEX IF NOT EXISTS itsd_tickets_id_index ON it_service.itsd_tickets (id);
CREATE INDEX IF NOT EXISTS itsd_tickets_status_index ON it_service.itsd_tickets (status);
CREATE INDEX IF NOT EXISTS itsd_tickets_priority_index ON it_service.itsd_tickets (priority);

COMMENT ON COLUMN it_service.itsd_tickets.id IS 'Ticket ID';
COMMENT ON COLUMN it_service.itsd_tickets.title IS 'Ticket Title';
COMMENT ON COLUMN it_service.itsd_tickets.description IS 'Ticket Description';
COMMENT ON COLUMN it_service.itsd_tickets.status IS 'Ticket Status';
COMMENT ON COLUMN it_service.itsd_tickets.priority IS 'Ticket Priority';
COMMENT ON COLUMN it_service.itsd_tickets.created_at IS 'Ticket Created At';
COMMENT ON COLUMN it_service.itsd_tickets.updated_at IS 'Ticket Updated At';
COMMENT ON COLUMN it_service.itsd_tickets.user_id IS 'User ID';
COMMENT ON TABLE it_service.itsd_tickets IS 'Tickets';


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

