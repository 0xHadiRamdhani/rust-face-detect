#!/bin/bash

# Face Detection Rust - Setup Script
# This script helps set up the development environment

set -e

echo "🎯 Setting up Face Detection Rust project..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
check_rust() {
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    local rust_version=$(rustc --version | cut -d' ' -f2)
    print_success "Rust version: $rust_version"
}

# Check if Cargo is installed
check_cargo() {
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    print_success "Cargo is available"
}

# Create necessary directories
create_directories() {
    print_status "Creating necessary directories..."
    
    mkdir -p uploads
    mkdir -p static/assets
    mkdir -p tests/fixtures
    
    print_success "Directories created"
}

# Copy environment file
setup_environment() {
    print_status "Setting up environment configuration..."
    
    if [ ! -f .env ]; then
        cp .env.example .env
        print_success "Environment file created from .env.example"
    else
        print_warning "Environment file already exists"
    fi
}

# Install dependencies
install_dependencies() {
    print_status "Installing Rust dependencies..."
    
    cargo fetch
    print_success "Dependencies installed"
}

# Build the project
build_project() {
    print_status "Building the project..."
    
    cargo build --release
    print_success "Project built successfully"
}

# Run tests
run_tests() {
    print_status "Running tests..."
    
    cargo test
    print_success "Tests completed"
}

# Create sample test images
create_test_images() {
    print_status "Creating sample test images..."
    
    # Create a simple test image using ImageMagick if available
    if command -v convert &> /dev/null; then
        convert -size 400x400 xc:white -fill black -draw "rectangle 100,100 200,200" tests/fixtures/test1.jpg
        convert -size 600x600 xc:white -fill black -draw "rectangle 100,100 200,200" -draw "rectangle 400,400 500,500" tests/fixtures/test2.jpg
        print_success "Sample test images created"
    else
        print_warning "ImageMagick not found. Skipping sample image creation."
        print_status "You can manually add test images to tests/fixtures/"
    fi
}

# Setup git hooks (optional)
setup_git_hooks() {
    print_status "Setting up git hooks..."
    
    # Create pre-commit hook for formatting
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Format code before commit
cargo fmt --check
if [ $? -ne 0 ]; then
    echo "Code is not formatted. Running cargo fmt..."
    cargo fmt
    echo "Code formatted. Please commit again."
    exit 1
fi

# Run clippy
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "Clippy found issues. Please fix them before committing."
    exit 1
fi
EOF
    
    chmod +x .git/hooks/pre-commit
    print_success "Git hooks configured"
}

# Main setup function
main() {
    echo "=========================================="
    echo "  Face Detection Rust - Setup Script"
    echo "=========================================="
    echo
    
    # Check prerequisites
    check_rust
    check_cargo
    
    # Setup project
    create_directories
    setup_environment
    install_dependencies
    build_project
    
    # Optional steps
    read -p "Run tests? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        run_tests
    fi
    
    read -p "Create sample test images? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        create_test_images
    fi
    
    read -p "Setup git hooks? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        setup_git_hooks
    fi
    
    echo
    echo "=========================================="
    print_success "Setup completed successfully!"
    echo "=========================================="
    echo
    echo "To start the server, run:"
    echo "  cargo run"
    echo
    echo "Then open your browser to:"
    echo "  http://localhost:8080"
    echo
    echo "For development, you can also use:"
    echo "  cargo watch -x run"
    echo
}

# Run main function
main "$@"