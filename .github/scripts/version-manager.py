#!/usr/bin/env python3

import os
import sys
import re
import subprocess
import json
from datetime import datetime

# Constants
COMMIT_PATTERN = r"^(BREAKING CHANGE|build|ci|feat|fix|docs|style|refactor|perf|test|revert|chore)(\(.*\))?:"

def run_command(cmd, capture_output=True):
    """Execute a command and return the result"""
    result = subprocess.run(cmd, shell=True, text=True, capture_output=capture_output)
    if result.returncode != 0:
        print(f"Error executing command: {cmd}")
        print(f"stderr: {result.stderr}")
        sys.exit(result.returncode)
    return result.stdout.strip() if capture_output else None

def get_latest_tag():
    """Get the latest tag, return v0.0.0 if none exists"""
    try:
        return run_command("git describe --abbrev=0 --tags")
    except:
        return "v0.0.0"

def get_eligible_commits(latest_tag):
    """Get eligible commits that match the commit pattern"""
    cmd = f'git log {latest_tag}..HEAD --pretty=format:"%h%x09%H%x09%s" --no-merges -P --grep="{COMMIT_PATTERN}"'
    result = run_command(cmd)
    return result.split('\n') if result else []

def determine_semver_level(commits):
    """Determine semantic versioning level from commits"""
    level = "PATCH"  # Default is PATCH
    
    for commit in commits:
        parts = commit.split('\t')
        if len(parts) < 3:
            continue
            
        message = parts[2]
        
        # BREAKING CHANGE is MAJOR
        if re.search(r"^BREAKING[ _]CHANGE", message):
            return "MAJOR"
            
        # perf is MAJOR
        if re.search(r"^perf(\(.*\))?:", message):
            return "MAJOR"
            
        # feat, revert are MINOR
        if re.search(r"^(feat|revert)(\(.*\))?:", message):
            level = "MINOR"
    
    return level

def calculate_next_version(current_version, level):
    """Calculate next version based on current version and semantic level"""
    # Remove 'v' prefix from version
    version = current_version.lstrip('v')
    major, minor, patch = map(int, version.split('.'))
    
    if level == "MAJOR":
        major += 1
        minor = 0
        patch = 0
    elif level == "MINOR":
        minor += 1
        patch = 0
    elif level == "PATCH":
        patch += 1
    
    return f"{major}.{minor}.{patch}"

def create_snapshot_version(version):
    """Create snapshot version with date and time suffix"""
    now = datetime.now()
    date_str = now.strftime("%Y%m%d")
    time_str = now.strftime("%H%M%S")
    return f"{version}-SNAPSHOT-{date_str}-{time_str}"

def create_release_note_header(server_url, repository, prev_tag, next_tag):
    """Create header section for release notes"""
    return f"""## Changes from {prev_tag} to {next_tag}

[Full Changelog]({server_url}/{repository}/compare/{prev_tag}...{next_tag})

"""

def create_release_note_body(commits):
    """Create body content for release notes organized by commit types"""
    if not commits:
        return "No changes"
        
    categories = {
        "feat": "### 🚀 New Features",
        "fix": "### 🐛 Bug Fixes",
        "perf": "### ⚡ Performance Improvements",
        "docs": "### 📝 Documentation",
        "style": "### 💄 Styles",
        "refactor": "### ♻️ Refactoring",
        "test": "### 🧪 Tests",
        "build": "### 🔨 Build",
        "ci": "### 👷 CI",
        "chore": "### 🔧 Chores",
        "revert": "### ⏪ Reverts",
        "BREAKING CHANGE": "### 💥 Breaking Changes"
    }
    
    # Organize commits by category
    categorized_commits = {}
    for commit in commits:
        parts = commit.split('\t')
        if len(parts) < 3:
            continue
            
        short_hash, full_hash, message = parts
        
        # Extract commit type and scope
        match = re.search(COMMIT_PATTERN, message)
        if not match:
            continue
            
        commit_type = match.group(1)
        commit_scope = match.group(2) if match.group(2) else ""
        
        # Extract scope without parentheses if present
        scope_text = ""
        if commit_scope:
            scope_text = commit_scope[1:-1]  # Remove ( and )
        
        # Remove commit type prefix from message
        message_body = re.sub(COMMIT_PATTERN + r"\s*", "", message)
        
        if commit_type not in categorized_commits:
            categorized_commits[commit_type] = []
            
        categorized_commits[commit_type].append({
            "short_hash": short_hash,
            "full_hash": full_hash,
            "message": message_body,
            "scope_text": scope_text
        })
    
    # Generate output by category
    output = []
    for commit_type, title in categories.items():
        if commit_type in categorized_commits and categorized_commits[commit_type]:
            output.append(title)
            
            for commit in categorized_commits[commit_type]:
                # Store scope and hash information
                scope_text = commit.get('scope_text', '')
                scope_display = f"({scope_text}) " if scope_text else ""
                
                # Add the commit with scope if present
                output.append(f"- {scope_display}{commit['message']} ([{commit['short_hash']}]({os.environ.get('GITHUB_SERVER_URL', '')}/{os.environ.get('GITHUB_REPOSITORY', '')}/commit/{commit['full_hash']}))")
            
            output.append("")
    
    return "\n".join(output)

