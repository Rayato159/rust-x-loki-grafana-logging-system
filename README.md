# Rust x Loki Logging System 💀🦀

Welcome to the **most giga-chad** logging system built with **Rust + Ntex + Loki + Grafana**.

## 🪄 Demo

![demo](/screenshots/demo.png)

## 🧐 How It Works

![architecture](/screenshots/architecture.png)

## ⚡ Tech Stack

- 💎 Rust (Ntex)
- 📜 Loki
- 📊 Grafana

## 🛠️ Setup Guide (a.k.a. How to Not Screw Up)

### 0. Create a `.env` file in the root directory with the following variables:

```env
LOKI_URL = http://localhost:3100
LOKI_JOB_NAME = backend
LOKI_SERVICE_NAME = weapons-service
```

### 1. Clone This God-Tier Repo

```sh
git clone https://github.com/YOUR-USERNAME/rust-loki-logging.git
cd rust-loki-logging
```

### 2. Start Loki & Grafana (Because Logs Don't Visualize Themselves)

```sh
docker-compose up -d
```

- **Grafana:** [http://localhost:3000](http://localhost:3000) (admin/admin)
- **Loki:** Runs on [http://localhost:3100](http://localhost:3100)

### 3️⃣ Run The Rust API Like a Pro

```sh
cargo run
```

### 4️⃣ Test The Logs

```sh
curl -X POST "http://localhost:8080/weapons" -d '{"name": "Excalibur", "damage": 100}' -H "Content-Type: application/json"
```

Then head to Grafana → Explore → Loki, and **witness greatness**. 🏆
