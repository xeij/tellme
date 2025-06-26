#!/bin/bash
# test_setup.sh - Project setup verification script

echo "tellme - Project Setup Test"
echo "=========================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ $2${NC}"
    else
        echo -e "${RED}✗ $2${NC}"
    fi
}

# Test 1: Check if Rust is installed
echo -e "${BLUE}1. Checking Rust installation...${NC}"
if command -v rustc >/dev/null 2>&1; then
    RUST_VERSION=$(rustc --version)
    print_status 0 "Rust found: $RUST_VERSION"
else
    print_status 1 "Rust not found. Please install from https://rustup.rs/"
    echo ""
    echo -e "${YELLOW}Quick install:${NC}"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Test 2: Check if Cargo is available
echo -e "${BLUE}2. Checking Cargo...${NC}"
if command -v cargo >/dev/null 2>&1; then
    CARGO_VERSION=$(cargo --version)
    print_status 0 "Cargo found: $CARGO_VERSION"
else
    print_status 1 "Cargo not found"
    exit 1
fi

# Test 3: Check project structure
echo -e "${BLUE}3. Checking project structure...${NC}"
required_files=(
    "Cargo.toml"
    "src/main.rs"
    "src/lib.rs"
    "src/content.rs"
    "src/database.rs"
    "src/ui.rs"
    "src/bin/fetch_data.rs"
    "README.md"
)

all_files_exist=true
for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        print_status 0 "$file exists"
    else
        print_status 1 "$file missing"
        all_files_exist=false
    fi
done

if [ "$all_files_exist" = false ]; then
    echo ""
    echo -e "${RED}Some required files are missing. Please check the project setup.${NC}"
    exit 1
fi

# Test 4: Try to compile the project
echo -e "${BLUE}4. Testing compilation...${NC}"
if cargo check >/dev/null 2>&1; then
    print_status 0 "Project compiles successfully"
else
    print_status 1 "Compilation failed"
    echo ""
    echo -e "${YELLOW}Running 'cargo check' for details:${NC}"
    cargo check
    exit 1
fi

# Test 5: Check if build works
echo -e "${BLUE}5. Testing build...${NC}"
if cargo build >/dev/null 2>&1; then
    print_status 0 "Project builds successfully"
else
    print_status 1 "Build failed"
    echo ""
    echo -e "${YELLOW}Running 'cargo build' for details:${NC}"
    cargo build
    exit 1
fi

echo ""
echo -e "${GREEN}🎉 All tests passed! Your tellme project is ready.${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Fetch Wikipedia content: cargo run --bin fetch_data"
echo "2. Run the app: cargo run --bin tellme"
echo ""
echo -e "${BLUE}For global installation:${NC}"
echo "cargo install --path ." 