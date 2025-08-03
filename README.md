# Rust x Loki x Otel x Tempo Logging & Tracing System 💀🦀

Welcome to the **most giga-chad** logging & tracing system built with **Rust + Ntex + Loki + Otel + Tempo + Grafana**.

## 🪄 Demo

Otel Tracing

![demo](/screenshots/demo-1.png)

Loki Logging

![demo](/screenshots/demo-2.png)

## 🧐 How It Works

![architecture](/screenshots/architecture.png)

## ⚡ Tech Stack

- 💎 Rust (Ntex)
- 📜 Loki
- 📈 OpenTelemetry
- 🔫 Tempo
- 📊 Grafana

## 🛠️ Setup Guide (a.k.a. How to Not Screw Up)

### 0. Create a `.env` file in the root directory with the following variables:

```env
LOKI_URL = http://localhost:3100
LOKI_JOB_NAME = backend
LOKI_SERVICE_NAME = weapons-service
OTEL_COLLECTOR_URL = http://localhost:4317
```

### 1. Clone This God-Tier Repo

```sh
git clone https://github.com/Rayato159/rust-x-loki-grafana-logging-system.git
cd rust-x-loki-grafana-logging-system
```

### 2. Start Loki & Tempo & Otel & Grafana (Because Logs Don't Visualize Themselves)

```sh
docker-compose up -d
```

- **Grafana:** [http://localhost:3000](http://localhost:3000) (admin/admin)
- **Loki:** Runs on [http://loki:3100](http://loki:3100)
- **Tempo:** Runs on [http://tempo:3200](http://tempo:3200)

### 3. Run The Rust API Like a Pro

```sh
cargo run
```

### 4️. Test The Logs

```sh
curl --location 'http://localhost:8080/weapons' \
--header 'Content-Type: application/json' \
--header 'Cookie: grafana_session=e3f682c7ccb048f9a549ed7b31e96cf8; grafana_session_expiry=1748865774' \
--data '{
    "name": "M4A1",
    "damage": 150
}'
```

Then head to Grafana → Explore → Loki, and **witness greatness**. 🏆
