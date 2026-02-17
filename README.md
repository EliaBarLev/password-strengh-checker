# password-strengh-checker
A high-performance, security-oriented password strength evaluation engine written in Rust, wrapped with an API and served through a modern React UI.

## 🚀 Components

### 1. `engine/` (Rust)
Core password analysis engine:
- Entropy calculation
- Brute-force time estimation
- Dictionary/pattern detection
- Leaked-password checks (Bloom filter)

### 2. `api/` (Go or Rust)
REST API that exposes:
- `POST /score`
- `GET /health`

### 3. `ui/` (React)
User interface with:
- Real-time password meter
- Entropy visualization
- Strength breakdown

### 4. `docker-compose`
Runs everything in isolated containers.

### 5. CI/CD (GitHub Actions)
Builds Rust engine, API, and UI automatically.
