# GitHub Week Labeler

A Rust program to automatically create weekly labels (e.g., `week-2025-47`) for a **GitHub organization** and assign them to **all existing repositories**. Labels are formatted with the ISO year and week number and can be generated for a configurable number of weeks.

---

## Features

* Generates weekly labels in the format: `week-YYYY-WW` (leading zero for weeks < 10)
* Creates labels on the **organization level**, so new repos automatically inherit them
* Applies the same labels to **all existing repositories** in the organization
* Skips labels that already exist to avoid errors
* Random color assigned per week (can be made consistent per week if desired)
* Configurable number of weeks
* Reads organization ID and GitHub token from a JSON config file
* Handles API errors gracefully

---

## Requirements

* Rust 2024 edition
* A **GitHub Personal Access Token (PAT)** with:

    * `repo` (access to repositories)
    * `admin:org` (to create org-level labels)

---

## Installation

1. Clone this repository or copy the Rust source code:

```bash
git clone <your-repo-url>
cd github-week-labeler
```

2. Add dependencies in `Cargo.toml`:

```toml
[dependencies]
chrono = "0.4"
rand = "0.8"
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

3. Build the program:

```bash
cargo build --release
```

---

## Configuration

Create a `config.json` file with the following structure:

```json
{
  "org_name": "YOUR_ORG_NAME",
  "org_id": "O_kgDOB12345",
  "github_token": "ghp_ABC1234567890token"
}
```

* `org_name`: GitHub organization login
* `org_id`: GraphQL organization ID (use GitHub API to fetch it)
* `github_token`: Personal Access Token

---

## Usage

```bash
cargo run -- config.json [WEEKS]
```

* `config.json` → path to your config file
* `[WEEKS]` → optional number of weeks to generate (default: 26)

### Example

Generate labels for 12 weeks:

```bash
cargo run -- config.json 12
```

---

## Example Output

```
Creating org-level label: week-2025-47
✅ Created org-level label week-2025-47
Creating org-level label: week-2025-48
✅ Created org-level label week-2025-48

Found 5 repositories in org 'my-org'
Processing repo: repo-1
  ✅ Created label week-2025-47 in repo-1
  ✅ Created label week-2025-48 in repo-1
Processing repo: repo-2
  ✅ Created label week-2025-47 in repo-2
  ✅ Created label week-2025-48 in repo-2
...
```

---

## Notes

* Labels are created on both the **organization level** and all **existing repositories**.
* Existing labels are skipped automatically.
* Each week is assigned a random color by default.
* Make sure your GitHub token has the necessary permissions; otherwise, API calls may fail.

---

## License

MIT License
