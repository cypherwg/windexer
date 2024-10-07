#!/bin/bash
set -e

install_grafana() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$NAME
    else
        echo "Cannot detect OS. Please install Grafana manually."
        exit 1
    fi

    case $OS in
        "Ubuntu"|"Debian GNU/Linux")
            echo "Detected Debian-based system. Installing Grafana..."
            sudo apt-get update
            sudo apt-get install -y software-properties-common
            sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
            wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
            sudo apt-get update
            sudo apt-get install -y grafana
            ;;
        "Fedora")
            echo "Detected Fedora. Installing Grafana..."
            sudo dnf install -y https://dl.grafana.com/oss/release/grafana-9.5.2-1.x86_64.rpm
            ;;
        "Arch Linux")
            echo "Detected Arch Linux. Installing Grafana..."
            sudo pacman -Sy
            sudo pacman -S grafana
            ;;
        *)
            echo "Unsupported OS: $OS. Please install Grafana manually."
            exit 1
            ;;
    esac
}

if ! command -v grafana-cli &> /dev/null
then
    echo "Grafana is not installed. Installing Grafana..."
    install_grafana
fi

sudo systemctl start grafana-server
sudo systemctl enable grafana-server

echo "Waiting for Grafana to start..."
until $(curl --output /dev/null --silent --head --fail http://localhost:3000); do
    printf '.'
    sleep 5
done

echo "Setting up Prometheus datasource..."
curl -X POST -H "Content-Type: application/json" -d '{
    "name":"Prometheus",
    "type":"prometheus",
    "url":"http://localhost:9090",
    "access":"proxy",
    "basicAuth":false
}' http://admin:admin@localhost:3000/api/datasources

echo "Importing dashboards..."
for dashboard in ./dashboards/*.json
do
    curl -X POST -H "Content-Type: application/json" -d "{
        \"dashboard\": $(cat $dashboard),
        \"overwrite\": true
    }" http://admin:admin@localhost:3000/api/dashboards/db
done

echo "Grafana setup complete!"