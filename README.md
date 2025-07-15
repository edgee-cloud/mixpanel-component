# 🧠 Mixpanel Component for Edgee

This is a Rust-based Edgee component that integrates Mixpanel analytics using the Edgee Data Collection protocol. It allows you to track user events, page views, and user identification, while sending them directly to Mixpanel via the `/track` and `/engage` APIs.

---

## ✨ Features

- ✅ Track user events (`track`)
- ✅ Track page views (`page`)
- ✅ Identify and update users (`user`)
- ✅ Highly optimized for execution at the edge

---

## 🔧 Settings

This component requires one setting:

| Key              | Type   | Required | Description                          |
|------------------|--------|----------|--------------------------------------|
| `mixpanel_token` | string | ✅       | Your Mixpanel project token (see below) |

You can find your project token in your Mixpanel project settings under **Project Settings > Access Keys**.

---

## 🧪 Testing Locally

You can test this component using the Edgee CLI:

### Build the component

```bash
edgee component build
```

### Run tests
```bash
cargo test
```
### Run live test with event simulation

```bash
edgee components test \
  --event-type track \
  --settings mixpanel_token=YOUR_TOKEN \
  --make-http-request
```
You can also test page and user events by changing --event-type.

### 🚀 Deploying to Edgee Registry
Once tested and ready, you can publish your component:
```bash
edgee components publish
```
### 📂 Project Structure
```text
├── src/
│   └── lib.rs               # Component logic
├── Cargo.toml               # Rust dependencies and metadata
├── edgee-component.toml     # Component manifest for Edgee
├── component.wasm           # Build output
```
### 📚 Learn More

- [Mixpanel HTTP Tracking API](https://developer.mixpanel.com/reference/track-event)
- [Edgee Developer Guide](Mixpanel HTTP Tracking API)