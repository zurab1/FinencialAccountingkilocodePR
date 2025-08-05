#!/bin/bash

# Financial Accounting System Startup Script

echo "🚀 Starting Financial Accounting System..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if .env file exists, create if not
if [ ! -f .env ]; then
    echo "📝 Creating .env file..."
    echo "DATABASE_URL=sqlite:accounting.db" > .env
fi

# Build and run the application
echo "🔨 Building application..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🌐 Starting web server..."
    echo "📊 Financial Accounting System will be available at:"
    echo "   http://127.0.0.1:3000"
    echo ""
    echo "Press Ctrl+C to stop the server"
    echo ""
    
    cargo run --release
else
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi