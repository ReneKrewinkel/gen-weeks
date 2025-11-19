# GitHub Week Labeler

A Rust program to automatically create weekly labels (e.g., `week-2025-47`) for **all repositories in a GitHub organization**. Labels are formatted with the ISO year and week number and can optionally be generated for a configurable number of weeks.

---

## Features

* Generates weekly labels in the format: `week-YYYY-WW` (leading zero for weeks < 10)
* Applies labels to **all repositories** in a GitHub organization
* Skips labels that already exist
* Random color assigned per label (optional: same color for all repos can be implemented)
* Configurable number of weeks
* Uses a JSON config file for organization and token settings
* Handles API errors gracefully

---

## Requirements

* Rust 2024 edition
* A **GitHub Personal Access Token** (PAT) with:

  * `repo` (access to repositories)
  * `admin:org` (for org-wide access if needed)

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
  "github_token": "ghp_ABC1234567890token"
}
```

* `org_name`: GitHub organization login
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

* Labels are created individually per repository. Existing labels are skipped.
* Each week gets a random color by default.
* Make sure your GitHub token has the correct permissions; otherwise, API calls may fail.

---

## License

MIT License.

---

This README provides **everything needed** to configure and run the GitHub Week Labeler.
