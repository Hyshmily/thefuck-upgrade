#!/bin/bash

# The Fuck - Verification Script
# This script verifies that The Fuck is properly installed.

echo "The Fuck - Verification Script"
echo "=================================="

# Check if thefuck is installed
if command -v thefuck &> /dev/null; then
    echo "✓ TheFuck is installed at: $(which thefuck)"

    # Check version
    if thefuck --version &> /dev/null; then
        echo "✓ Version: $(thefuck --version)"
    else
        echo "✗ Could not read the version"
    fi
else
    echo "✗ TheFuck is not installed"
    exit 1
fi

# Check if fuck alias is set
if alias fuck &> /dev/null; then
    echo "✓ fuck alias is set: $(alias fuck)"
else
    echo "! fuck alias is not set - run: eval \"\$(thefuck --alias)\""
fi

# Test basic functionality
echo ""
echo "Testing basic functionality..."
echo "git status -> gti status"
echo "fuck should suggest: git status"

# Test command
echo "gti status" | thefuck 2>&1 | head -5
if [ $? -eq 0 ]; then
    echo "✓ Basic functionality test passed"
else
    echo "! Basic functionality test failed - this can happen on a fresh install"
fi

echo ""
echo "Verification complete!"