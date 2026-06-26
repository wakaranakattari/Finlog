# Finlog

Personal expense tracker with CLI and web interface.

## Stack

<p align="left">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" height="35">
  <img src="https://img.shields.io/badge/Clojure-5881D8?style=for-the-badge&logo=clojure&logoColor=white" height="35">
  <img src="https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white" height="35">
  <img src="https://img.shields.io/badge/Shadow--cljs-5881D8?style=for-the-badge&logo=clojure&logoColor=white" height="35">
  <img src="https://img.shields.io/badge/JSON-000000?style=for-the-badge&logo=json&logoColor=white" height="35">
</p>

## Screenshots

<details>
<summary>CLI</summary>

| Main menu | Add spending | View spending |
|-----------|-------------|---------------|
| ![menu](assets/cli_menu.png) | ![add](assets/cli_add.png) | ![view](assets/cli_view.png) |

| Statistics | Find item | Delete item |
|-----------|-----------|-------------|
| ![stats](assets/cli_stats.png) | ![find](assets/cli_find.png) | ![delete](assets/cli_delete.png) |

</details>

<details>
<summary>Web</summary>

| Dashboard | Expenses | Statistics |
|-----------|----------|------------|
| ![dashboard](assets/web_dashboard.png) | ![expenses](assets/web_expenses.png) | ![stats](assets/web_stats.png) |

</details>

## Features

### CLI
- Add, view, delete and search expenses
- Colored terminal output
- Spending statistics
- Username configuration
- Automatic JSON storage

### Web
- Dashboard with time-based greeting
- Weekly and all-time statistics
- Interactive charts: by category, by day, by month, distribution
- Full expense list
- Responsive design

## Requirements

<p align="left">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" height="35">
  <img src="https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white" height="35">
  <img src="https://img.shields.io/badge/JDK-ED8B00?style=for-the-badge&logo=openjdk&logoColor=white" height="35">
</p>

## Quick Start

```bash
git clone https://github.com/wakaranakattari/Finlog.git
cd Finlog
cargo run --release
```

Select option `1` from the menu to start the web version.
On first launch, dependencies are installed and the frontend is compiled automatically.
Open browser at `http://127.0.0.1:3000`.

## Project Structure

```
Finlog/
├── src/
│   ├── core/        # Spending manager
│   ├── storage/     # JSON persistence
│   ├── server/      # Axum web server
│   └── utils/       # Colors, console, error handling
└── web/
    ├── src/
    │   └── finlog/
    │       ├── api.cljs
    │       ├── core.cljs
    │       └── components/
    │           ├── header.cljs
    │           ├── dashboard.cljs
    │           ├── expenses.cljs
    │           └── statistics.cljs
    └── public/
        ├── index.html
        ├── css/
        └── data/
```

## License
```
MIT
```