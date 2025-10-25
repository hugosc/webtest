#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔄 Starting auto-rebuild watcher...${NC}"
echo -e "${BLUE}Watching for changes in src/lib.rs${NC}"
echo ""
echo -e "${BLUE}Press Ctrl+C to stop${NC}"
echo ""

cargo watch \
  -x "build --target wasm32-unknown-unknown --release" \
  -s "rm -rf public/pkg && cp -r pkg public/pkg && echo -e '${GREEN}✅ Built and copied! Refresh your browser${NC}'" \
  -c
