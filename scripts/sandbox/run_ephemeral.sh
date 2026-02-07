#!/bin/bash
# Ephemeral Sandbox Runner
# Creates a temporary environment, executes a command, and ensures total cleanup.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMP_ROOT=$(mktemp -d)
SANDBOX_DIR="$TEMP_ROOT/ephemeral_sandbox"

# Ensure cleanup on script exit (even if interrupted)
cleanup() {
    echo -e "
🧹 Tearing down ephemeral environment: $TEMP_ROOT"
    rm -rf "$TEMP_ROOT"
}
trap cleanup EXIT SIGINT SIGTERM SIGHUP

echo "🌊 Initializing ephemeral environment at $SANDBOX_DIR..."

# Populate the sandbox using existing logic
"$SCRIPT_DIR/mksandbox.sh" --output "$SANDBOX_DIR" --count 5 > /dev/null

echo "🚀 Executing command in sandbox context..."
echo "----------------------------------------"

# Run the user command
# We use eval to allow for piped commands and complex arguments
cd "$SANDBOX_DIR" && eval "$@"

echo "----------------------------------------"
echo "✅ Command execution finished."
