#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Testing Devbox Init Command...${NC}"

# Test directory
TEST_DIR="test-init-output"
rm -rf "$TEST_DIR" 2>/dev/null

# Function to run test and check result
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_dir="$3"
    
    echo -e "\n${YELLOW}Testing: $test_name${NC}"
    echo "Command: $command"
    
    # Run the command
    eval $command
    
    # Check if project was created
    if [ -d "$expected_dir" ] && [ -f "$expected_dir/devbox.yaml" ]; then
        echo -e "${GREEN}PASS: $test_name${NC}"
        echo "   Project created: $expected_dir"
        echo "   Files:"
        find "$expected_dir" -type f | head -10
    else
        echo -e "${RED}FAIL: $test_name${NC}"
        echo "   Expected directory: $expected_dir"
        ls -la 2>/dev/null | grep "$expected_dir" || echo "   Directory not found"
    fi
}

# Test 1: Basic init with yes flag
run_test "Basic init with --yes" \
    "cargo run -- init test-init-output-basic --yes" \
    "test-init-output-basic"

# Test 2: Init with Docker
run_test "Init with Docker" \
    "cargo run -- init test-init-output-docker --yes --docker" \
    "test-init-output-docker"

# Test 3: Different templates
run_test "Web template" \
    "cargo run -- init test-init-output-web --yes --template web" \
    "test-init-output-web"

run_test "API template" \
    "cargo run -- init test-init-output-api --yes --template api" \
    "test-init-output-api"

run_test "Fullstack template" \
    "cargo run -- init test-init-output-fullstack --yes --template fullstack" \
    "test-init-output-fullstack"

# Test 4: Verify file contents
echo -e "\n${YELLOW}Verifying file contents...${NC}"

check_file() {
    local file="$1"
    if [ -f "$file" ]; then
        echo -e "${GREEN}$file exists${NC}"
        # Show first few lines
        head -5 "$file" 2>/dev/null | sed 's/^/   /'
    else
        echo -e "${RED}$file missing${NC}"
    fi
}

if [ -d "test-init-output-basic" ]; then
    check_file "test-init-output-basic/devbox.yaml"
    check_file "test-init-output-basic/frontend/package.json"
    check_file "test-init-output-basic/api/package.json"
fi

if [ -d "test-init-output-docker" ]; then
    check_file "test-init-output-docker/docker-compose.yml"
    check_file "test-init-output-docker/.dockerignore"
fi

if [ -d "test-init-output-fullstack" ]; then
    check_file "test-init-output-fullstack/database/init.sql"
fi

# Summary
echo -e "\n${YELLOW}Test Summary:${NC}"
echo "Projects created:"
find . -maxdepth 1 -type d -name "test-init-output-*" 2>/dev/null

# Cleanup option
echo -e "\n${YELLOW}Cleanup options:${NC}"
echo "1. Keep test projects"
echo "2. Remove test projects"
read -p "Choose [1-2]: " cleanup_choice

if [ "$cleanup_choice" = "2" ]; then
    echo "Cleaning up test projects..."
    rm -rf test-init-output-*
    echo -e "${GREEN}Cleanup complete${NC}"
else
    echo -e "${YELLOW}Test projects kept in: test-init-output-*${NC}"
fi

echo -e "\n${GREEN}Init command testing complete!${NC}"