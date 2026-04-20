# DuckDuckGo Command Line Interface 🦆

The standalone `ddg` binary translates CLI commands into structured requests executed by the underlying Rust engine. It exposes multiple interfaces for searching DuckDuckGo natively from your terminal.

## 📦 Installation

To enable the CLI binary compilation, you must install the crate utilizing the `rust-binary` feature.

```bash
cargo install duckduckgo --features rust-binary
```

The resulting executable is named `ddg`.

## 🛠 Available Subcommands & Usage

### 1. General DuckDuckGo Search (Default)

Executes a general DuckDuckGo search matching standard website outcomes via the `Auto` backend, optionally integrating specific DuckDuckGo search operators.

```bash
ddg --query "rust programming language"
```

With advanced operators to limit results to a specific site:

```bash
ddg --query "error handling" --operators "+site:docs.rs"
```

### 2. Lite Search Backend

DuckDuckGo Lite is perfect for fetching minimal, structured payloads without JavaScript bloat. Simply pass the payload switch.

```bash
ddg --query "linux coreutils" --backend Lite
```

### 3. Image Search Backend

Retrieve image URLs and metadata using the `/i.js` endpoint system.

```bash
ddg --query "ferris the crab" --backend Images --safe true
```

### 4. News Search Backend

Retrieve trending article results leveraging the `/news.js` endpoint mapping.

```bash
ddg --query "rust lang updates" --backend News
```

## 🎨 Options Configuration

You can heavily configure the behavior of the internal search client dynamically per-invocation.

| Flag / Option        | Default      | Description                                                    |
| -------------------- | ------------ | -------------------------------------------------------------- |
| `--query`, `-q`      | _(required)_ | The search query to locate.                                    |
| `--operators`, `-o`  | `""`         | Set search operators (e.g., `+site:github.com`).               |
| `--safe`, `-s`       | `false`      | Enable safe search. (Family-friendly mode)                     |
| `--format`, `-f`     | `false`      | Output format: `false` = list, `true` = detailed output.       |
| `--limit`, `-l`      | `10`         | Hard limit constraints for maximal search pagination arrays.   |
| `--user-agent`, `-u` | `firefox`    | Set a custom preset user agent payload for HTTP requests.      |
| `--cookie`, `-c`     | `true`       | Persist cookies across paginated requests for the search loop. |
| `--proxy`, `-p`      | `""`         | Bind traffic through a designated HTTP or Socks proxy proxy.   |
| `--backend`, `-b`    | `Auto`       | Backend mapping: `Auto`, `Lite`, `Images`, or `News`.          |

## 🗺 User Agent Profiles

The backend incorporates multiple pre-defined User-Agent identifier aliases allowing simplified mimicking for different execution targets. This avoids immediate fingerprint blocks.

When running invocations, the value passed to `--user-agent` maps iteratively against the engine profiles.

<details>
<summary>Available User Agents Tracker</summary>

| user_agents            |
| :--------------------- |
| abrowse                |
| aol                    |
| avant                  |
| baidu                  |
| bing                   |
| bot                    |
| camino                 |
| charon                 |
| cheshire               |
| chimera                |
| chrome                 |
| chromium               |
| columbus               |
| curl                   |
| default                |
| edge                   |
| emailwolf              |
| epiphany               |
| firefox                |
| flock                  |
| flok                   |
| galeon                 |
| google                 |
| icab                   |
| iceape                 |
| icecat                 |
| iceweasel              |
| inet browser           |
| internet explorer      |
| irider                 |
| iron                   |
| k-meleon               |
| k-ninja                |
| kapiko                 |
| kazehakase             |
| kindle browser         |
| kkman                  |
| kmlite                 |
| konqueror              |
| leechcraft             |
| links                  |
| lobo                   |
| lolifox                |
| lorentz                |
| lunascape              |
| lynx                   |
| madfox                 |
| maxthon                |
| midori                 |
| minefield              |
| mozilla                |
| myibrow                |
| myie2                  |
| namoroka               |
| navscape               |
| ncsa_mosaic            |
| netnewswire            |
| netpositive            |
| netscape               |
| netsurf                |
| omniweb                |
| opera                  |
| orca                   |
| oregano                |
| osb-browser            |
| palemoon               |
| phoenix                |
| pogo                   |
| prism                  |
| qtweb internet browser |
| rekonq                 |
| retawq                 |
| rockmelt               |
| safari                 |
| seamonkey              |
| shiira                 |
| shiretoko              |
| sleipnir               |
| slimbrowser            |
| stainless              |
| sundance               |
| sunrise                |
| surf                   |
| sylera                 |
| tencent traveler       |
| tenfourfox             |
| theworld browser       |
| uzbl                   |
| vimprobable            |
| vonkeror               |
| w3m                    |
| weltweitimnetzbrowser  |
| worldwideweb           |
| wyzo                   |

</details>

```bash
ddg -q "rust docs" -b Lite -u safari
```

## 🍪 Advanced Usage

### Set cookie for subsequent requests:

```bash
ddg --query "rust lang" --cookie
```

### Set proxy:

```bash
ddg --query "rust lang" --proxy "socks5://192.168.1.1:9000"
```