def set_github_output(name, value):
    """Set output values for GitHub Actions"""
    if os.environ.get('GITHUB_OUTPUT'):
        with open(os.environ.get('GITHUB_OUTPUT'), 'a') as f:
            f.write(f"{name}={value}\n")
    else:
        # Fallback for local execution (only print)
        print(f"Output: {name}={value}")

def prepare_release():
    """Prepare a release by analyzing commits and creating version files"""
    latest_tag = get_latest_tag()
    print(f"Latest tag: {latest_tag}")
    set_github_output("prev_tag", latest_tag)
    
    commits = get_eligible_commits(latest_tag)
    commit_count = len(commits)
    print(f"Eligible commit count: {commit_count}")
    set_github_output("count", str(commit_count))
    
    if commit_count == 0:
        set_github_output("has_changes", "false")
        return
    
    semver_level = determine_semver_level(commits)
    print(f"Semantic versioning level: {semver_level}")
    set_github_output("semver_level", semver_level)
    
    next_version = calculate_next_version(latest_tag, semver_level)
    next_tag = f"v{next_version}"
    print(f"Next version: {next_version}")
    print(f"Next tag: {next_tag}")
    set_github_output("next_version", next_version)
    set_github_output("next_tag", next_tag)
    set_github_output("has_changes", "true")
    
    # Create version file
    version_file = os.environ.get('VERSION_FILE', 'version')
    with open(version_file, "w") as f:
        f.write(next_version)
    
    # Generate release notes
    server_url = os.environ.get('GITHUB_SERVER_URL', 'https://github.com')
    repository = os.environ.get('GITHUB_REPOSITORY', '')
    
    header = create_release_note_header(server_url, repository, latest_tag, next_tag)
    body = create_release_note_body(commits)
    
    changelog_file = os.environ.get('CHANGELOG_FILE', 'changelog.txt')
    with open(changelog_file, "w") as f:
        f.write(header + body)

def create_snapshot():
    """Create snapshot version from current version or latest tag"""
    version_file = os.environ.get('VERSION_FILE', 'version')
    
    try:
        # Try to read from version file if it exists
        with open(version_file, "r") as f:
            current_version = f.read().strip()
    except FileNotFoundError:
        # If file doesn't exist, get version from Git tag
        try:
            latest_tag = get_latest_tag()
            current_version = latest_tag.lstrip('v')
        except:
            # Use default value if all else fails
            current_version = "0.1.0"
            print(f"Warning: Could not determine current version, using default: {current_version}")
    
    snapshot_version = create_snapshot_version(current_version)
    print(f"Snapshot version: {snapshot_version}")
    set_github_output("snapshot_version", snapshot_version)
    
    # Update version file (only meaningful in GitHub Actions)
    with open(version_file, "w") as f:
        f.write(snapshot_version)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python version-manager.py <prepare-release|create-snapshot>")
        sys.exit(1)
    
    mode = sys.argv[1]
    if mode == "prepare-release":
        prepare_release()
    elif mode == "create-snapshot":
        create_snapshot()
    else:
        print(f"Unknown mode: {mode}")
        sys.exit(1)
