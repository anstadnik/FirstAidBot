# FirstAidRobot

[![Test](https://github.com/anstadnik/FirstAidBot/actions/workflows/CI.yml/badge.svg)](https://github.com/anstadnik/FirstAidBot/actions/workflows/CI.yml)
[![Deploy](https://github.com/anstadnik/FirstAidBot/actions/workflows/CD.yml/badge.svg)](https://github.com/anstadnik/FirstAidBot/actions/workflows/CD.yml)
[![dependency status](https://deps.rs/repo/github/anstadnik/FirstAidBot/status.svg)](https://deps.rs/repo/github/anstadnik/FirstAidBot)

🔺БОТ-ПОМІЧНИК ДЛЯ НАДАННЯ ПЕРШОЇ ДОМЕДИЧНОЇ ДОПОМОГИ🔻

👉[@FirstAidRobot](https://t.me/FirstAidRobot) створений командою однодумців та волонтерів для надання рекомендацій з домедичної допомоги громадянам під час воєнного стану.
👉У боті зібрано інструкції для надання домедичної допомоги згідно з наказом МОЗ Nº441від 9.03.2022.
👉Бот буде доповнюватись за появи нових запитів.

🙏Допоможіть поширити бота - це може врятувати життя🙏

# Architecture
- `core` (Rust) - data loading, state management
- `bot` (Rust) - telegram bot. Messaging, commands, broadcasting
- `app` (Flutter) - flutter app. Uses `core` for content management

# Git workflow
- `main` - production-ready code, only pull requests. After CI passes, merged
commits are deployed to production
- `dev` - development branch. All feature branches are merged here. After CI passes,
merged commits are deployed to staging
