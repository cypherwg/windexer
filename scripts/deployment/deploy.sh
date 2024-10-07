#!/bin/bash
set -e

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <environment>"
    exit 1
fi

ENVIRONMENT=$1

case $ENVIRONMENT in
    "dev")
        DOCKER_REPO="your-dev-repo"
        K8S_NAMESPACE="windexer-dev"
        ;;
    "prod")
        DOCKER_REPO="your-prod-repo"
        K8S_NAMESPACE="windexer-prod"
        ;;
    *)
        echo "Invalid environment. Use 'dev' or 'prod'."
        exit 1
        ;;
esac

echo "Building Docker image..."
docker build -t windexer:latest .

DOCKER_TAG="$DOCKER_REPO/windexer:$(git rev-parse --short HEAD)"
docker tag windexer:latest $DOCKER_TAG

echo "Pushing image to repository..."
docker push $DOCKER_TAG

echo "Updating Kubernetes deployment..."
kubectl set image deployment/windexer windexer=$DOCKER_TAG -n $K8S_NAMESPACE

echo "Waiting for rollout to complete..."
kubectl rollout status deployment/windexer -n $K8S_NAMESPACE

echo "Deployment complete!"