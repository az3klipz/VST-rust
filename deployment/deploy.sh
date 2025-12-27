#!/bin/bash
set -e

echo "Antigravity Cloud Deployment"
echo "============================"

if [ -z "$GOOGLE_CLOUD_PROJECT" ]; then
    echo "Error: GOOGLE_CLOUD_PROJECT env var is not set."
    echo "Run 'gcloud config set project YOUR_PROJECT_ID' first."
    exit 1
fi

echo "Deploying to Project: $GOOGLE_CLOUD_PROJECT"

# Initialize Terraform
cd deployment
terraform init

# Plan
echo "Planning deployment..."
terraform plan -out=tfplan

# Apply
read -p "Do you want to apply this plan? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    terraform apply "tfplan"
    echo "Deployment Complete!"
else
    echo "Deployment cancelled."
fi
