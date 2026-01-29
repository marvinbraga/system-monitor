# Plano de Implementação - System Monitor v2.0
## Arquitetura Distribuída com Backend + Frontend

---

## 📋 Índice

1. [Visão Geral](#visão-geral)
2. [Arquitetura do Sistema](#arquitetura-do-sistema)
3. [Estrutura de Diretórios](#estrutura-de-diretórios)
4. [Fase 1: Preparação](#fase-1-preparação)
5. [Fase 2: Backend - Serviço Coletor](#fase-2-backend---serviço-coletor)
6. [Fase 3: API REST/WebSocket](#fase-3-api-restwebsocket)
7. [Fase 4: Banco de Dados](#fase-4-banco-de-dados)
8. [Fase 5: Cliente TUI](#fase-5-cliente-tui)
9. [Fase 6: Web Dashboard](#fase-6-web-dashboard)
10. [Fase 7: Deploy e Produção](#fase-7-deploy-e-produção)
11. [Tecnologias Utilizadas](#tecnologias-utilizadas)
12. [Cronograma Estimado](#cronograma-estimado)
13. [Testes e Validação](#testes-e-validação)

---

## 🎯 Visão Geral

### Objetivo
Transformar o monitor de sistema monolítico atual em uma arquitetura distribuída com:
- **Backend**: Serviço coletor que roda como daemon
- **Frontend**: Múltiplos clientes (TUI + Web Dashboard)
- **API**: REST + WebSocket para comunicação
- **Persistência**: Banco de dados com histórico de métricas

### Benefícios
- ✅ Separação de responsabilidades
- ✅ Múltiplos clientes simultâneos
- ✅ Histórico de métricas
- ✅ Análise temporal e gráficos
- ✅ Escalabilidade horizontal
- ✅ Deploy independente
- ✅ Acesso remoto via web

---

## 🏗️ Arquitetura do Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                     SERVIÇO COLETOR                         │
│                    (system-monitor-daemon)                   │
│                                                              │
│  ┌──────────────────────────────────────────────────┐      │
│  │              MÓDULOS DE COLETA                   │      │
│  │  ┌─────────┐ ┌─────────┐ ┌──────────┐ ┌──────┐ │      │
│  │  │   CPU   │ │ Memory  │ │   Temp   │ │ USB  │ │      │
│  │  └────┬────┘ └────┬────┘ └────┬─────┘ └───┬──┘ │      │
│  │       │           │            │            │     │      │
│  │  ┌────▼───────────▼────────────▼────────────▼──┐ │      │
│  │  │         AGREGADOR DE MÉTRICAS              │ │      │
│  │  └────────────────────┬────────────────────────┘ │      │
│  └───────────────────────┼──────────────────────────┘      │
│                          ▼                                  │
│           ┌──────────────────────────────┐                 │
│           │   DETECTOR DE ANOMALIAS      │                 │
│           │  - Regras configuráveis      │                 │
│           │  - Machine Learning (futuro) │                 │
│           └──────────────┬───────────────┘                 │
│                          ▼                                  │
│           ┌──────────────────────────────┐                 │
│           │    CAMADA DE PERSISTÊNCIA    │                 │
│           │  - SQLite (dev/single)       │                 │
│           │  - PostgreSQL (prod)         │                 │
│           │  - TimescaleDB (opcional)    │                 │
│           └──────────────┬───────────────┘                 │
│                          ▼                                  │
│           ┌──────────────────────────────┐                 │
│           │         API LAYER            │                 │
│           │  - REST (HTTP)               │                 │
│           │  - WebSocket (Real-time)     │                 │
│           │  - Auth/JWT (opcional)       │                 │
│           └──────────────┬───────────────┘                 │
└──────────────────────────┼─────────────────────────────────┘
                           │
          ┌────────────────┼────────────────┐
          │                │                │
          ▼                ▼                ▼
   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
   │   TUI APP   │  │   WEB APP   │  │  MOBILE APP │
   │             │  │             │  │   (futuro)  │
   │ - crossterm │  │ - React     │  │             │
   │ - ratatui   │  │ - Tailwind  │  │             │
   │ - tokio     │  │ - Chart.js  │  │             │
   └─────────────┘  └─────────────┘  └─────────────┘
```

---

## 📁 Estrutura de Diretórios

```
system-monitor/
├── Cargo.toml                      # Workspace root
├── README.md
├── PLANO_IMPLEMENTACAO_COMPLETO.md
├── docker-compose.yml              # Orquestração
│
├── collector/                      # Backend - Serviço Coletor
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                 # Entry point do daemon
│   │   ├── lib.rs                  # Biblioteca compartilhada
│   │   ├── config.rs               # Configurações
│   │   ├── collectors/             # Módulos de coleta
│   │   │   ├── mod.rs
│   │   │   ├── cpu.rs
│   │   │   ├── memory.rs
│   │   │   ├── temperature.rs
│   │   │   ├── disk.rs
│   │   │   ├── usb.rs
│   │   │   ├── network.rs
│   │   │   └── processes.rs
│   │   ├── detector/               # Detecção de anomalias
│   │   │   ├── mod.rs
│   │   │   ├── rules.rs
│   │   │   └── analyzer.rs
│   │   ├── storage/                # Camada de persistência
│   │   │   ├── mod.rs
│   │   │   ├── models.rs
│   │   │   ├── repository.rs
│   │   │   └── migrations/
│   │   │       └── 001_initial.sql
│   │   └── api/                    # API Layer
│   │       ├── mod.rs
│   │       ├── rest.rs
│   │       ├── websocket.rs
│   │       └── routes.rs
│   ├── config.toml                 # Configuração do daemon
│   └── systemd/
│       └── system-monitor.service
│
├── tui-client/                     # Cliente Terminal
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── api_client.rs           # Cliente HTTP/WS
│   │   ├── ui/
│   │   │   ├── mod.rs
│   │   │   ├── dashboard.rs
│   │   │   ├── cpu_view.rs
│   │   │   ├── memory_view.rs
│   │   │   ├── disk_view.rs
│   │   │   ├── anomalies_view.rs
│   │   │   └── charts.rs
│   │   └── config.rs
│   └── config.toml
│
├── web-frontend/                   # Web Dashboard
│   ├── package.json
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   ├── index.html
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── api/
│   │   │   ├── client.ts
│   │   │   └── websocket.ts
│   │   ├── components/
│   │   │   ├── Dashboard.tsx
│   │   │   ├── CpuChart.tsx
│   │   │   ├── MemoryChart.tsx
│   │   │   ├── TemperatureGauge.tsx
│   │   │   ├── DiskUsage.tsx
│   │   │   ├── AnomalyList.tsx
│   │   │   └── SystemInfo.tsx
│   │   ├── hooks/
│   │   │   ├── useMetrics.ts
│   │   │   └── useWebSocket.ts
│   │   ├── types/
│   │   │   └── metrics.ts
│   │   └── utils/
│   │       └── formatters.ts
│   └── public/
│
├── shared/                         # Código compartilhado
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── types.rs                # Tipos compartilhados
│       └── constants.rs
│
├── scripts/                        # Scripts utilitários
│   ├── build-all.sh
│   ├── run-dev.sh
│   ├── run-prod.sh
│   └── migrate-db.sh
│
└── docs/                           # Documentação
    ├── API.md                      # Documentação da API
    ├── DEPLOY.md                   # Guia de deploy
    └── DEVELOPMENT.md              # Guia de desenvolvimento
```

---

## 📦 Fase 1: Preparação

### 1.1 Criar Workspace Rust

**Arquivo: `Cargo.toml` (root)**
```toml
[workspace]
members = [
    "collector",
    "tui-client",
    "shared"
]
resolver = "2"

[workspace.dependencies]
sysinfo = "0.32"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### 1.2 Criar Estrutura de Diretórios

```bash
# Criar workspace
mkdir -p collector/src/collectors
mkdir -p collector/src/detector
mkdir -p collector/src/storage/migrations
mkdir -p collector/src/api
mkdir -p tui-client/src/ui
mkdir -p web-frontend/src/{api,components,hooks,types,utils}
mkdir -p shared/src
mkdir -p scripts
mkdir -p docs
```

### 1.3 Migrar Código Existente

- Identificar módulos reutilizáveis do `main.rs` atual
- Separar lógica de coleta da lógica de apresentação
- Mover estruturas de dados para `shared/`

---

## 🔧 Fase 2: Backend - Serviço Coletor

### 2.1 Estruturas de Dados Compartilhadas

**Arquivo: `shared/src/types.rs`**
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub temperatures: Vec<Temperature>,
    pub disks: Vec<DiskMetrics>,
    pub usb_devices: Vec<UsbDevice>,
    pub network: NetworkMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub global_usage: f32,
    pub per_core: Vec<f32>,
    pub load_avg_1: f32,
    pub load_avg_5: f32,
    pub load_avg_15: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temperature {
    pub sensor: String,
    pub value: f32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub read_mb: f32,
    pub write_mb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub id: String,
    pub manufacturer: String,
    pub product: String,
    pub has_timeout: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub severity: AnomalySeverity,
    pub category: AnomalyCategory,
    pub message: String,
    pub metrics: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnomalySeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyCategory {
    Cpu,
    Memory,
    Temperature,
    Disk,
    Usb,
    Network,
    System,
}
```

### 2.2 Módulos de Coleta

**Arquivo: `collector/src/collectors/cpu.rs`**
```rust
use shared::types::CpuMetrics;
use sysinfo::System;

pub struct CpuCollector {
    sys: System,
}

impl CpuCollector {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn collect(&mut self) -> CpuMetrics {
        self.sys.refresh_cpu();

        let global_usage = self.sys.global_cpu_info().cpu_usage();
        let per_core = self.sys.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        let load_avg = System::load_average();

        CpuMetrics {
            global_usage,
            per_core,
            load_avg_1: load_avg.one as f32,
            load_avg_5: load_avg.five as f32,
            load_avg_15: load_avg.fifteen as f32,
        }
    }
}
```

**Arquivo: `collector/src/collectors/memory.rs`**
```rust
use shared::types::MemoryMetrics;
use sysinfo::System;

pub struct MemoryCollector {
    sys: System,
}

impl MemoryCollector {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn collect(&mut self) -> MemoryMetrics {
        self.sys.refresh_memory();

        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let available = self.sys.available_memory();

        MemoryMetrics {
            total,
            used,
            available,
            usage_percent: (used as f32 / total as f32) * 100.0,
            swap_total: self.sys.total_swap(),
            swap_used: self.sys.used_swap(),
        }
    }
}
```

### 2.3 Agregador Principal

**Arquivo: `collector/src/collectors/mod.rs`**
```rust
pub mod cpu;
pub mod memory;
pub mod temperature;
pub mod disk;
pub mod usb;
pub mod network;

use shared::types::SystemMetrics;
use chrono::Utc;

pub struct MetricsCollector {
    cpu: cpu::CpuCollector,
    memory: memory::MemoryCollector,
    temperature: temperature::TemperatureCollector,
    disk: disk::DiskCollector,
    usb: usb::UsbCollector,
    network: network::NetworkCollector,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CpuCollector::new(),
            memory: memory::MemoryCollector::new(),
            temperature: temperature::TemperatureCollector::new(),
            disk: disk::DiskCollector::new(),
            usb: usb::UsbCollector::new(),
            network: network::NetworkCollector::new(),
        }
    }

    pub fn collect_all(&mut self) -> SystemMetrics {
        SystemMetrics {
            timestamp: Utc::now(),
            cpu: self.cpu.collect(),
            memory: self.memory.collect(),
            temperatures: self.temperature.collect(),
            disks: self.disk.collect(),
            usb_devices: self.usb.collect(),
            network: self.network.collect(),
        }
    }
}
```

### 2.4 Detector de Anomalias

**Arquivo: `collector/src/detector/rules.rs`**
```rust
use shared::types::{SystemMetrics, Anomaly, AnomalySeverity, AnomalyCategory};
use chrono::Utc;

pub struct AnomalyRules {
    previous: Option<SystemMetrics>,
}

impl AnomalyRules {
    pub fn new() -> Self {
        Self { previous: None }
    }

    pub fn check(&mut self, current: &SystemMetrics) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // CPU rules
        if current.cpu.global_usage > 90.0 {
            anomalies.push(Anomaly {
                id: 0, // Will be set by DB
                timestamp: Utc::now(),
                severity: AnomalySeverity::Critical,
                category: AnomalyCategory::Cpu,
                message: format!("CPU crítica: {:.1}%", current.cpu.global_usage),
                metrics: serde_json::to_value(&current.cpu).unwrap(),
            });
        }

        // Memory rules
        if current.memory.usage_percent > 95.0 {
            anomalies.push(Anomaly {
                id: 0,
                timestamp: Utc::now(),
                severity: AnomalySeverity::Critical,
                category: AnomalyCategory::Memory,
                message: format!("Memória crítica: {:.1}%", current.memory.usage_percent),
                metrics: serde_json::to_value(&current.memory).unwrap(),
            });
        }

        // Temperature rules
        if let Some(max_temp) = current.temperatures.iter()
            .map(|t| t.value)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
        {
            if max_temp > 85.0 {
                anomalies.push(Anomaly {
                    id: 0,
                    timestamp: Utc::now(),
                    severity: AnomalySeverity::Critical,
                    category: AnomalyCategory::Temperature,
                    message: format!("Temperatura crítica: {:.1}°C", max_temp),
                    metrics: serde_json::to_value(&current.temperatures).unwrap(),
                });
            }
        }

        // Disk rules
        for disk in &current.disks {
            if disk.usage_percent > 90.0 {
                anomalies.push(Anomaly {
                    id: 0,
                    timestamp: Utc::now(),
                    severity: AnomalySeverity::Warning,
                    category: AnomalyCategory::Disk,
                    message: format!("Disco crítico: {} em {:.1}%",
                        disk.name, disk.usage_percent),
                    metrics: serde_json::to_value(disk).unwrap(),
                });
            }
        }

        // USB timeout rules
        if current.usb_devices.iter().any(|u| u.has_timeout) {
            anomalies.push(Anomaly {
                id: 0,
                timestamp: Utc::now(),
                severity: AnomalySeverity::Critical,
                category: AnomalyCategory::Usb,
                message: "Timeout USB detectado".to_string(),
                metrics: serde_json::to_value(&current.usb_devices).unwrap(),
            });
        }

        // Compare with previous if available
        if let Some(prev) = &self.previous {
            // CPU spike
            if current.cpu.global_usage - prev.cpu.global_usage > 40.0 {
                anomalies.push(Anomaly {
                    id: 0,
                    timestamp: Utc::now(),
                    severity: AnomalySeverity::Warning,
                    category: AnomalyCategory::Cpu,
                    message: format!("Pico de CPU: {:.1}% → {:.1}%",
                        prev.cpu.global_usage, current.cpu.global_usage),
                    metrics: serde_json::to_value(&current.cpu).unwrap(),
                });
            }

            // Memory spike
            if current.memory.usage_percent - prev.memory.usage_percent > 20.0 {
                anomalies.push(Anomaly {
                    id: 0,
                    timestamp: Utc::now(),
                    severity: AnomalySeverity::Warning,
                    category: AnomalyCategory::Memory,
                    message: format!("Pico de memória: {:.1}% → {:.1}%",
                        prev.memory.usage_percent, current.memory.usage_percent),
                    metrics: serde_json::to_value(&current.memory).unwrap(),
                });
            }
        }

        self.previous = Some(current.clone());
        anomalies
    }
}
```

### 2.5 Loop Principal do Daemon

**Arquivo: `collector/src/main.rs`**
```rust
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

mod collectors;
mod detector;
mod storage;
mod api;
mod config;

use collectors::MetricsCollector;
use detector::rules::AnomalyRules;
use storage::repository::MetricsRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize
    tracing_subscriber::fmt::init();
    let config = config::load()?;

    // Database
    let repo = Arc::new(MetricsRepository::new(&config.database_url).await?);

    // Shared state
    let current_metrics = Arc::new(RwLock::new(None));
    let recent_anomalies = Arc::new(RwLock::new(Vec::new()));

    // Start API server
    let api_handle = tokio::spawn(
        api::rest::start_server(
            config.clone(),
            Arc::clone(&current_metrics),
            Arc::clone(&recent_anomalies),
            Arc::clone(&repo)
        )
    );

    // Collection loop
    let mut collector = MetricsCollector::new();
    let mut detector = AnomalyRules::new();
    let mut tick = interval(Duration::from_secs(config.collection_interval));

    loop {
        tick.tick().await;

        // Collect metrics
        let metrics = collector.collect_all();

        // Detect anomalies
        let anomalies = detector.check(&metrics);

        // Store in database
        repo.store_metrics(&metrics).await?;
        for anomaly in &anomalies {
            repo.store_anomaly(anomaly).await?;
        }

        // Update shared state for API
        *current_metrics.write().await = Some(metrics);
        *recent_anomalies.write().await = anomalies;

        tracing::info!("Metrics collected and stored");
    }
}
```

---

## 🌐 Fase 3: API REST/WebSocket

### 3.1 Definição de Endpoints REST

**Arquivo: `collector/src/api/routes.rs`**
```rust
use axum::{
    routing::{get, post},
    Router, Json, extract::{State, Path, Query},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TimeRange {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Current metrics
        .route("/api/v1/metrics/current", get(get_current_metrics))

        // Historical metrics
        .route("/api/v1/metrics/history", get(get_metrics_history))

        // Anomalies
        .route("/api/v1/anomalies", get(get_anomalies))
        .route("/api/v1/anomalies/:id", get(get_anomaly_by_id))

        // System info
        .route("/api/v1/system/info", get(get_system_info))

        // Health check
        .route("/health", get(health_check))

        // WebSocket
        .route("/ws", get(websocket_handler))

        .with_state(state)
}

// Handler examples
async fn get_current_metrics(
    State(state): State<AppState>
) -> Json<Option<SystemMetrics>> {
    let metrics = state.current_metrics.read().await;
    Json(metrics.clone())
}

async fn get_metrics_history(
    State(state): State<AppState>,
    Query(range): Query<TimeRange>,
) -> Json<Vec<SystemMetrics>> {
    let metrics = state.repo
        .get_metrics_range(range.start, range.end)
        .await
        .unwrap_or_default();
    Json(metrics)
}

async fn get_anomalies(
    State(state): State<AppState>,
    Query(range): Query<TimeRange>,
) -> Json<Vec<Anomaly>> {
    let anomalies = state.repo
        .get_anomalies_range(range.start, range.end)
        .await
        .unwrap_or_default();
    Json(anomalies)
}
```

### 3.2 WebSocket para Real-time

**Arquivo: `collector/src/api/websocket.rs`**
```rust
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
};
use tokio::time::{interval, Duration};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut tick = interval(Duration::from_secs(2));

    loop {
        tick.tick().await;

        let metrics = state.current_metrics.read().await;
        if let Some(m) = metrics.as_ref() {
            let json = serde_json::to_string(m).unwrap();
            if socket.send(axum::extract::ws::Message::Text(json)).await.is_err() {
                break;
            }
        }
    }
}
```

### 3.3 Documentação da API

**Arquivo: `docs/API.md`**
```markdown
# System Monitor API Documentation

## Base URL
```
http://localhost:8080/api/v1
```

## Endpoints

### GET /metrics/current
Retorna as métricas atuais do sistema.

**Response:**
```json
{
  "timestamp": "2024-01-29T10:30:00Z",
  "cpu": {
    "global_usage": 45.2,
    "per_core": [42.1, 48.3, ...],
    "load_avg_1": 2.5,
    "load_avg_5": 2.1,
    "load_avg_15": 1.8
  },
  "memory": {
    "total": 33769259008,
    "used": 15234567890,
    "available": 18534691118,
    "usage_percent": 45.1,
    "swap_total": 2147483648,
    "swap_used": 0
  },
  ...
}
```

### GET /metrics/history
Retorna histórico de métricas.

**Query Parameters:**
- `start`: ISO 8601 timestamp (opcional)
- `end`: ISO 8601 timestamp (opcional)

**Response:** Array de métricas

### GET /anomalies
Lista anomalias detectadas.

**Query Parameters:**
- `start`: ISO 8601 timestamp (opcional)
- `end`: ISO 8601 timestamp (opcional)
- `severity`: info|warning|critical (opcional)

**Response:**
```json
[
  {
    "id": 1,
    "timestamp": "2024-01-29T10:25:00Z",
    "severity": "critical",
    "category": "cpu",
    "message": "CPU crítica: 92.5%",
    "metrics": { ... }
  }
]
```

### WebSocket /ws
Stream de métricas em tempo real.

**Message Format:** JSON igual ao /metrics/current

**Update Interval:** 2 segundos
```

---

## 💾 Fase 4: Banco de Dados

### 4.1 Schema SQL

**Arquivo: `collector/src/storage/migrations/001_initial.sql`**
```sql
-- Tabela de métricas
CREATE TABLE IF NOT EXISTS metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL,
    cpu_global REAL NOT NULL,
    cpu_per_core TEXT NOT NULL,  -- JSON array
    load_avg_1 REAL NOT NULL,
    load_avg_5 REAL NOT NULL,
    load_avg_15 REAL NOT NULL,
    memory_total INTEGER NOT NULL,
    memory_used INTEGER NOT NULL,
    memory_available INTEGER NOT NULL,
    memory_percent REAL NOT NULL,
    swap_total INTEGER NOT NULL,
    swap_used INTEGER NOT NULL,
    temperatures TEXT NOT NULL,  -- JSON array
    disks TEXT NOT NULL,         -- JSON array
    usb_devices TEXT NOT NULL,   -- JSON array
    network_rx INTEGER NOT NULL,
    network_tx INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_metrics_timestamp ON metrics(timestamp);

-- Tabela de anomalias
CREATE TABLE IF NOT EXISTS anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL,
    severity TEXT NOT NULL,  -- 'info', 'warning', 'critical'
    category TEXT NOT NULL,  -- 'cpu', 'memory', 'temperature', etc
    message TEXT NOT NULL,
    metrics TEXT NOT NULL,   -- JSON com detalhes
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_anomalies_timestamp ON anomalies(timestamp);
CREATE INDEX idx_anomalies_severity ON anomalies(severity);
CREATE INDEX idx_anomalies_category ON anomalies(category);

-- Tabela de configuração
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 4.2 Repository Pattern

**Arquivo: `collector/src/storage/repository.rs`**
```rust
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use shared::types::{SystemMetrics, Anomaly};
use chrono::{DateTime, Utc};

pub struct MetricsRepository {
    pool: SqlitePool,
}

impl MetricsRepository {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn store_metrics(&self, metrics: &SystemMetrics) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO metrics (
                timestamp, cpu_global, cpu_per_core, load_avg_1, load_avg_5, load_avg_15,
                memory_total, memory_used, memory_available, memory_percent,
                swap_total, swap_used, temperatures, disks, usb_devices,
                network_rx, network_tx
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            metrics.timestamp,
            metrics.cpu.global_usage,
            serde_json::to_string(&metrics.cpu.per_core)?,
            metrics.cpu.load_avg_1,
            metrics.cpu.load_avg_5,
            metrics.cpu.load_avg_15,
            metrics.memory.total,
            metrics.memory.used,
            metrics.memory.available,
            metrics.memory.usage_percent,
            metrics.memory.swap_total,
            metrics.memory.swap_used,
            serde_json::to_string(&metrics.temperatures)?,
            serde_json::to_string(&metrics.disks)?,
            serde_json::to_string(&metrics.usb_devices)?,
            metrics.network.rx_bytes,
            metrics.network.tx_bytes,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn store_anomaly(&self, anomaly: &Anomaly) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO anomalies (timestamp, severity, category, message, metrics)
            VALUES (?, ?, ?, ?, ?)
            "#,
            anomaly.timestamp,
            format!("{:?}", anomaly.severity),
            format!("{:?}", anomaly.category),
            anomaly.message,
            serde_json::to_string(&anomaly.metrics)?,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_metrics_range(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<SystemMetrics>> {
        // Implementation...
        todo!()
    }

    pub async fn get_anomalies_range(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<Anomaly>> {
        // Implementation...
        todo!()
    }
}
```

### 4.3 Configuração

**Arquivo: `collector/config.toml`**
```toml
# Database configuration
database_url = "sqlite:///var/lib/system-monitor/metrics.db"

# Collection settings
collection_interval = 2  # seconds

# API settings
api_host = "0.0.0.0"
api_port = 8080

# Retention policy
retention_days = 30  # Keep data for 30 days

# Anomaly thresholds
[thresholds]
cpu_critical = 90.0
cpu_spike = 40.0
memory_critical = 95.0
memory_spike = 20.0
temperature_critical = 85.0
disk_critical = 90.0
```

---

## 🖥️ Fase 5: Cliente TUI

### 5.1 Cliente API

**Arquivo: `tui-client/src/api_client.rs`**
```rust
use shared::types::{SystemMetrics, Anomaly};
use reqwest::Client;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::StreamExt;

pub struct ApiClient {
    base_url: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn get_current_metrics(&self) -> anyhow::Result<Option<SystemMetrics>> {
        let url = format!("{}/api/v1/metrics/current", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.json().await?)
    }

    pub async fn get_anomalies(&self) -> anyhow::Result<Vec<Anomaly>> {
        let url = format!("{}/api/v1/anomalies", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.json().await?)
    }

    pub async fn connect_websocket(&self) -> anyhow::Result<impl StreamExt<Item = SystemMetrics>> {
        let url = format!("ws://{}/ws", self.base_url.replace("http://", ""));
        let (ws_stream, _) = connect_async(url).await?;

        Ok(ws_stream.filter_map(|msg| async move {
            match msg {
                Ok(Message::Text(text)) => {
                    serde_json::from_str(&text).ok()
                }
                _ => None,
            }
        }))
    }
}
```

### 5.2 Interface TUI

**Arquivo: `tui-client/src/ui/dashboard.rs`**
```rust
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};
use shared::types::SystemMetrics;

pub fn render(f: &mut Frame, metrics: &SystemMetrics) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Header
            Constraint::Length(5),   // CPU
            Constraint::Length(5),   // Memory
            Constraint::Length(8),   // Temperatures
            Constraint::Length(10),  // Disks
            Constraint::Min(5),      // Anomalies
        ])
        .split(f.size());

    // Header
    render_header(f, chunks[0]);

    // CPU
    render_cpu(f, chunks[1], &metrics.cpu);

    // Memory
    render_memory(f, chunks[2], &metrics.memory);

    // Temperatures
    render_temperatures(f, chunks[3], &metrics.temperatures);

    // Disks
    render_disks(f, chunks[4], &metrics.disks);

    // Anomalies (separate data source)
}

fn render_cpu(f: &mut Frame, area: Rect, cpu: &CpuMetrics) {
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("CPU"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(cpu.global_usage as u16);

    f.render_widget(gauge, area);
}

// ... outros componentes
```

---

## 🌐 Fase 6: Web Dashboard

### 6.1 Setup Frontend

**Arquivo: `web-frontend/package.json`**
```json
{
  "name": "system-monitor-web",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "recharts": "^2.10.0",
    "axios": "^1.6.0",
    "date-fns": "^2.30.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "autoprefixer": "^10.4.16",
    "postcss": "^8.4.32",
    "tailwindcss": "^3.4.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0"
  }
}
```

### 6.2 API Client

**Arquivo: `web-frontend/src/api/client.ts`**
```typescript
import axios from 'axios';
import { SystemMetrics, Anomaly } from '../types/metrics';

const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8080/api/v1';

export const api = {
  getCurrentMetrics: async (): Promise<SystemMetrics | null> => {
    const { data } = await axios.get(`${API_BASE}/metrics/current`);
    return data;
  },

  getMetricsHistory: async (
    start?: Date,
    end?: Date
  ): Promise<SystemMetrics[]> => {
    const params = new URLSearchParams();
    if (start) params.append('start', start.toISOString());
    if (end) params.append('end', end.toISOString());

    const { data } = await axios.get(`${API_BASE}/metrics/history`, { params });
    return data;
  },

  getAnomalies: async (): Promise<Anomaly[]> => {
    const { data } = await axios.get(`${API_BASE}/anomalies`);
    return data;
  },
};
```

### 6.3 WebSocket Hook

**Arquivo: `web-frontend/src/hooks/useWebSocket.ts`**
```typescript
import { useEffect, useState } from 'react';
import { SystemMetrics } from '../types/metrics';

const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws';

export const useWebSocket = () => {
  const [metrics, setMetrics] = useState<SystemMetrics | null>(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const ws = new WebSocket(WS_URL);

    ws.onopen = () => {
      console.log('WebSocket connected');
      setConnected(true);
    };

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setMetrics(data);
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      setConnected(false);
    };

    ws.onclose = () => {
      console.log('WebSocket disconnected');
      setConnected(false);
    };

    return () => {
      ws.close();
    };
  }, []);

  return { metrics, connected };
};
```

### 6.4 Dashboard Component

**Arquivo: `web-frontend/src/components/Dashboard.tsx`**
```typescript
import React from 'react';
import { useWebSocket } from '../hooks/useWebSocket';
import CpuChart from './CpuChart';
import MemoryChart from './MemoryChart';
import TemperatureGauge from './TemperatureGauge';
import DiskUsage from './DiskUsage';
import AnomalyList from './AnomalyList';

export const Dashboard: React.FC = () => {
  const { metrics, connected } = useWebSocket();

  if (!connected || !metrics) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-xl">Conectando ao servidor...</div>
      </div>
    );
  }

  return (
    <div className="p-6 bg-gray-100 min-h-screen">
      <h1 className="text-3xl font-bold mb-6">System Monitor</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* CPU */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">CPU Usage</h2>
          <CpuChart data={metrics.cpu} />
        </div>

        {/* Memory */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Memory Usage</h2>
          <MemoryChart data={metrics.memory} />
        </div>

        {/* Temperature */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Temperatures</h2>
          <div className="grid grid-cols-2 gap-4">
            {metrics.temperatures.map((temp, idx) => (
              <TemperatureGauge key={idx} temp={temp} />
            ))}
          </div>
        </div>

        {/* Disks */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Disk Usage</h2>
          <DiskUsage disks={metrics.disks} />
        </div>
      </div>

      {/* Anomalies */}
      <div className="mt-6 bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">Recent Anomalies</h2>
        <AnomalyList />
      </div>
    </div>
  );
};
```

### 6.5 Chart Components

**Arquivo: `web-frontend/src/components/CpuChart.tsx`**
```typescript
import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from 'recharts';
import { CpuMetrics } from '../types/metrics';

interface Props {
  data: CpuMetrics;
}

export const CpuChart: React.FC<Props> = ({ data }) => {
  const chartData = data.per_core.map((usage, idx) => ({
    name: `Core ${idx}`,
    usage: usage.toFixed(1),
  }));

  return (
    <div>
      <div className="text-4xl font-bold mb-4">
        {data.global_usage.toFixed(1)}%
      </div>
      <LineChart width={400} height={200} data={chartData}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis dataKey="name" />
        <YAxis domain={[0, 100]} />
        <Tooltip />
        <Line type="monotone" dataKey="usage" stroke="#8884d8" />
      </LineChart>
      <div className="mt-2 text-sm text-gray-600">
        Load Avg: {data.load_avg_1.toFixed(2)} / {data.load_avg_5.toFixed(2)} / {data.load_avg_15.toFixed(2)}
      </div>
    </div>
  );
};

export default CpuChart;
```

---

## 🚀 Fase 7: Deploy e Produção

### 7.1 Docker Compose

**Arquivo: `docker-compose.yml`**
```yaml
version: '3.8'

services:
  collector:
    build:
      context: .
      dockerfile: collector/Dockerfile
    container_name: system-monitor-collector
    restart: unless-stopped
    volumes:
      - /sys:/host/sys:ro
      - /proc:/host/proc:ro
      - ./data:/var/lib/system-monitor
    environment:
      - RUST_LOG=info
    ports:
      - "8080:8080"
    networks:
      - monitor-net
    privileged: true  # Necessário para acessar métricas do sistema

  web:
    build:
      context: ./web-frontend
      dockerfile: Dockerfile
    container_name: system-monitor-web
    restart: unless-stopped
    ports:
      - "3000:80"
    environment:
      - VITE_API_URL=http://localhost:8080/api/v1
      - VITE_WS_URL=ws://localhost:8080/ws
    depends_on:
      - collector
    networks:
      - monitor-net

networks:
  monitor-net:
    driver: bridge

volumes:
  monitor-data:
```

### 7.2 Systemd Service

**Arquivo: `collector/systemd/system-monitor.service`**
```ini
[Unit]
Description=System Monitor Collector Service
After=network.target

[Service]
Type=simple
User=monitor
Group=monitor
WorkingDirectory=/opt/system-monitor
ExecStart=/opt/system-monitor/collector/target/release/collector
Restart=on-failure
RestartSec=10s

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/system-monitor

# Environment
Environment="RUST_LOG=info"
Environment="CONFIG_PATH=/etc/system-monitor/config.toml"

[Install]
WantedBy=multi-user.target
```

### 7.3 Scripts de Deploy

**Arquivo: `scripts/deploy.sh`**
```bash
#!/bin/bash
set -euo pipefail

echo "🚀 Deploying System Monitor..."

# Build backend
echo "📦 Building collector..."
cd collector
cargo build --release
cd ..

# Build frontend
echo "🎨 Building web frontend..."
cd web-frontend
npm install
npm run build
cd ..

# Install systemd service
echo "⚙️ Installing systemd service..."
sudo cp collector/systemd/system-monitor.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable system-monitor
sudo systemctl restart system-monitor

# Setup nginx for web frontend
echo "🌐 Setting up nginx..."
sudo cp scripts/nginx.conf /etc/nginx/sites-available/system-monitor
sudo ln -sf /etc/nginx/sites-available/system-monitor /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

echo "✅ Deploy complete!"
echo "   Backend: http://localhost:8080"
echo "   Frontend: http://localhost:3000"
```

### 7.4 Nginx Configuration

**Arquivo: `scripts/nginx.conf`**
```nginx
server {
    listen 80;
    server_name monitor.example.com;

    # Frontend
    location / {
        root /opt/system-monitor/web-frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    # API proxy
    location /api/ {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # WebSocket proxy
    location /ws {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
    }
}
```

---

## 🛠️ Tecnologias Utilizadas

### Backend (Rust)
| Crate | Versão | Propósito |
|-------|--------|-----------|
| tokio | 1.35 | Runtime assíncrono |
| axum | 0.7 | Web framework |
| sysinfo | 0.32 | Métricas do sistema |
| sqlx | 0.7 | Database ORM |
| serde | 1.0 | Serialização |
| chrono | 0.4 | Data/hora |
| tracing | 0.1 | Logging |
| anyhow | 1.0 | Error handling |
| tokio-tungstenite | 0.21 | WebSocket |

### Frontend (Web)
| Tecnologia | Versão | Propósito |
|------------|--------|-----------|
| React | 18.2 | UI Framework |
| TypeScript | 5.3 | Type safety |
| Vite | 5.0 | Build tool |
| Tailwind CSS | 3.4 | Styling |
| Recharts | 2.10 | Charts/graphs |
| Axios | 1.6 | HTTP client |

### Frontend (TUI)
| Crate | Versão | Propósito |
|-------|--------|-----------|
| ratatui | 0.25 | Terminal UI |
| crossterm | 0.27 | Terminal control |
| reqwest | 0.11 | HTTP client |
| tokio-tungstenite | 0.21 | WebSocket |

### Database
- **SQLite** (desenvolvimento/single-node)
- **PostgreSQL** (produção/multi-node) - opcional

---

## 📅 Cronograma Estimado

### Sprint 1 (Semana 1-2)
- ✅ Setup workspace Rust
- ✅ Estruturas de dados compartilhadas
- ✅ Módulos de coleta (migração do código atual)
- ✅ Detector de anomalias

### Sprint 2 (Semana 3-4)
- ✅ Banco de dados SQLite
- ✅ Repository pattern
- ✅ Migrations
- ✅ Loop principal do daemon

### Sprint 3 (Semana 5-6)
- ✅ API REST com Axum
- ✅ Endpoints principais
- ✅ WebSocket real-time
- ✅ Testes de API

### Sprint 4 (Semana 7-8)
- ✅ Cliente TUI
- ✅ Interface com ratatui
- ✅ Consumo de API/WebSocket
- ✅ Replicar UI atual

### Sprint 5 (Semana 9-10)
- ✅ Setup frontend web (React + Vite)
- ✅ Componentes principais
- ✅ Charts e gráficos
- ✅ Real-time updates

### Sprint 6 (Semana 11-12)
- ✅ Docker/Docker Compose
- ✅ Systemd service
- ✅ Scripts de deploy
- ✅ Documentação final
- ✅ Testes end-to-end

---

## 🧪 Testes e Validação

### Testes Unitários
```bash
# Backend
cd collector
cargo test

# Frontend
cd web-frontend
npm test
```

### Testes de Integração
```bash
# Testar API
curl http://localhost:8080/api/v1/metrics/current
curl http://localhost:8080/health

# Testar WebSocket
wscat -c ws://localhost:8080/ws
```

### Testes de Performance
```bash
# Stress test da API
ab -n 10000 -c 100 http://localhost:8080/api/v1/metrics/current

# Monitor de recursos
htop # Ver consumo do daemon
```

### Validação de Métricas
```bash
# Comparar com ferramentas nativas
top -b -n 1
free -h
df -h
sensors
lsusb

# Comparar com output do sistema
./target/release/system-monitor  # TUI atual
```

---

## 📊 Métricas de Sucesso

### Performance
- [ ] Coleta de métricas: < 10ms por ciclo
- [ ] API response time: < 50ms (p95)
- [ ] WebSocket latency: < 100ms
- [ ] Frontend render: 60fps constante

### Confiabilidade
- [ ] Daemon uptime: > 99.9%
- [ ] Zero data loss em crash recovery
- [ ] Anomaly detection accuracy: > 95%

### Usabilidade
- [ ] Dashboard carrega em < 2s
- [ ] Real-time updates sem delay perceptível
- [ ] UI responsiva em mobile

---

## 🔄 Próximas Fases (Futuro)

### Fase 8: Machine Learning
- Detecção de anomalias com ML
- Predição de falhas
- Clustering de padrões

### Fase 9: Alertas
- Email notifications
- Slack/Discord webhooks
- SMS alerts (Twilio)

### Fase 10: Multi-node
- Coletar de múltiplos servidores
- Dashboard centralizado
- Comparação entre hosts

### Fase 11: Mobile App
- React Native
- Notificações push
- Controle remoto

---

## 📚 Recursos Adicionais

### Documentação
- [Axum Documentation](https://docs.rs/axum)
- [Ratatui Book](https://ratatui.rs)
- [React Documentation](https://react.dev)
- [SQLx Guide](https://github.com/launchbadge/sqlx)

### Exemplos
- Ver `examples/` no repositório
- Testes em `tests/`

### Suporte
- Issues: GitHub Issues
- Discussões: GitHub Discussions

---

## ✅ Checklist de Implementação

### Backend
- [ ] Workspace setup
- [ ] Estruturas de dados (shared)
- [ ] Módulos de coleta
  - [ ] CPU
  - [ ] Memory
  - [ ] Temperature
  - [ ] Disk
  - [ ] USB
  - [ ] Network
- [ ] Detector de anomalias
- [ ] Database schema
- [ ] Repository pattern
- [ ] API REST
- [ ] WebSocket
- [ ] Daemon principal
- [ ] Configuração
- [ ] Logging/tracing

### Frontend TUI
- [ ] API client
- [ ] WebSocket client
- [ ] Dashboard layout
- [ ] Componentes UI
- [ ] Configuração

### Frontend Web
- [ ] Setup Vite + React
- [ ] API client
- [ ] WebSocket hook
- [ ] Dashboard
- [ ] Componentes
  - [ ] CPU Chart
  - [ ] Memory Chart
  - [ ] Temperature Gauge
  - [ ] Disk Usage
  - [ ] Anomaly List
- [ ] Styling (Tailwind)
- [ ] Responsividade

### DevOps
- [ ] Dockerfile (collector)
- [ ] Dockerfile (web)
- [ ] Docker Compose
- [ ] Systemd service
- [ ] Nginx config
- [ ] Scripts de deploy
- [ ] CI/CD pipeline

### Documentação
- [ ] README principal
- [ ] API documentation
- [ ] Development guide
- [ ] Deploy guide
- [ ] Troubleshooting

---

## 🎯 Conclusão

Este plano de implementação fornece um roadmap completo para transformar o monitor de sistema atual em uma arquitetura moderna e escalável. A divisão em fases permite desenvolvimento incremental e validação contínua.

**Prioridades:**
1. Manter a funcionalidade atual enquanto refatora
2. Zero downtime durante migração
3. Backwards compatibility quando possível
4. Documentação contínua

**Próximos Passos Imediatos:**
1. Criar workspace Rust
2. Migrar estruturas de dados para `shared/`
3. Separar lógica de coleta
4. Implementar primeira versão da API

---

**Última Atualização:** 2026-01-29
**Versão do Plano:** 1.0
