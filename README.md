# PHPMD LSP for Zed Editor

> A Language Server Protocol implementation that brings PHP Mess Detector integration to Zed Editor

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PHP](https://img.shields.io/badge/PHP-8.0%2B-777BB4?logo=php&logoColor=white)](https://php.net)
[![Zed](https://img.shields.io/badge/Zed-Editor-blue?logo=zed&logoColor=white)](https://zed.dev)
[![PHPMD](https://img.shields.io/badge/PHPMD-2.15%2B-green)](https://phpmd.org)

This extension integrates PHP Mess Detector with Zed Editor to provide real-time code quality analysis. It detects code smells, possible bugs, suboptimal code, overcomplicated expressions, and unused parameters, methods, and properties.

## Features

- **Real-time analysis** - Detect code issues as you type
- **Zero configuration** - Works out of the box with sensible defaults
- **Live configuration** - Settings changes apply immediately without restart
- **Auto-recovery** - Automatically handles deleted or invalid config files
- **Multiple rulesets** - Built-in support for all PHPMD rulesets
- **Project awareness** - Automatically discovers phpmd.xml configuration
- **Smart PHPMD detection** - Prefers project-local installations
- **Cross-platform** - Includes binaries for Linux, macOS, and Windows
- **Flexible configuration** - Via Zed settings, environment variables, or project files

### Performance & Reliability

- **Async process handling** - Non-blocking PHPMD execution keeps editor responsive
- **Concurrent processing** - Analyze up to 4 files simultaneously for faster results
- **Timeout protection** - Automatic 10-second timeout prevents hanging on large files
- **Memory optimization** - LZ4 compression reduces memory usage by ~85%
- **Smart caching** - Results cached to avoid redundant analysis on unchanged files
- **Process management** - Automatic cleanup of zombie processes

## Quick Start

### Installation

```bash
# Via Zed Extensions (coming soon)
# For now: manual installation for development
```

### Basic Usage

1. **Ensure the PHPMD extension is installed** in Zed from the Extensions panel

2. **Enable the language server** in your Zed settings.json:

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "phpmd", "!phpactor"]
    }
  }
}
```

3. **Open any PHP project** and the extension will start analyzing your code:

```php
<?php
// This will show warnings for code complexity issues
class User {
    private $unusedProperty; // Unused property warning
    
    public function complexMethod($a, $b, $c, $d, $e, $f) { // Too many parameters
        if ($a) {
            if ($b) {
                if ($c) {
                    if ($d) {
                        if ($e) {
                            return $f; // Excessive nesting
                        }
                    }
                }
            }
        }
    }
    
    private function unusedMethod() { // Unused method warning
        return true;
    }
}
```

## Configuration

> **Note:** The extension works without any configuration, using PHPMD's default rulesets.

### Rulesets

<details>
<summary><strong>Automatic Discovery</strong> (recommended)</summary>

The extension follows **PHP Mess Detector's native discovery behavior** with this priority order:

1. **Project config files** (discovered automatically):
   - `.phpmd.xml` (highest priority)
   - `phpmd.xml`
   - `.phpmd.xml.dist`
   - `phpmd.xml.dist`
   - `ruleset.xml` (lowest config file priority)
2. **Zed settings** - Custom configuration in settings.json  
3. **Environment variables** - `PHPMD_RULESET`
4. **Default rulesets** - All built-in PHPMD rulesets enabled

</details>

<details>
<summary><strong>Zed Settings Configuration</strong></summary>

Configure rulesets in your **Zed settings.json** file (open with `Cmd+,` or `Ctrl+,`):

**Single ruleset:**
```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "ruleset": "cleancode"
      }
    }
  }
}
```

**Multiple rulesets (comma-separated):**
```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "ruleset": ["cleancode", "codesize", "controversial", "design", "naming", "unusedcode"]
      }
    }
  }
}
```

**Path to custom ruleset:**
```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "ruleset": "/path/to/custom-phpmd.xml"
      }
    }
  }
}
```

**Relative path to project ruleset:**
```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "ruleset": "./ruleset.xml"
      }
    }
  }
}
```

> **💡 Tip:** You can also set these in **local project settings** by creating `.zed/settings.json` in your project root.

</details>

<details>
<summary><strong>Environment Variables</strong></summary>

```bash
export PHPMD_RULESET="cleancode,codesize"
export PHPMD_PATH="/custom/path/to/phpmd"
```

</details>

### PHPMD Executable

<details>
<summary><strong>Automatic Discovery</strong> (recommended)</summary>

The extension finds PHPMD in this priority order:

1. **Project composer** - `vendor/bin/phpmd` (includes project dependencies)
2. **Bundled PHAR** - Modern PHPMD v2.15+ (included with extension) 
3. **System PATH** - Global phpmd installation

> **💡 Enhanced Compatibility:** The extension prioritizes your project's local PHPMD installation, ensuring full compatibility with Composer-installed plugins and the exact PHPMD version your project requires.

</details>

<details>
<summary><strong>Custom Paths</strong></summary>

Specify custom PHPMD path in settings.json:

```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "phpmdPath": "/custom/path/to/phpmd"
      }
    }
  }
}
```

</details>

## Available Rulesets

| Ruleset | Description |
|---------|-------------|
| **cleancode** | Rules for clean code that follows best practices |
| **codesize** | Rules for code size and complexity metrics |
| **controversial** | Controversial rules that not everyone agrees with |
| **design** | Rules for software design principles |
| **naming** | Rules for naming conventions |
| **unusedcode** | Rules to detect unused code |

## Project Configuration

Create a `phpmd.xml` in your project root for team consistency. The extension will automatically discover and use any of these files (in priority order):

- `.phpmd.xml` (typically for local overrides, often gitignored)
- `phpmd.xml` (main project configuration) 
- `.phpmd.xml.dist` (distributable version, lower priority)
- `phpmd.xml.dist` (template version, lowest priority)
- `ruleset.xml` (alternative name)

```xml
<?xml version="1.0"?>
<ruleset name="Project Mess Detector Rules"
         xmlns="http://pmd.sf.net/ruleset/1.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://pmd.sf.net/ruleset/1.0.0
                             http://pmd.sf.net/ruleset_xml_schema.xsd"
         xsi:noNamespaceSchemaLocation="http://pmd.sf.net/ruleset_xml_schema.xsd">
    
    <description>Custom ruleset for our project</description>

    <!-- Import base rulesets -->
    <rule ref="rulesets/cleancode.xml">
        <!-- Exclude specific rules -->
        <exclude name="ElseExpression"/>
    </rule>
    
    <rule ref="rulesets/codesize.xml">
        <exclude name="ExcessiveMethodLength"/>
    </rule>
    
    <rule ref="rulesets/design.xml"/>
    <rule ref="rulesets/naming.xml">
        <exclude name="ShortVariable"/>
        <exclude name="LongVariable"/>
    </rule>
    
    <!-- Customize specific rules -->
    <rule ref="rulesets/codesize.xml/CyclomaticComplexity">
        <properties>
            <property name="reportLevel" value="20"/>
        </properties>
    </rule>
    
    <rule ref="rulesets/naming.xml/ShortVariable">
        <properties>
            <property name="minimum" value="2"/>
        </properties>
    </rule>

    <!-- Exclude directories -->
    <exclude-pattern>*/vendor/*</exclude-pattern>
    <exclude-pattern>*/tests/*</exclude-pattern>
    <exclude-pattern>*/storage/*</exclude-pattern>
</ruleset>
```

## Auto-Recovery

The extension automatically handles configuration changes and edge cases:

### **Deleted Config Files**
If you delete a `phpmd.xml` file after the LSP is running:
- **Proactive detection** - Checks file exists before each analysis
- Automatically re-scans for other config files (`.phpmd.xml.dist`, etc.)
- Falls back to default rulesets if no config found
- **No restart required** - recovery happens seamlessly

### **Invalid Config Files**
If a config file becomes corrupted or references missing rules:
- File existence validated before use, with immediate re-discovery if missing
- Ruleset discovery process re-runs automatically for any configuration issues
- Graceful fallback to working configuration
- Detailed logging shows the recovery process

### **Dynamic Updates**
- **Settings changes** - Applied immediately via `did_change_configuration`
- **Workspace changes** - Config re-discovered when switching projects
- **File system changes** - Config errors trigger automatic re-discovery

## Understanding PHPMD Messages

PHPMD categorizes issues by priority:

- **Priority 1 (Error)** - Critical issues that likely indicate bugs
- **Priority 2-3 (Warning)** - Important code quality issues
- **Priority 4 (Info)** - Minor issues and suggestions
- **Priority 5 (Hint)** - Style preferences and minor optimizations

## Troubleshooting

<details>
<summary><strong>Extension not working?</strong></summary>

1. Check Zed's debug console for error messages
2. Verify PHPMD is accessible (custom paths must exist)
3. **No restart needed** - configuration changes apply immediately

</details>

<details>
<summary><strong>No diagnostics showing?</strong></summary>

1. Ensure you're editing a `.php` file
2. Check that your configured rulesets exist
3. Test with a file containing obvious code issues (deeply nested code, unused variables)

</details>

<details>
<summary><strong>Custom rules not working?</strong></summary>

1. Validate your `phpmd.xml` syntax
2. Ensure paths are relative to your project root
3. Test your configuration manually with `phpmd src/ text phpmd.xml`

</details>

<details>
<summary><strong>Want to set global defaults?</strong></summary>

**Set PHPMD global configuration (affects all projects without local config):**
```bash
# Set via environment variable
export PHPMD_RULESET="cleancode,codesize,design"

# Create user-specific config file
cat > ~/.phpmd.xml << 'EOF'
<?xml version="1.0"?>
<ruleset name="My Default Rules">
    <rule ref="rulesets/cleancode.xml"/>
    <rule ref="rulesets/codesize.xml"/>
    <rule ref="rulesets/design.xml"/>
</ruleset>
EOF
```

> **💡 Pro Tip:** The extension respects all PHPMD configuration methods, so you can mix global defaults with project-specific overrides.

</details>

## Differences from PHPCS

While PHPCS focuses on **coding standards** (formatting, style), PHPMD focuses on **code quality**:

| PHPCS | PHPMD |
|-------|-------|
| Indentation and spacing | Code complexity |
| Bracket placement | Unused code detection |
| Naming conventions (style) | Naming conventions (clarity) |
| Line length | Method/class size |
| Comment formatting | Design pattern violations |

**Both tools complement each other** - use PHPCS for style consistency and PHPMD for code quality.

## Resources & Credits

### Learn More
- [PHP Mess Detector Documentation](https://phpmd.org)
- [PHPMD Rules Documentation](https://phpmd.org/rules/index.html)
- [Zed Editor Documentation](https://zed.dev/docs)

### Built With
- [PHP Mess Detector](https://phpmd.org) - The powerful tool that analyzes code quality
- [Zed Editor](https://zed.dev) - The fast, collaborative editor
- [Tower LSP](https://github.com/ebkalderon/tower-lsp) - Rust LSP framework

## License

### Main License

This project is licensed under the [MIT License](LICENSE).

```
MIT License

Copyright (c) 2025 Mike Bronner

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### Third-Party Licenses

This extension bundles and redistributes third-party software. For a complete list of third-party licenses and attributions, please see [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).

**Key third-party components:**

- **PHP Mess Detector** - BSD 3-Clause License  
  The core tool that powers code analysis. Bundled as PHAR binary.
  
- **Rust Dependencies** - Various permissive licenses (Apache-2.0, MIT, etc.)  
  All dependencies are compatible with the MIT license. See the full list in the third-party licenses file.

-----
**Made with ❤️ and lots of ☕ for the PHP community.**