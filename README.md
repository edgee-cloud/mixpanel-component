<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">Mixpanel component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/mixpanel-component/badge.svg)](https://coveralls.io/github/edgee-cloud/mixpanel-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/mixpanel-component.svg)](https://github.com/edgee-cloud/mixpanel-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/mixpanel)

This is a Rust-based Edgee component that integrates Mixpanel analytics using the Edgee Data Collection protocol. It allows you to track user events, page views, and identify users, sending data to Mixpanel via the `/import` and `/engage` APIs.

---

## ✨ Features

- ✅ Track custom user events (`track`)
- ✅ Track page views (`page`)
- ✅ Identify and update users (`user`)
- ✅ Built for Edge execution: fast, secure, serverless

---

## 🔧 Settings
 
This component requires the following settings:

| Key              | Type   | Required | Description                                                        |
|------------------|--------|----------|--------------------------------------------------------------------|
| `api_secret`     | string | ✅       | Your Mixpanel **API Secret** (from Project Settings > Access Keys) |
| `project_token`  | string | ✅       | Your Mixpanel **Project Token** (used by the Engage API)           |
| `project_id`     | string | ❌       | Optional Mixpanel Project ID (used for strict mode on import)      |
| `region`         | string | ❌       | Mixpanel region: `api`, `api-eu`, or `api-in` (defaults to `api`)  |

---

## 🧪 Testing Locally

### 🛠️ Build the component

```bash
edgee component build
```

### ✅ Run unit tests

```bash
cargo test
```
### 🔍 Run a live test with simulated events

```bash
edgee components test \
  --event-type track \
  --settings api_secret=YOUR_API_SECRET,project_token=YOUR_PROJECT_TOKEN,project_id=YOUR_PROJECT_ID,region=api-eu \
  --make-http-request
```
Replace event-type with page or user to test other event types.

### 🚀 Deploy to Edgee Registry
Once tested and ready, you can publish your component:
```bash
edgee components push
```
### 📂 Project Structure
```text
mixpanel-component/
├── src/
│   └── lib.rs                 # Main component logic
├── target/
│   └── wasm32-wasip2/
│       └── release/
│           └── mixpanel.wasm  # Built WebAssembly output
├── mixpanel.png               # Component icon
├── Cargo.toml                 # Rust dependencies
└── edgee-component.toml       # Edgee manifest
```
### 📚 Learn More

- [Mixpanel HTTP import API](https://developer.mixpanel.com/reference/import-events)
- [Mixpanel HTTP Engage API](https://developer.mixpanel.com/reference/profile-set)
- [Edgee Developer Guide](https://www.edgee.cloud/docs/services/registry/developer-guide)
