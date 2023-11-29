# Build your container image
docker build -t gcr.io/your-project-id/my-app .

# Push the image to Google Container Registry
docker push gcr.io/your-project-id/my-app

gcloud run deploy my-app-service \
    --image gcr.io/your-project-id/my-app \
    --platform managed \
    --region your-region \
    --allow-unauthenticated
