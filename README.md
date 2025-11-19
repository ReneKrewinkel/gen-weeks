# GitHub Weekly Label Builder `gen-weeks`

A Rust CLI tool to automate creating and managing weekly GitHub labels across your repositories. This tool helps you organize issues and pull requests by ISO week number, with a color gradient from **light blue (week 1)** to **dark purple (week 52)**.

---

## Features

- Generate weekly labels in the format `week-YYYY-WW` with leading zeros.  
- Apply labels to:  
  - **A single repository**  
  - **All repositories in an organization**  
  - **All personal repositories** if no organization is specified.  
- Forces label creation or updates existing labels with new colors and descriptions.  
- Supports GitHub Projects filtering by week labels.  
- Configurable number of weeks (default: 26).  
- Panic-safe JSON parsing and API responses.  
- Supports CLI flags: `-f <config_file>` and `-w <weeks>`.

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/ReneKrewinkel/gen-weeks
cd gen-weeks
```

2. Build the project with Rust:

```bash
cargo build --release
```

3. The executable will be available at `target/release/gen-weeks`.

---

## Configuration

Create a `config.json` file:

### Single Repository

```json
{
  "github_token": "ghp_your_personal_access_token",
  "repo": "owner/repo"
}
```

### All Repositories in an Organization

```json
{
  "github_token": "ghp_your_personal_access_token",
  "org_name": "my-org"
}
```

### Personal Repositories

```json
{
  "github_token": "ghp_your_personal_access_token"
}
```

> Note: The GitHub token must have `repo` permissions and `read:org` if updating organization repositories.

---

## Usage

```bash
./gen-weeks [-f <config_file>] [-w <weeks>]
```

- `-f <config_file>` → Path to the JSON configuration file (default: `config.json`).  
- `-w <weeks>` → Number of upcoming weeks to generate labels for (default: 26).  

### Examples

```bash
# Use default config.json and 26 weeks
./gen-weeks

# Specify a config file
./gen-weeks -f my_config.json

# Specify number of weeks
./gen-weeks -w 52

# Specify both config file and weeks
./gen-weeks -f my_config.json -w 52
```

---

## How It Works

1. Reads the JSON config file and GitHub token.  
2. Determines repositories to update:
   - Specific repository if `repo` is provided.  
   - Organization repositories if `org_name` is provided.  
   - Personal repositories if neither is specified.  
3. Generates weekly labels with a light blue → dark purple gradient.  
4. Creates new labels or updates existing ones in each repository.  
5. Labels can be used in **GitHub Projects views** to filter by week.

---

## Example

- Label for week 5 of 2025: `week-2025-05` with a mid-blue color.  
- Week 26: `week-2025-26` with a purple color.  

You can easily glance at issues and pull requests and see which ISO week they belong to.

---

## License

MIT License © Rene Krewinkel

---

## Contribution

Feel free to open issues or submit pull requests for new features, such as:

- GitHub Actions integration  
- Parallelized label updates  
- Custom color gradients  

---

This tool simplifies label management and helps you maintain a **consistent, visual weekly tracking system** across all your GitHub repositories.  

