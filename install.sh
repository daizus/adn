cat > install.sh << 'EOF'
#!/bin/sh

set -e

echo "Installing adn..."
install -Dm755 ./adn /usr/local/bin/adn

echo "✅ Installed to /usr/local/bin/adn"
EOF

