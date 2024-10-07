# Windexer Deployment Guide

## Prerequisites

- Docker and Docker Compose
- Kubernetes cluster (for production deployment)
- Access to a Solana RPC node
- ScyllaDB and ClickHouse instances

## Local Development Deployment

1. Clone the repository:
   ```
   git clone https://github.com/cypherwg/windexer.git
   cd windexer
   ```

2. Create a `.env` file with necessary environment variables:
   ```
   SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
   SCYLLA_HOST=scylla
   CLICKHOUSE_HOST=clickhouse
   ```

3. Start the services using Docker Compose:
   ```
   docker-compose up -d
   ```

4. Verify that all services are running:
   ```
   docker-compose ps
   ```

## Production Deployment

1. Set up a Kubernetes cluster (e.g., using GKE, EKS, or AKS).

2. Install Helm if not already installed:
   ```
   curl https://raw.githubusercontent.com/helm/helm/master/scripts/get-helm-3 | bash
   ```

3. Add the Windexer Helm repository:
   ```
   helm repo add windexer https://charts.cypheros.xyz
   helm repo update
   ```

4. Create a `values.yaml` file with your configuration:
   ```yaml
   solanaRpcUrl: "https://rpc1v01.cypheros.xyz/api"
   scyllaDb:
     host: "scylla-v1.cypheros.xyz"
   clickhouse:
     host: "clickhouse-v1.cypheros.xyz
   ```

5. Install Windexer using Helm:
   ```
   helm install windexer windexer/windexer -f values.yaml
   ```

6. Verify the deployment:
   ```
   kubectl get pods
   ```

## Monitoring and Maintenance

1. Access Grafana for monitoring:
   ```
   kubectl port-forward svc/windexer-grafana 3000:80
   ```
   Then open `http://localhost:3000` in your browser.

2. View logs:
   ```
   kubectl logs -l app=windexer
   ```

3. Upgrade Windexer:
   ```
   helm upgrade windexer windexer/windexer -f values.yaml
   ```

4. Scale the deployment:
   ```
   kubectl scale deployment windexer --replicas=3
   ```

## Troubleshooting

- Check the logs for any error messages.
- Ensure all required environment variables are set correctly.
- Verify connectivity to Solana RPC node, ScyllaDB, and ClickHouse.
- Check Prometheus metrics for any performance issues.

For more detailed troubleshooting, refer to the [Troubleshooting Guide](./troubleshooting.md).
```