#!/bin/bash

echo "🧪 Debugging Devbox Init Command..."

# Clean up first
rm -rf test-debug-*

echo "1. Testing minimal init..."
cargo run -- init test-debug-minimal --yes

echo "2. Checking if directory was created..."
ls -la | grep test-debug

echo "3. If directory exists, show contents..."
if [ -d "test-debug-minimal" ]; then
    echo "✅ Directory exists!"
    ls -la test-debug-minimal/
    cat test-debug-minimal/devbox.yaml
else
    echo "❌ Directory was not created"
    
    # Check for any error output
    echo "4. Running with stderr output..."
    cargo run -- init test-debug-minimal --yes 2>&1
fi

echo "🎉 Debug complete!"