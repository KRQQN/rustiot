CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,
    name            TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE devices (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    device_type     TEXT,                       -- optional label
    -- How data is received
    protocol        TEXT NOT NULL,              -- 'mqtt', 'http', 'websocket', etc.
    config          JSONB NOT NULL DEFAULT '{}', -- topics, endpoints, auth, payload mapping, etc.
    -- Device authentication
    device_token    TEXT UNIQUE,                -- used by the device when sending data
    is_active       BOOLEAN NOT NULL DEFAULT true,
    last_seen_at    TIMESTAMPTZ,                -- optional, can be updated from Influx or gateway
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (user_id, name)
);

CREATE INDEX idx_devices_user_id ON devices(user_id);
CREATE INDEX idx_devices_token ON devices(device_token) WHERE device_token IS NOT NULL;

-- Expected metrics / values for a device
CREATE TABLE device_metrics (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id       UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    -- Identification of the metric
    key             TEXT NOT NULL,              -- e.g. "temperature", "humidity", "battery_level"
    display_name    TEXT,                       -- "Temperature", "Battery %"
    -- Expected characteristics
    data_type       TEXT NOT NULL DEFAULT 'float',  -- 'float', 'integer', 'boolean', 'string'
    unit            TEXT,                       -- "°C", "%", "V", etc.
    -- Extra metadata
    description     TEXT,
    config          JSONB DEFAULT '{}',         -- thresholds, aggregation hints, colors, etc.
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (device_id, key)
);

CREATE INDEX idx_device_metrics_device_id ON device_metrics(device_id);

CREATE TABLE dashboards (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    is_default      BOOLEAN NOT NULL DEFAULT false,
    layout_config   JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_dashboards_user_id ON dashboards(user_id);

CREATE TABLE panels (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dashboard_id    UUID NOT NULL REFERENCES dashboards(id) ON DELETE CASCADE,
    title           TEXT NOT NULL,
    panel_type      TEXT NOT NULL,              -- 'line', 'gauge', 'stat', 'table', etc.
    position        JSONB NOT NULL DEFAULT '{"x":0,"y":0,"w":4,"h":3}',
    config          JSONB NOT NULL DEFAULT '{}',
    -- Example:
    -- {
    --   "device_id": "uuid",
    --   "metrics": ["temperature", "humidity"],
    --   "time_range": "24h",
    --   "aggregation": "mean",
    --   "refresh_seconds": 30
    -- }
    
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_panels_dashboard_id ON panels(dashboard_id);
